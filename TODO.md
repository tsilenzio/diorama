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
- [x] Wire captured samples into real panels (8 languages; animated captures
      flattened through tmux so pytest/vitest/etc. render as final frames)

## Short Term
- [ ] Word wrap in fullscreen (per-panel: wrap prose panels, truncate tables/art)
- [ ] Reproducible sample flattening: run the tmux flatten inside generate-samples
- [ ] Wire C# and Go captures (C# dotnet output is very long; Go has no color)
- [ ] Sanitize capture paths (temp dirs, usernames) or decide to keep them real
- [ ] Decide whether real panels replace the hand-built ones
- [ ] Detect and use real CLI tools (eza, bat, jq) when available

## Medium Term
- [ ] Live embedded tool panes via PTY (tui-term) with a static fallback
- [ ] User-scriptable panes: run a command per pane, custom panels via config
- [ ] Cycle between detected tool variants in fullscreen (e.g. eza vs ls)
- [ ] Detect package managers via env vars and cache presence
- [ ] Persist user preferences in ~/.config/diorama.toml

## Ideas
- [ ] User-defined panel templates with ANSI escape codes
