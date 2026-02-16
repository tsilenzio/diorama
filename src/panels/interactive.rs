use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use super::*;

pub(super) fn python_repl_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            s("Python ", WHITE),
            s("3.12.0", CYAN),
            s(" (main, Oct  2 2024, 12:00:00) [Clang 15.0.0]", WHITE),
        ]),
        Line::from(vec![
            s("Type ", WHITE),
            s("\"help\"", GREEN),
            s(", ", WHITE),
            s("\"copyright\"", GREEN),
            s(" or ", WHITE),
            s("\"license\"", GREEN),
            s(" for more information.", WHITE),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("import ", BLUE),
            s("json", WHITE),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("data = {", WHITE),
            s("\"users\"", GREEN),
            s(": [", WHITE),
            s("\"alice\"", GREEN),
            s(", ", WHITE),
            s("\"bob\"", GREEN),
            s(", ", WHITE),
            s("\"charlie\"", GREEN),
            s("], ", WHITE),
            s("\"count\"", GREEN),
            s(": ", WHITE),
            s("3", MAGENTA),
            s("}", WHITE),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("json.dumps(data, indent=", WHITE),
            s("2", MAGENTA),
            s(", sort_keys=", WHITE),
            s("True", YELLOW),
            s(")", WHITE),
        ]),
        Line::from(s("'{\\n  \"count\": 3,\\n  \"users\": [\\n    \"alice\",\\n    \"bob\",\\n    \"charlie\"\\n  ]\\n}'", GREEN)),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("len", CYAN),
            s("(data[", WHITE),
            s("\"users\"", GREEN),
            s("])", WHITE),
        ]),
        Line::from(s("3", MAGENTA)),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("[x.upper() ", WHITE),
            s("for ", BLUE),
            s("x ", WHITE),
            s("in ", BLUE),
            s("data[", WHITE),
            s("\"users\"", GREEN),
            s("] ", WHITE),
            s("if ", BLUE),
            s("len", CYAN),
            s("(x) > ", WHITE),
            s("3", MAGENTA),
            s("]", WHITE),
        ]),
        Line::from(vec![
            s("['ALICE', 'CHARLIE']", WHITE),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("1", MAGENTA),
            s(" / ", WHITE),
            s("0", MAGENTA),
        ]),
        Line::from(vec![
            s("Traceback (most recent call last):", RED),
        ]),
        Line::from(vec![
            s("  File ", WHITE),
            s("\"<stdin>\"", GREEN),
            s(", line ", WHITE),
            s("1", MAGENTA),
            s(", in ", WHITE),
            s("<module>", WHITE),
        ]),
        Line::from(vec![
            sb("ZeroDivisionError", RED),
            s(": division by zero", WHITE),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
        ]),
    ];

    Panel {
        title: "Python REPL".into(),
        icon: '\u{e73c}',
        border_color: YELLOW,
        content: lines,
    }
}

