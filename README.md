# term-preview

A TUI tool for previewing terminal color themes. Shows realistic terminal scenarios like shell prompts, build output, git logs, and system tools using only ANSI colors 0-15 so your theme controls the appearance.

## Usage

```
cargo run
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

## License

MIT
