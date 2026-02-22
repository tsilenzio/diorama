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
diorama
```

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

## License

MIT