pub(super) fn node_repl_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            s("Welcome to Node.js ", WHITE),
            s("v20.11.0", GREEN),
            s(".", WHITE),
        ]),
        Line::from(vec![
            s("Type ", WHITE),
            s("\".help\"", GREEN),
            s(" for more information.", WHITE),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("const ", BLUE),
            s("user = {", WHITE),
        ]),
        Line::from(vec![
            sb("... ", BRIGHT_BLACK),
            s("  name: ", WHITE),
            s("'Alice Chen'", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            sb("... ", BRIGHT_BLACK),
            s("  email: ", WHITE),
            s("'alice@example.com'", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            sb("... ", BRIGHT_BLACK),
            s("  age: ", WHITE),
            s("30", MAGENTA),
            s(",", WHITE),
        ]),
        Line::from(vec![
            sb("... ", BRIGHT_BLACK),
            s("  roles: [", WHITE),
            s("'admin'", GREEN),
            s(", ", WHITE),
            s("'user'", GREEN),
            s("]", WHITE),
        ]),
        Line::from(vec![
            sb("... ", BRIGHT_BLACK),
            s("}", WHITE),
        ]),
        Line::from(vec![
            s("undefined", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("Object", CYAN),
            s(".entries(user).map(([k, v]) => `${k}: ${", WHITE),
            s("typeof", BLUE),
            s(" v}`)", WHITE),
        ]),
        Line::from(vec![
            s("[ ", WHITE),
            s("'name: string'", GREEN),
            s(", ", WHITE),
            s("'email: string'", GREEN),
            s(", ", WHITE),
            s("'age: number'", GREEN),
            s(", ", WHITE),
            s("'roles: object'", GREEN),
            s(" ]", WHITE),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("user.roles.", WHITE),
            s("map", CYAN),
            s("(r => r.toUpperCase())", WHITE),
        ]),
        Line::from(vec![
            s("[ ", WHITE),
            s("'ADMIN'", GREEN),
            s(", ", WHITE),
            s("'USER'", GREEN),
            s(" ]", WHITE),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("const ", BLUE),
            s("{ name, ...rest } = user", WHITE),
        ]),
        Line::from(vec![
            s("undefined", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("JSON", CYAN),
            s(".parse(", WHITE),
            s("'{ invalid json }'", GREEN),
            s(")", WHITE),
        ]),
        Line::from(vec![
            s("Uncaught SyntaxError: Expected property name or '}' in JSON at position 2", RED),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("await ", BLUE),
            s("Promise", CYAN),
            s(".resolve(", WHITE),
            s("42", MAGENTA),
            s(")", WHITE),
        ]),
        Line::from(vec![
            s("42", MAGENTA),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
        ]),
    ];

    Panel {
        title: "Node REPL".into(),
        icon: '\u{e718}',
        border_color: GREEN,
        content: lines,
    }
}

pub(super) fn sudo_panel() -> Panel {
    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());

    let mut lines = vec![
        Line::from(sd("# Normal user prompt:", BRIGHT_BLACK)),
        Line::from(vec![
            s(&format!("{user}@host"), GREEN),
            s(":~$ ", WHITE),
            s("whoami", BRIGHT_WHITE),
        ]),
        Line::from(s(&user, WHITE)),
        Line::from(vec![
            s(&format!("{user}@host"), GREEN),
            s(":~$ ", WHITE),
            s("cat /etc/shadow", BRIGHT_WHITE),
        ]),
        Line::from(sb("cat: /etc/shadow: Permission denied", RED)),
        blank(),
        Line::from(sd("# Elevated prompt (sudo -s):", BRIGHT_BLACK)),
        Line::from(vec![s(&format!("[sudo] password for {user}: "), BRIGHT_WHITE)]),
        Line::from(vec![
            sfgbg(" \u{26a1}", YELLOW, RED),
            sfgbg(" root ", BRIGHT_WHITE, RED),
            sfgbg("", RED, YELLOW),
            sfgbg(" /etc ", BLACK, YELLOW),
            sfgbg(" ", YELLOW, BLACK),
            s(" ", WHITE),
        ]),
        Line::from(vec![s("whoami", BRIGHT_WHITE)]),
        Line::from(sb("root", RED)),
        Line::from(vec![s("id", BRIGHT_WHITE)]),
        Line::from(vec![
            s("uid=", WHITE),
            sb("0", RED),
            s("(root) gid=", WHITE),
            sb("0", RED),
            s("(root) groups=", WHITE),
            sb("0", RED),
            s("(root)", WHITE),
        ]),
        blank(),
        Line::from(vec![
            sfgbg(" \u{26a1}", YELLOW, RED),
            sfgbg(" root ", BRIGHT_WHITE, RED),
            sfgbg("", RED, YELLOW),
            sfgbg(" /etc ", BLACK, YELLOW),
            sfgbg(" ", YELLOW, BLACK),
            s(" ", WHITE),
        ]),
        Line::from(vec![
            s("head -3 ", BRIGHT_WHITE),
            s("/etc/shadow", WHITE),
        ]),
        Line::from(vec![
            s("root:", WHITE),
            s("$6$rAnDoMsAlT$xK9y2z...hashed_password...:19234:0:99999:7:::", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            s("daemon:", WHITE),
            s("*:19001:0:99999:7:::", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            s(&format!("{user}:"), WHITE),
            s("$6$aNoThErSaLt$mN3p4q...hashed_password...:19456:0:99999:7:::", BRIGHT_BLACK),
        ]),
    ];

    lines.push(blank());
    lines.push(Line::from(vec![
        sfgbg(" \u{26a1}", YELLOW, RED),
        sfgbg(" root ", BRIGHT_WHITE, RED),
        sfgbg("", RED, YELLOW),
        sfgbg(" /etc ", BLACK, YELLOW),
        sfgbg(" ", YELLOW, BLACK),
        s(" ", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("systemctl status nginx", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        s("\u{25cf} ", GREEN),
        sb("nginx.service", WHITE),
        s(" - A high performance web server and reverse proxy", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("     Loaded: loaded (", WHITE),
        s("/lib/systemd/system/nginx.service", BRIGHT_BLACK),
        s("; enabled)", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("     Active: ", WHITE),
        sb("active (running)", GREEN),
        s(" since Mon 2025-02-20 03:14:22 UTC; 3 days ago", WHITE),
    ]));

    Panel {
        title: "Root/Sudo".into(),
        icon: '\u{f0d9c}',
        border_color: RED,
        content: lines,
    }
}

pub(super) fn color_palette_panel() -> Panel {
    let colors = [
        (0, "Black"), (1, "Red"), (2, "Green"), (3, "Yellow"),
        (4, "Blue"), (5, "Magenta"), (6, "Cyan"), (7, "White"),
        (8, "Bright Black"), (9, "Bright Red"), (10, "Bright Green"),
        (11, "Bright Yellow"), (12, "Bright Blue"), (13, "Bright Magenta"),
        (14, "Bright Cyan"), (15, "Bright White"),
    ];

    let mut lines = Vec::new();
    for (idx, name) in colors {
        let color = Color::Indexed(idx);
        lines.push(Line::from(vec![
            Span::styled(
                "  ████  ",
                Style::default().fg(color).bg(color),
            ),
            s(&format!(" {:>2}  {}", idx, name), color),
        ]));
    }

    Panel {
        title: "Color Palette".into(),
        icon: '\u{f0266}',
        border_color: BRIGHT_MAGENTA,
        content: lines,
    }
}
