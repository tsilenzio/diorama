# diorama

Preview terminal color themes with realistic scenarios. Shows shell prompts, build output, git logs, diffs, docker containers, system monitors, and more -- all using ANSI colors 0-15 so your theme controls the appearance.

Useful for testing and designing terminal color schemes. Instead of staring at color swatches, see how your 16 colors actually play together across real-world terminal output.

## Install

```
brew install tsilenzio/tap/diorama
```

Or with Cargo:

```
cargo install diorama
```

## Usage

```
diorama                # launch the TUI
diorama --list         # print available panels and exit
diorama -p rust        # jump straight to the Rust panel
diorama --offline      # skip tool detection, use fallback prompts
```

| Flag | Short | Description |
|------|-------|-------------|
| `--panel <name>` | `-p` | Jump to a panel by name (case-insensitive substring) |
| `--offline` | | Skip tool detection for faster startup |
| `--list` | `-l` | List available panels and exit |

## Controls

| Key | Action |
|-----|--------|
| `Tab` / `Enter` | Toggle grid / fullscreen |
| `h` `l` / `←` `→` | Cycle panels (fullscreen) |
| `j` `k` / `↑` `↓` | Scroll |
| `PgUp` / `PgDn` | Page scroll |
| `Home` / `End` | Jump to top / bottom |
| `q` | Quit |
| `Esc` | Exit fullscreen or quit (press twice) |

## Panels

- **Language prompts** -- Python, Node.js, Rust, Go, C/C++, Zig, Java, C#, Ruby, Lua (detects your prompt engine: starship, oh-my-posh, p10k, etc.)
- **Git** -- log with branch graph, multi-file diff
- **System** -- docker containers, file listing, system info, process monitor
- **Dev tools** -- structured logs, ripgrep results, kubectl pods, JSON output
- **Interactive** -- Python REPL, Node REPL, sudo/root prompt contrast
- **Color palette** -- all 16 ANSI colors with swatches

## Development

```
mise install      # rust, task, lefthook, cocogitto
lefthook install  # git hooks
task              # list tasks
task lint         # rustfmt check + clippy
task test
task build        # release binary
```

CI runs rustfmt, clippy, and the tests on every push, and commit messages and
branch names get checked against conventional-commit rules on every PR.

## License

MIT
