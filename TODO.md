# TODO

## Done
- [x] Language panels (python, node, rust, go, c++, zig, java, c#, ruby, lua)
- [x] Fullscreen mode for individual panels
- [x] Responsive grid columns based on terminal width
- [x] Tool detection and prompt scaffolding
- [x] System and devtools panels (docker, file listing, logs, ripgrep, etc.)
- [x] CLI args (`--panel`, `--offline`, `--list`, `--no-color`)
- [x] GitHub Actions CI (clippy, fmt, test, conventions via cocogitto)
- [x] Sample capture script for real tool output (`scripts/generate-samples`)
- [x] Fixed-width fullscreen status bar (no shifting on panel cycle)
- [x] lefthook + cocogitto + Taskfile dev tooling
- [x] Publish to crates.io
- [x] Homebrew tap serving prebuilt binaries
- [x] GitHub Releases with prebuilt binaries (macOS + Linux, arm64/x86_64)
- [x] Synchronized-output rendering (no resize flicker)
- [x] NO_COLOR support (`--no-color` and the `NO_COLOR` env var)
- [x] Wire clean captured samples into panels (C/C++, Ruby, Lua)

## Short Term
- [ ] Word wrap in fullscreen (per-panel: wrap prose panels, truncate tables/art)
- [ ] Flatten animated captures (pytest, vitest, cargo, etc.) so they can be wired in
- [ ] Detect and use real CLI tools (eza, bat, jq) when available

## Medium Term
- [ ] Live embedded tool panes via PTY (tui-term) with a static fallback
- [ ] User-scriptable panes: run a command per pane, custom panels via config
- [ ] Cycle between detected tool variants in fullscreen (e.g. eza vs ls)
- [ ] Detect package managers via env vars and cache presence
- [ ] Persist user preferences in ~/.config/diorama.toml

## Ideas
- [ ] User-defined panel templates with ANSI escape codes
