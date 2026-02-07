# Quickstart: DST-1 模组更新检测与替换

## Prerequisites

- Windows 环境
- Steam 已安装并包含 `steamapps` 结构

## Run via CLI

```powershell
python mods_update.py --steam-path "C:\\Program Files (x86)\\Steam"
```

**Notes**: The Steam path must exist and include `steamapps/common` and `steamapps/workshop` with the DST and DST Dedicated Server directories.

## Use as a Reusable Interface

```python
from mods_update.core import run_update

results = run_update(steam_path="C:\\Program Files (x86)\\Steam")
for item in results:
    print(item["workshop_id"], item["status"], item["reason"])
```
