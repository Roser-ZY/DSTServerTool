# DSTServerTool Development Guidelines

Auto-generated for agentic coding. Last updated: 2026-03-17

## Project Overview

Desktop application for managing Don't Starve Together (DST) game servers. Built with Vue 3, TypeScript, Tailwind CSS, and Tauri (Rust backend).

## Active Technologies

- **Frontend**: Vue 3 + TypeScript + Vite
- **UI Framework**: Reka UI + Tailwind CSS
- **Desktop Runtime**: Tauri 2.0 (Rust)
- **Build Tools**: npm, Cargo

## Project Structure

```
src/                      # Vue.js frontend source
  components/ui/          # Reka UI component library
  lib/                    # Utilities and helpers
  router/                 # Vue Router configuration
  views/                  # Page components
src-tauri/                # Rust backend (Tauri)
  src/main.rs             # Entry point
  Cargo.toml              # Rust dependencies
  tauri.conf.json         # Tauri configuration
dist/                     # Built frontend assets
```

## Commands

### Frontend (Vue/TypeScript)

```bash
# Development
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Type checking only
vue-tsc --noEmit
```

### Desktop (Tauri)

```bash
# Run Tauri dev server
npm run tauri dev

# Build Tauri app
npm run tauri build

# Direct Cargo commands (in src-tauri/)
cd src-tauri && cargo build
cd src-tauri && cargo run
```

### Running Tests

Currently no test framework configured. When tests are added:
- Use Vitest for Vue component testing
- Run single test: `vitest run <test-file>`

## Code Style

### TypeScript

- **Strict mode enabled** in `tsconfig.json`
- Use explicit types for function parameters and return values
- Use `interface` for object shapes, `type` for unions/aliases
- Prefer `import type` for type-only imports
- Enable `noUnusedLocals` and `noUnusedParameters`

### Vue Components

- Use `<script setup lang="ts">` syntax
- Use Reka UI components from `@/components/ui/`
- Use `cn()` utility for class merging (from `@/lib/utils`)
- Define props with `withDefaults(defineProps<Props>(), {...})`

### Imports

- Use path alias `@/` for project imports (configured in tsconfig)
- Order: external libs → internal modules → relative imports
- Example:
  ```typescript
  import { createApp } from "vue"
  import type { HTMLAttributes } from "vue"
  import { cn } from "@/lib/utils"
  import { buttonVariants } from "."
  ```

### Naming Conventions

- **Components**: PascalCase (e.g., `Button.vue`, `Sidebar.vue`)
- **Variables/functions**: camelCase
- **Types/interfaces**: PascalCase with descriptive names
- **Files**: kebab-case for utilities, PascalCase for components

### Tailwind CSS

- Use utility classes from Tailwind CSS
- Use `cn()` helper to merge with component-specific classes
- Follow Tailwind's default color system

### Error Handling

- Use descriptive error messages in Rust `expect()` calls
- Let TypeScript strict mode catch type errors at compile time
- Handle Tauri IPC errors gracefully in frontend

### Rust (src-tauri)

- Follow standard Rust conventions
- Use `thiserror` for error types when needed
- Keep `main.rs` minimal; prefer module organization

## TypeScript Config Reference

```json
{
  "compilerOptions": {
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  }
}
```

## Tauri Configuration

- App identifier: `com.dst.server.tool`
- Default window: 960x640, resizable
- Uses `createWebHashHistory` for Vue Router

## Recent Changes

- Migrated to Vue 3 + TypeScript + Tauri 2.0

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
