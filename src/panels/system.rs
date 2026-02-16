use ratatui::style::Color;
use ratatui::text::Line;

use super::*;

pub(super) fn docker_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("docker ps --format table", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        colb("CONTAINER ID", 14, BRIGHT_WHITE),
        colb("IMAGE", 22, BRIGHT_WHITE),
        colb("STATUS", 16, BRIGHT_WHITE),
        colb("PORTS", 22, BRIGHT_WHITE),
        colb("NAMES", 14, BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        col("a1b2c3d4e5f6", 14, BRIGHT_BLACK),
        col("nginx:latest", 22, CYAN),
        col("Up 3 days", 16, GREEN),
        col("0.0.0.0:80->80/tcp", 22, WHITE),
        col("web-proxy", 14, BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        col("f6e5d4c3b2a1", 14, BRIGHT_BLACK),
        col("postgres:16", 22, CYAN),
        col("Up 3 days", 16, GREEN),
        col("5432/tcp", 22, WHITE),
        col("db", 14, BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        col("1a2b3c4d5e6f", 14, BRIGHT_BLACK),
        col("redis:7-alpine", 22, CYAN),
        col("Up 3 days", 16, GREEN),
        col("6379/tcp", 22, WHITE),
        col("cache", 14, BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        col("6f5e4d3c2b1a", 14, BRIGHT_BLACK),
        col("my-app:latest", 22, CYAN),
        sfgbg(" Restarting ", BRIGHT_WHITE, RED),
        col("", 5, WHITE),
        col("8080/tcp", 22, WHITE),
        col("api", 14, BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        col("b1c2d3e4f5a6", 14, BRIGHT_BLACK),
        col("grafana/grafana", 22, CYAN),
        col("Up 12 hours", 16, GREEN),
        col("0.0.0.0:3000->3000", 22, WHITE),
        col("monitoring", 14, BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("5", BRIGHT_GREEN),
        s(" containers, ", WHITE),
        sb("4", GREEN),
        s(" running, ", WHITE),
        sb("1", RED),
        s(" restarting", WHITE),
    ]));

    Panel {
        title: "Docker".into(),
        icon: '\u{f308}',
        border_color: BRIGHT_CYAN,
        content: lines,
    }
}

pub(super) fn file_listing_panel() -> Panel {
    const FP: usize = 12;
    const FS: usize = 6;
    const FU: usize = 7;
    const FG: usize = 7;
    const FD: usize = 14;
    const FX: usize = 2;

    let lines = vec![
        Line::from(vec![
            s("$ ", WHITE),
            s("eza -la --git --icons --group --time-style relative", BRIGHT_WHITE),
        ]),
        blank(),
        Line::from(vec![
            col("drwxr-xr-x", FP, BRIGHT_BLACK),
            colr("-", FS, BRIGHT_BLACK),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("23 Feb 10:00", FD, BLUE),
            col("  ", FX, GREEN),
            sb("\u{f413} ", BLUE),
            sb(".git/", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("drwxr-xr-x", FP, BRIGHT_BLACK),
            colr("-", FS, BRIGHT_BLACK),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("23 Feb 14:30", FD, BLUE),
            col("  ", FX, GREEN),
            sb("\u{f413} ", BLUE),
            sb("src/", BLUE),
        ]),
        Line::from(vec![
            col("drwxr-xr-x", FP, BRIGHT_BLACK),
            colr("-", FS, BRIGHT_BLACK),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("22 Feb 09:15", FD, BLUE),
            col("  ", FX, GREEN),
            sb("\u{f413} ", BLUE),
            sb("tests/", BLUE),
        ]),
        Line::from(vec![
            col("drwxr-xr-x", FP, BRIGHT_BLACK),
            colr("-", FS, BRIGHT_BLACK),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("21 Feb 11:45", FD, BLUE),
            col("  ", FX, GREEN),
            sb("\u{f413} ", BLUE),
            sb("benches/", BLUE),
        ]),
        Line::from(vec![
            col("drwxr-xr-x", FP, BRIGHT_BLACK),
            colr("-", FS, BRIGHT_BLACK),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("22 Feb 08:20", FD, BLUE),
            col("  ", FX, GREEN),
            sb("\u{f413} ", BLUE),
            sb("target/", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col(".rw-r--r--", FP, BRIGHT_BLACK),
            colr("2.1k", FS, GREEN),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("23 Feb 14:28", FD, BLUE),
            cold("M ", FX, YELLOW),
            s("\u{e7a8} ", RED),
            s("Cargo.toml", WHITE),
        ]),
        Line::from(vec![
            col(".rw-r--r--", FP, BRIGHT_BLACK),
            colr("42k", FS, GREEN),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("23 Feb 14:28", FD, BLUE),
            col("  ", FX, GREEN),
            s("\u{f023} ", BRIGHT_BLACK),
            s("Cargo.lock", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col(".rw-r--r--", FP, BRIGHT_BLACK),
            colr("847", FS, GREEN),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("23 Feb 14:30", FD, BLUE),
            cold("M ", FX, YELLOW),
            s("\u{e7a8} ", RED),
            s("src/main.rs", WHITE),
        ]),
        Line::from(vec![
            col(".rw-r--r--", FP, BRIGHT_BLACK),
            colr("3.2k", FS, GREEN),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("22 Feb 09:15", FD, BLUE),
            col("  ", FX, GREEN),
            s("\u{f0219} ", YELLOW),
            s("README.md", WHITE),
        ]),
        Line::from(vec![
            col(".rw-r--r--", FP, BRIGHT_BLACK),
            colr("156", FS, GREEN),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("20 Feb 16:45", FD, BLUE),
            col("  ", FX, GREEN),
            s("\u{f013} ", BRIGHT_BLACK),
            s(".gitignore", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col(".rw-r--r--", FP, BRIGHT_BLACK),
            colr("45", FS, GREEN),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("23 Feb 14:35", FD, BLUE),
            colb("N ", FX, GREEN),
            s("\u{f0219} ", YELLOW),
            s("TODO.md", GREEN),
        ]),
        Line::from(vec![
            col(".rwxr-xr-x", FP, BRIGHT_BLACK),
            colr("512", FS, GREEN),
            col("alice", FU, YELLOW),
            col("staff", FG, BRIGHT_BLACK),
            col("21 Feb 08:00", FD, BLUE),
            col("  ", FX, GREEN),
            s("\u{f489} ", GREEN),
            s("run.sh", GREEN),
        ]),
    ];

    Panel {
        title: "File Listing".into(),
        icon: '\u{f413}',
        border_color: BLUE,
        content: lines,
    }
}

pub(super) fn system_info_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            sb("                  ", GREEN),
            sb("alice", CYAN),
            s("@", WHITE),
            sb("macbook-pro", CYAN),
        ]),
        Line::from(vec![
            sb("       .:'        ", GREEN),
            s("──────────────────────────────────────────", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sb("    _ :'_ _       ", GREEN),
            sb("OS:       ", BLUE),
            s("macOS Sequoia 15.3 arm64", WHITE),
        ]),
        Line::from(vec![
            sb("  .'`_`-'_``.     ", GREEN),
            sb("Host:     ", BLUE),
            s("MacBook Pro (16-inch, 2024, Apple M3 Pro)", WHITE),
        ]),
        Line::from(vec![
            sb(" :________.-'     ", GREEN),
            sb("Kernel:   ", BLUE),
            s("Darwin 24.3.0", WHITE),
        ]),
        Line::from(vec![
            sb(" :_______:        ", GREEN),
            sb("Uptime:   ", BLUE),
            s("3 days, 7 hours, 42 minutes", WHITE),
        ]),
        Line::from(vec![
            sb("  :_______`-;     ", GREEN),
            sb("Shell:    ", BLUE),
            s("zsh 5.9 (with starship prompt)", WHITE),
        ]),
        Line::from(vec![
            sb("   `._.-._.'      ", GREEN),
            sb("Terminal: ", BLUE),
            s("Ghostty 1.1.0", WHITE),
        ]),
        blank(),
        Line::from(vec![
            sb("CPU:      ", BLUE),
            s("Apple M3 Pro ", WHITE),
            s("(12 cores: 6P + 6E)", BRIGHT_BLACK),
            s(" @ 4.06 GHz", CYAN),
        ]),
        Line::from(vec![
            sb("GPU:      ", BLUE),
            s("Apple M3 Pro ", WHITE),
            s("(18 cores)", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sb("Memory:   ", BLUE),
            s("8.2 GiB", YELLOW),
            s(" / ", BRIGHT_BLACK),
            s("36.0 GiB", WHITE),
            s(" (23%)", BRIGHT_BLACK),
            s(" ▮▮▮▮▮▯▯▯▯▯▯▯▯▯▯▯▯▯▯▯", GREEN),
        ]),
        Line::from(vec![
            sb("Disk (/): ", BLUE),
            s("287 GiB", YELLOW),
            s(" / ", BRIGHT_BLACK),
            s("1.0 TiB", WHITE),
            s(" (28%)", BRIGHT_BLACK),
            s(" ▮▮▮▮▮▮▯▯▯▯▯▯▯▯▯▯▯▯▯▯", GREEN),
        ]),
        Line::from(vec![
            sb("Battery:  ", BLUE),
            s("87%", GREEN),
            s(" [Charging]", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sb("Locale:   ", BLUE),
            s("en_US.UTF-8", WHITE),
        ]),
        blank(),
        Line::from(vec![
            s("  ", WHITE),
            sfgbg("   ", BLACK, Color::Indexed(0)),
            sfgbg("   ", RED, Color::Indexed(1)),
            sfgbg("   ", GREEN, Color::Indexed(2)),
            sfgbg("   ", YELLOW, Color::Indexed(3)),
            sfgbg("   ", BLUE, Color::Indexed(4)),
            sfgbg("   ", MAGENTA, Color::Indexed(5)),
            sfgbg("   ", CYAN, Color::Indexed(6)),
            sfgbg("   ", WHITE, Color::Indexed(7)),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sfgbg("   ", BRIGHT_BLACK, Color::Indexed(8)),
            sfgbg("   ", BRIGHT_RED, Color::Indexed(9)),
            sfgbg("   ", BRIGHT_GREEN, Color::Indexed(10)),
            sfgbg("   ", BRIGHT_YELLOW, Color::Indexed(11)),
            sfgbg("   ", BRIGHT_BLUE, Color::Indexed(12)),
            sfgbg("   ", BRIGHT_MAGENTA, Color::Indexed(13)),
            sfgbg("   ", BRIGHT_CYAN, Color::Indexed(14)),
            sfgbg("   ", BRIGHT_WHITE, Color::Indexed(15)),
        ]),
    ];

    Panel {
        title: "System Info".into(),
        icon: '\u{f108}',
        border_color: GREEN,
        content: lines,
    }
}

pub(super) fn process_monitor_panel() -> Panel {
    fn bar(label: &str, used: usize, total: usize, color: Color) -> Line<'static> {
        let pct = (used * 30) / total;
        let filled: String = "▮".repeat(pct);
        let empty: String = "▯".repeat(30 - pct);
        Line::from(vec![
            sb(label, BLUE),
            s("[", BRIGHT_BLACK),
            s(&filled, color),
            s(&empty, BRIGHT_BLACK),
            s("] ", BRIGHT_BLACK),
            s(&format!("{:.1}%", (used as f64 / total as f64) * 100.0), WHITE),
        ])
    }

    const PP: usize = 8;
    const PU: usize = 10;
    const PC: usize = 7;
    const PM: usize = 7;
    const PT: usize = 12;

    let lines = vec![
        bar("CPU0:  ", 45, 100, GREEN),
        bar("CPU1:  ", 78, 100, YELLOW),
        bar("CPU2:  ", 12, 100, GREEN),
        bar("CPU3:  ", 92, 100, RED),
        bar("CPU4:  ", 34, 100, GREEN),
        bar("CPU5:  ", 67, 100, YELLOW),
        bar("Mem:   ", 82, 100, YELLOW),
        bar("Swap:  ",  3, 100, GREEN),
        blank(),
        Line::from(vec![
            s("Tasks: ", WHITE),
            sb("142", BRIGHT_WHITE),
            s(", ", WHITE),
            sb("3", GREEN),
            s(" running, ", WHITE),
            sb("138", WHITE),
            s(" sleeping, ", WHITE),
            sb("1", RED),
            s(" stopped", WHITE),
        ]),
        Line::from(vec![
            s("Load average: ", WHITE),
            sb("2.45", YELLOW),
            s(" ", WHITE),
            sb("1.89", GREEN),
            s(" ", WHITE),
            sb("1.23", GREEN),
        ]),
        blank(),
        Line::from(vec![
            colrb("PID", PP, CYAN),
            colb("USER", PU, CYAN),
            colrb("CPU%", PC, CYAN),
            colrb("MEM%", PM, CYAN),
            colrb("TIME+", PT, CYAN),
            sb("COMMAND", CYAN),
        ]),
        Line::from(vec![
            colr("1284", PP, WHITE),
            col("alice", PU, YELLOW),
            colrb("42.1", PC, RED),
            colr("8.3", PM, WHITE),
            colr("14:22.38", PT, BRIGHT_BLACK),
            sb("node /home/alice/projects/api/dist/server.js", GREEN),
        ]),
        Line::from(vec![
            colr("892", PP, WHITE),
            col("postgres", PU, YELLOW),
            colr("18.5", PC, YELLOW),
            colrb("24.1", PM, RED),
            colr("8:45.12", PT, BRIGHT_BLACK),
            sb("postgres: writer process", GREEN),
        ]),
        Line::from(vec![
            colr("2341", PP, WHITE),
            col("alice", PU, YELLOW),
            colr("15.2", PC, YELLOW),
            colr("4.7", PM, WHITE),
            colr("2:11.84", PT, BRIGHT_BLACK),
            sb("cargo build --release -j8", GREEN),
        ]),
        Line::from(vec![
            colr("445", PP, WHITE),
            col("root", PU, RED),
            colr("8.7", PC, WHITE),
            colr("3.2", PM, WHITE),
            colr("1:45:30.2", PT, BRIGHT_BLACK),
            sb("dockerd --host=unix:///var/run/docker.sock", GREEN),
        ]),
        Line::from(vec![
            colr("3102", PP, WHITE),
            col("alice", PU, YELLOW),
            colr("5.3", PC, WHITE),
            colr("1.1", PM, WHITE),
            colr("0:08.42", PT, BRIGHT_BLACK),
            sb("rg --type rust \"handle_request\" ./src", GREEN),
        ]),
        Line::from(vec![
            colr("1567", PP, WHITE),
            col("redis", PU, YELLOW),
            colr("2.1", PC, WHITE),
            colr("6.8", PM, WHITE),
            colr("22:15.73", PT, BRIGHT_BLACK),
            sb("redis-server *:6379 [cluster]", GREEN),
        ]),
        Line::from(vec![
            colr("4521", PP, WHITE),
            col("alice", PU, YELLOW),
            colr("1.8", PC, WHITE),
            colr("2.4", PM, WHITE),
            colr("0:45.21", PT, BRIGHT_BLACK),
            sb("nvim src/handler.rs", GREEN),
        ]),
        Line::from(vec![
            colr("5678", PP, WHITE),
            col("alice", PU, YELLOW),
            colr("0.9", PC, WHITE),
            colr("0.3", PM, WHITE),
            colr("0:02.15", PT, BRIGHT_BLACK),
            sb("zsh -i", GREEN),
        ]),
    ];

    Panel {
        title: "Process Monitor".into(),
        icon: '\u{f085}',
        border_color: BRIGHT_GREEN,
        content: lines,
    }
}
