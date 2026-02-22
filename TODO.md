# TODO

## Short Term
- [ ] Split panels into separate module files
- [ ] Word wrap content in fullscreen mode
- [ ] Detect and use real CLI tools (eza, bat, jq) when available
- [ ] Add Lua panel
- [ ] Add clap for CLI argument parsing
- [ ] Add `--offline` flag to skip tool detection
- [ ] Add `--panel <name>` to jump to a specific panel

## Medium Term
- [ ] Cycle between detected tool variants in fullscreen (e.g., eza vs ls)
- [ ] Capture real tool output at wide terminal width for fullscreen
- [ ] Detect package managers via env vars and cache presence
- [ ] Persist user preferences in ~/.config/term-preview.toml

## Ideas
- [ ] User-defined panel templates with ANSI escape codes
- [ ] More language panels
