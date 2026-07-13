use ratatui::style::Color;
use ratatui::text::{Line, Span};

use super::*;
use crate::scaffold::DetectedTools;

pub(super) fn python_repl_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            s("Python ", WHITE),
            s("3.12.0", CYAN),
            s(
                " (main, Oct  2 2024, 12:00:00) [Clang 15.0.0 (clang-1500.3.9.4)]",
                WHITE,
            ),
        ]),
        Line::from(vec![
            s("Type ", WHITE),
            s("\"help\"", GREEN),
            s(", ", WHITE),
            s("\"copyright\"", GREEN),
            s(", ", WHITE),
            s("\"credits\"", GREEN),
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
            s("from ", BLUE),
            s("datetime ", WHITE),
            s("import ", BLUE),
            s("datetime, timezone", WHITE),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("from ", BLUE),
            s("collections ", WHITE),
            s("import ", BLUE),
            s("Counter", WHITE),
        ]),
        Line::from(vec![sb(">>> ", GREEN)]),
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
        Line::from(s(
            "'{\\n  \"count\": 3,\\n  \"users\": [\\n    \"alice\",\\n    \"bob\",\\n    \"charlie\"\\n  ]\\n}'",
            GREEN,
        )),
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
        Line::from(vec![s("['ALICE', 'CHARLIE']", WHITE)]),
        Line::from(vec![sb(">>> ", GREEN)]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("Counter(", WHITE),
            s("\"mississippi\"", GREEN),
            s(").most_common(", WHITE),
            s("3", MAGENTA),
            s(")", WHITE),
        ]),
        Line::from(vec![s("[('s', 4), ('i', 4), ('p', 2)]", WHITE)]),
        Line::from(vec![sb(">>> ", GREEN)]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("now = datetime.now(timezone.utc)", WHITE),
        ]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("f", GREEN),
            s("\"Current time: {now.isoformat()}\"", GREEN),
        ]),
        Line::from(vec![s(
            "'Current time: 2025-02-23T10:15:32.456789+00:00'",
            GREEN,
        )]),
        Line::from(vec![sb(">>> ", GREEN)]),
        Line::from(vec![
            sb(">>> ", GREEN),
            s("1", MAGENTA),
            s(" / ", WHITE),
            s("0", MAGENTA),
        ]),
        Line::from(vec![s("Traceback (most recent call last):", RED)]),
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
        Line::from(vec![sb(">>> ", GREEN)]),
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
            s("],", WHITE),
        ]),
        Line::from(vec![
            sb("... ", BRIGHT_BLACK),
            s("  settings: { theme: ", WHITE),
            s("'dark'", GREEN),
            s(", notifications: ", WHITE),
            s("true", YELLOW),
            s(" }", WHITE),
        ]),
        Line::from(vec![sb("... ", BRIGHT_BLACK), s("}", WHITE)]),
        Line::from(vec![s("undefined", BRIGHT_BLACK)]),
        Line::from(vec![sb("> ", GREEN), s("user", WHITE)]),
        Line::from(vec![s("{", WHITE)]),
        Line::from(vec![
            s("  name: ", WHITE),
            s("'Alice Chen'", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  email: ", WHITE),
            s("'alice@example.com'", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![s("  age: ", WHITE), s("30", MAGENTA), s(",", WHITE)]),
        Line::from(vec![
            s("  roles: [ ", WHITE),
            s("'admin'", GREEN),
            s(", ", WHITE),
            s("'user'", GREEN),
            s(" ],", WHITE),
        ]),
        Line::from(vec![
            s("  settings: { theme: ", WHITE),
            s("'dark'", GREEN),
            s(", notifications: ", WHITE),
            s("true", YELLOW),
            s(" }", WHITE),
        ]),
        Line::from(vec![s("}", WHITE)]),
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
            s(", ", WHITE),
            s("'settings: object'", GREEN),
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
        Line::from(vec![s("undefined", BRIGHT_BLACK)]),
        Line::from(vec![sb("> ", GREEN), s("rest", WHITE)]),
        Line::from(vec![
            s("{ email: ", WHITE),
            s("'alice@example.com'", GREEN),
            s(", age: ", WHITE),
            s("30", MAGENTA),
            s(", roles: [ ", WHITE),
            s("'admin'", GREEN),
            s(", ", WHITE),
            s("'user'", GREEN),
            s(" ], settings: { theme: ", WHITE),
            s("'dark'", GREEN),
            s(", ... } }", WHITE),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("JSON", CYAN),
            s(".parse(", WHITE),
            s("'{ invalid json }'", GREEN),
            s(")", WHITE),
        ]),
        Line::from(vec![s(
            "Uncaught SyntaxError: Expected property name or '}' in JSON at position 2",
            RED,
        )]),
        Line::from(vec![
            s("    at JSON.parse (", BRIGHT_BLACK),
            s("<anonymous>", WHITE),
            s(")", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sb("> ", GREEN),
            s("await ", BLUE),
            s("Promise", CYAN),
            s(".resolve(", WHITE),
            s("42", MAGENTA),
            s(")", WHITE),
        ]),
        Line::from(vec![s("42", MAGENTA)]),
        Line::from(vec![sb("> ", GREEN)]),
    ];

    Panel {
        title: "Node REPL".into(),
        icon: '\u{e718}',
        border_color: GREEN,
        content: lines,
    }
}

pub(super) fn sudo_panel(tools: &DetectedTools) -> Panel {
    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());

    // Get a prompt from any captured one for the normal user display
    let user_prompt = tools
        .prompts
        .first()
        .map(|(_, text)| text.lines.clone())
        .unwrap_or_else(|| {
            vec![Line::from(vec![
                s(&format!("{user}@host"), GREEN),
                s(":~$ ", WHITE),
            ])]
        });

    let mut lines = vec![Line::from(sd("# Normal user prompt:", BRIGHT_BLACK))];
    lines.extend(user_prompt);
    lines.push(Line::from(vec![s("whoami", BRIGHT_WHITE)]));
    lines.push(Line::from(s(&user, WHITE)));
    lines.push(Line::from(vec![s("id", BRIGHT_WHITE)]));
    lines.push(Line::from(vec![
        s(
            &format!("uid=1000({user}) gid=1000({user}) groups=1000({user}),"),
            WHITE,
        ),
        s("4(adm),27(sudo),999(docker)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(sd(
        "# Attempting restricted access:",
        BRIGHT_BLACK,
    )));
    lines.push(Line::from(vec![s("cat /etc/shadow", BRIGHT_WHITE)]));
    lines.push(Line::from(sb("cat: /etc/shadow: Permission denied", RED)));
    lines.push(blank());
    lines.push(Line::from(sd("# Elevated prompt (sudo -s):", BRIGHT_BLACK)));
    lines.push(Line::from(vec![s(
        &format!("[sudo] password for {user}: "),
        BRIGHT_WHITE,
    )]));
    lines.push(Line::from(vec![
        sfgbg(" ⚡", YELLOW, RED),
        sfgbg(" root ", BRIGHT_WHITE, RED),
        sfgbg("", RED, YELLOW),
        sfgbg(" /etc ", BLACK, YELLOW),
        sfgbg(" ", YELLOW, BLACK),
        s(" ", WHITE),
    ]));
    lines.push(Line::from(vec![s("whoami", BRIGHT_WHITE)]));
    lines.push(Line::from(sb("root", RED)));
    lines.push(Line::from(vec![s("id", BRIGHT_WHITE)]));
    lines.push(Line::from(vec![
        s("uid=", WHITE),
        sb("0", RED),
        s("(root) gid=", WHITE),
        sb("0", RED),
        s("(root) groups=", WHITE),
        sb("0", RED),
        s("(root)", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sfgbg(" ⚡", YELLOW, RED),
        sfgbg(" root ", BRIGHT_WHITE, RED),
        sfgbg("", RED, YELLOW),
        sfgbg(" /etc ", BLACK, YELLOW),
        sfgbg(" ", YELLOW, BLACK),
        s(" ", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("head -3 ", BRIGHT_WHITE),
        s("/etc/shadow", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("root:", WHITE),
        s(
            "$6$rAnDoMsAlT$xK9y2z...hashed_password...:19234:0:99999:7:::",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(Line::from(vec![
        s("daemon:", WHITE),
        s("*:19001:0:99999:7:::", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s(&format!("{user}:"), WHITE),
        s(
            "$6$aNoThErSaLt$mN3p4q...hashed_password...:19456:0:99999:7:::",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sfgbg(" ⚡", YELLOW, RED),
        sfgbg(" root ", BRIGHT_WHITE, RED),
        sfgbg("", RED, YELLOW),
        sfgbg(" /etc ", BLACK, YELLOW),
        sfgbg(" ", YELLOW, BLACK),
        s(" ", WHITE),
    ]));
    lines.push(Line::from(vec![s("systemctl status nginx", BRIGHT_WHITE)]));
    lines.push(Line::from(vec![
        s("● ", GREEN),
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
    lines.push(Line::from(vec![
        s("   Main PID: ", WHITE),
        s("789", MAGENTA),
        s(" (nginx)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("      Tasks: ", WHITE),
        s("5", MAGENTA),
        s(" (limit: 4915)", BRIGHT_BLACK),
    ]));

    Panel {
        title: "Root/Sudo".into(),
        icon: '\u{f0d9c}',
        border_color: RED,
        content: lines,
    }
}

pub(super) fn color_palette_panel() -> Panel {
    let color_names = [
        (0, "Black", "Background, borders, shadows"),
        (1, "Red", "Errors, deletions, failures"),
        (2, "Green", "Success, additions, running"),
        (3, "Yellow", "Warnings, modified, pending"),
        (4, "Blue", "Info, directories, links"),
        (5, "Magenta", "Special, tags, decorators"),
        (6, "Cyan", "Types, constants, paths"),
        (7, "White", "Primary text, foreground"),
        (8, "Bright Black", "Comments, disabled, dimmed"),
        (9, "Bright Red", "Critical errors, urgent"),
        (10, "Bright Green", "Highlights, active items"),
        (11, "Bright Yellow", "Alerts, emphasis, search"),
        (12, "Bright Blue", "Links, references, info"),
        (13, "Bright Magenta", "Keywords, special syntax"),
        (14, "Bright Cyan", "Labels, metadata, hints"),
        (15, "Bright White", "Bold text, headings, focus"),
    ];

    let mut lines = Vec::new();

    lines.push(Line::from(sb("Standard Colors (0-7)", WHITE)));
    lines.push(blank());

    for &(idx, name, usage) in &color_names[..8] {
        let color = Color::Indexed(idx);
        lines.push(Line::from(vec![
            sfgbg("        ", color, color),
            s("  ", WHITE),
            s(&format!("{:>2}", idx), BRIGHT_BLACK),
            s("  ", WHITE),
            col(name, 16, color),
            sd(usage, BRIGHT_BLACK),
        ]));
    }

    lines.push(blank());
    lines.push(Line::from(sb("Bright Colors (8-15)", WHITE)));
    lines.push(blank());

    for &(idx, name, usage) in &color_names[8..] {
        let color = Color::Indexed(idx);
        lines.push(Line::from(vec![
            sfgbg("        ", color, color),
            s("  ", WHITE),
            s(&format!("{:>2}", idx), BRIGHT_BLACK),
            s("  ", WHITE),
            col(name, 16, color),
            sd(usage, BRIGHT_BLACK),
        ]));
    }

    lines.push(blank());
    lines.push(Line::from(sb("All 16 Colors:", WHITE)));
    lines.push(blank());

    // Row of all normal colors — wider swatches
    let mut normal_row: Vec<Span<'static>> = Vec::new();
    normal_row.push(s("  ", WHITE));
    for idx in 0..8u8 {
        normal_row.push(sfgbg("      ", Color::Indexed(idx), Color::Indexed(idx)));
    }
    lines.push(Line::from(normal_row));

    // Row of all bright colors
    let mut bright_row: Vec<Span<'static>> = Vec::new();
    bright_row.push(s("  ", WHITE));
    for idx in 8..16u8 {
        bright_row.push(sfgbg("      ", Color::Indexed(idx), Color::Indexed(idx)));
    }
    lines.push(Line::from(bright_row));

    lines.push(blank());

    // Normal/bold text samples
    lines.push(Line::from(sb("Text Styles:", WHITE)));
    lines.push(Line::from(vec![
        s("  Normal ", WHITE),
        sb(" Bold ", WHITE),
        sd(" Dim ", WHITE),
        si(" Italic ", WHITE),
        su(" Underline ", WHITE),
    ]));

    Panel {
        title: "Color Palette".into(),
        icon: '\u{f0266}',
        border_color: BRIGHT_MAGENTA,
        content: lines,
    }
}
