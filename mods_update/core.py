"""Core logic for DST mod updates."""

from __future__ import annotations

from dataclasses import asdict, dataclass
from pathlib import Path
import os
import re
import shutil
import uuid
from typing import Dict, Iterable, List, Optional

SERVER_DIR_NAME = "Dont Starve Together Dedicated Server"
GAME_DIR_NAME = "Dont Starve Together"
WORKSHOP_PREFIX = "workshop-"


@dataclass
class UpdateResult:
    workshop_id: str
    status: str  # updated | unchanged | missing | failed
    reason: str
    source_path: Optional[str] = None
    server_mod_path: Optional[str] = None


@dataclass
class ServerMod:
    workshop_id: str
    path: Path
    last_modified_at: float


@dataclass
class SourceMod:
    workshop_id: str
    path: Path
    source_type: str  # workshop | game_mods
    last_modified_at: float


class ModUpdateError(RuntimeError):
    pass


def validate_steam_root(steam_path: str) -> Path:
    root = Path(steam_path).expanduser().resolve()
    if not root.exists():
        raise ModUpdateError(f"Steam path does not exist: {root}")
    steamapps = root / "steamapps"
    if not steamapps.exists():
        raise ModUpdateError(f"steamapps not found under: {root}")
    return root


def resolve_game_paths(steam_root: Path) -> Dict[str, Path]:
    common_dir = steam_root / "steamapps" / "common"
    workshop_dir = steam_root / "steamapps" / "workshop"

    if not common_dir.exists():
        raise ModUpdateError(f"steamapps/common not found under: {steam_root}")
    if not workshop_dir.exists():
        raise ModUpdateError(f"steamapps/workshop not found under: {steam_root}")

    server_dir = common_dir / SERVER_DIR_NAME
    game_dir = common_dir / GAME_DIR_NAME
    if not server_dir.exists():
        raise ModUpdateError(f"{SERVER_DIR_NAME} not found under: {common_dir}")
    if not game_dir.exists():
        raise ModUpdateError(f"{GAME_DIR_NAME} not found under: {common_dir}")

    server_mods_dir = server_dir / "mods"
    game_mods_dir = game_dir / "mods"
    if not server_mods_dir.exists():
        raise ModUpdateError(f"Server mods directory not found: {server_mods_dir}")
    if not game_mods_dir.exists():
        raise ModUpdateError(f"Game mods directory not found: {game_mods_dir}")

    return {
        "common_dir": common_dir,
        "workshop_dir": workshop_dir,
        "server_mods_dir": server_mods_dir,
        "game_mods_dir": game_mods_dir,
    }


def list_server_mods(server_mods_dir: Path) -> Dict[str, ServerMod]:
    result: Dict[str, ServerMod] = {}
    pattern = re.compile(rf"^{re.escape(WORKSHOP_PREFIX)}(\d+)$")
    for entry in server_mods_dir.iterdir():
        if not entry.is_dir():
            continue
        match = pattern.match(entry.name)
        if not match:
            continue
        workshop_id = match.group(1)
        result[workshop_id] = ServerMod(
            workshop_id=workshop_id,
            path=entry,
            last_modified_at=entry.stat().st_mtime,
        )
    return result


def collect_source_mods(root: Path, workshop_ids: Iterable[str], source_type: str) -> Dict[str, List[SourceMod]]:
    ids_set = set(workshop_ids)
    collected: Dict[str, List[SourceMod]] = {workshop_id: [] for workshop_id in ids_set}

    for dirpath, dirnames, _ in os.walk(root):
        current = Path(dirpath)
        name = current.name
        if name in ids_set:
            try:
                mtime = current.stat().st_mtime
            except OSError:
                mtime = 0.0
            collected[name].append(
                SourceMod(
                    workshop_id=name,
                    path=current,
                    source_type=source_type,
                    last_modified_at=mtime,
                )
            )
            # Avoid descending into the matched directory
            dirnames[:] = []
            continue
    return collected


def select_best_source(sources: List[SourceMod]) -> Optional[SourceMod]:
    if not sources:
        return None
    return max(sources, key=lambda s: s.last_modified_at)


def safe_replace_dir(source: Path, destination: Path) -> None:
    if not source.exists():
        raise ModUpdateError(f"Source mod path missing: {source}")

    destination_parent = destination.parent
    tmp_dir = destination_parent / f".tmp-{destination.name}-{uuid.uuid4().hex}"
    backup_dir = destination_parent / f".bak-{destination.name}-{uuid.uuid4().hex}"

    try:
        shutil.copytree(source, tmp_dir)
        if destination.exists():
            destination.rename(backup_dir)
        tmp_dir.rename(destination)
        if backup_dir.exists():
            shutil.rmtree(backup_dir, ignore_errors=True)
    except Exception as exc:
        # Rollback if possible
        try:
            if destination.exists() and backup_dir.exists():
                shutil.rmtree(destination, ignore_errors=True)
                backup_dir.rename(destination)
            elif not destination.exists() and backup_dir.exists():
                backup_dir.rename(destination)
        finally:
            if tmp_dir.exists():
                shutil.rmtree(tmp_dir, ignore_errors=True)
        raise ModUpdateError(f"Failed to replace {destination}: {exc}")


def run_update(steam_path: str) -> List[dict]:
    steam_root = validate_steam_root(steam_path)
    paths = resolve_game_paths(steam_root)

    server_mods = list_server_mods(paths["server_mods_dir"])
    results: List[UpdateResult] = []
    if not server_mods:
        return []

    workshop_ids = list(server_mods.keys())
    workshop_sources = collect_source_mods(paths["workshop_dir"], workshop_ids, "workshop")
    game_sources = collect_source_mods(paths["game_mods_dir"], workshop_ids, "game_mods")

    for workshop_id in sorted(workshop_ids, key=lambda s: int(s)):
        server_mod = server_mods[workshop_id]
        sources = workshop_sources.get(workshop_id, []) + game_sources.get(workshop_id, [])
        best_source = select_best_source(sources)

        if best_source is None:
            results.append(
                UpdateResult(
                    workshop_id=workshop_id,
                    status="missing",
                    reason="No matching source mod found",
                    source_path=None,
                    server_mod_path=str(server_mod.path),
                )
            )
            continue

        if best_source.last_modified_at <= server_mod.last_modified_at:
            results.append(
                UpdateResult(
                    workshop_id=workshop_id,
                    status="unchanged",
                    reason="Source is not newer than server mod",
                    source_path=str(best_source.path),
                    server_mod_path=str(server_mod.path),
                )
            )
            continue

        try:
            safe_replace_dir(best_source.path, server_mod.path)
            results.append(
                UpdateResult(
                    workshop_id=workshop_id,
                    status="updated",
                    reason="Source newer; replaced server mod",
                    source_path=str(best_source.path),
                    server_mod_path=str(server_mod.path),
                )
            )
        except Exception as exc:
            results.append(
                UpdateResult(
                    workshop_id=workshop_id,
                    status="failed",
                    reason=str(exc),
                    source_path=str(best_source.path),
                    server_mod_path=str(server_mod.path),
                )
            )

    return [asdict(item) for item in results]
