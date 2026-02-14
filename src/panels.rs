use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::scaffold::DetectedTools;

const RED: Color = Color::Indexed(1);
const GREEN: Color = Color::Indexed(2);
const YELLOW: Color = Color::Indexed(3);
const BLUE: Color = Color::Indexed(4);
const MAGENTA: Color = Color::Indexed(5);
const CYAN: Color = Color::Indexed(6);
const WHITE: Color = Color::Indexed(7);
const BRIGHT_BLACK: Color = Color::Indexed(8);
const BRIGHT_RED: Color = Color::Indexed(9);
const BRIGHT_GREEN: Color = Color::Indexed(10);
const BRIGHT_YELLOW: Color = Color::Indexed(11);
const BRIGHT_CYAN: Color = Color::Indexed(14);
const BRIGHT_MAGENTA: Color = Color::Indexed(13);
const BRIGHT_WHITE: Color = Color::Indexed(15);

fn s(text: &str, fg: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(fg))
}

fn sb(text: &str, fg: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(fg).add_modifier(Modifier::BOLD),
    )
}

fn sd(text: &str, fg: Color) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::default().fg(fg).add_modifier(Modifier::DIM),
    )
}

fn sfgbg(text: &str, fg: Color, bg: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(fg).bg(bg))
}

fn blank() -> Line<'static> {
    Line::from("")
}

fn col(text: &str, width: usize, fg: Color) -> Span<'static> {
    Span::styled(format!("{:<width$}", text), Style::default().fg(fg))
}

fn colb(text: &str, width: usize, fg: Color) -> Span<'static> {
    Span::styled(
        format!("{:<width$}", text),
        Style::default().fg(fg).add_modifier(Modifier::BOLD),
    )
}

pub struct Panel {
    pub title: String,
    pub icon: char,
    pub border_color: Color,
    pub content: Vec<Line<'static>>,
}

pub fn build_all(_tools: &DetectedTools) -> Vec<Panel> {
    vec![
        color_palette_panel(),
        python_panel(),
        node_panel(),
        rust_panel(),
        go_panel(),
        cpp_panel(),
        zig_panel(),
        java_panel(),
        git_log_panel(),
        git_diff_panel(),
        docker_panel(),
    ]
}

fn color_palette_panel() -> Panel {
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

fn python_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("pytest ", GREEN),
        s("-v tests/", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(sb("======================== test session starts =========================", WHITE)));
    lines.push(Line::from(vec![
        s("collected ", WHITE),
        sb("6 items", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![s("tests/test_auth.py::test_login ", WHITE), sb("PASSED", GREEN), sd("  [16%]", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![s("tests/test_auth.py::test_logout ", WHITE), sb("PASSED", GREEN), sd(" [33%]", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![s("tests/test_models.py::test_create ", WHITE), sb("PASSED", GREEN), sd(" [50%]", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![s("tests/test_models.py::test_delete ", WHITE), sb("FAILED", RED), sd(" [66%]", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![s("tests/test_api.py::test_get ", WHITE), sb("PASSED", GREEN), sd("    [83%]", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![s("tests/test_api.py::test_post ", WHITE), sb("PASSED", GREEN), sd("   [100%]", BRIGHT_BLACK)]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("=====", RED),
        s(" ", WHITE),
        sb("1 failed", RED),
        s(", ", WHITE),
        sb("5 passed", GREEN),
        s(" in ", WHITE),
        s("0.82s", BRIGHT_BLACK),
        s(" ", WHITE),
        sb("=====", RED),
    ]));

    Panel {
        title: "Python".into(),
        icon: '\u{e73c}',
        border_color: YELLOW,
        content: lines,
    }
}

fn node_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("npm ", RED),
        s("test", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![s("> ", BRIGHT_BLACK), s("my-node-app@1.0.0 test", WHITE)]));
    lines.push(Line::from(vec![s("> ", BRIGHT_BLACK), s("jest --verbose", WHITE)]));
    lines.push(blank());
    lines.push(Line::from(vec![sb(" PASS ", GREEN), s(" tests/auth.test.ts", WHITE)]));
    lines.push(Line::from(vec![s("  ✓ ", GREEN), s("should authenticate valid user ", WHITE), sd("(3 ms)", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![s("  ✓ ", GREEN), s("should reject invalid token ", WHITE), sd("(1 ms)", BRIGHT_BLACK)]));
    lines.push(blank());
    lines.push(Line::from(vec![sb(" FAIL ", RED), s(" tests/api.test.ts", WHITE)]));
    lines.push(Line::from(vec![s("  ✕ ", RED), s("should return 404 for missing user ", WHITE), sd("(5 ms)", BRIGHT_BLACK)]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("Tests:  ", WHITE),
        sb("1 failed", RED),
        s(", ", WHITE),
        sb("2 passed", GREEN),
        s(", 3 total", WHITE),
    ]));

    Panel {
        title: "Node.js".into(),
        icon: '\u{e718}',
        border_color: GREEN,
        content: lines,
    }
}

fn rust_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("cargo ", BRIGHT_WHITE),
        s("build", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![sb("   Compiling ", GREEN), s("libc v0.2.153", WHITE)]));
    lines.push(Line::from(vec![sb("   Compiling ", GREEN), s("serde v1.0.197", WHITE)]));
    lines.push(Line::from(vec![sb("   Compiling ", GREEN), s("my-rust-app v0.1.0", WHITE)]));
    lines.push(Line::from(vec![
        sb("warning", YELLOW),
        s("[unused_imports]", BRIGHT_BLACK),
        s(": unused import: `std::io::Read`", WHITE),
    ]));
    lines.push(Line::from(vec![
        sd(" --> ", BLUE),
        s("src/main.rs:3:5", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("warning", YELLOW),
        s(": `my-rust-app` generated 1 warning", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("    Finished ", GREEN),
        s("`dev` profile [unoptimized + debuginfo] in 2.31s", WHITE),
    ]));

    Panel {
        title: "Rust".into(),
        icon: '\u{e7a8}',
        border_color: RED,
        content: lines,
    }
}

fn go_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![s("go test ", CYAN), s("./...", WHITE)]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("ok  ", GREEN),
        s("  github.com/user/my-go-app/auth   ", WHITE),
        sd("0.012s", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("ok  ", GREEN),
        s("  github.com/user/my-go-app/models ", WHITE),
        sd("0.008s", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("FAIL", RED),
        s("  github.com/user/my-go-app/api    ", WHITE),
        sd("0.015s", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("--- ", RED),
        sb("FAIL: ", RED),
        s("TestGetUser ", WHITE),
        sd("(0.003s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    api_test.go:42: expected status 200, got 404", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(sb("FAIL", RED)));

    Panel {
        title: "Go".into(),
        icon: '\u{e627}',
        border_color: CYAN,
        content: lines,
    }
}

fn zig_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("zig ", BRIGHT_YELLOW),
        s("build", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("info", BRIGHT_CYAN),
        s(": ", WHITE),
        s("Compiling src/main.zig...", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("info", BRIGHT_CYAN),
        s(": ", WHITE),
        s("Linking my-zig-app...", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("error", RED),
        s(": expected expression, found ','", WHITE),
    ]));
    lines.push(Line::from(vec![
        sd(" --> ", BLUE),
        s("src/main.zig:14:22", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("   |", BLUE),
    ]));
    lines.push(Line::from(vec![
        sd("14 ", BRIGHT_BLACK),
        s("| ", BLUE),
        s("    try stdout.print(", WHITE),
        sb(",", RED),
        s(" .{});", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("   |", BLUE),
        s("                      ", WHITE),
        sb("^", RED),
    ]));

    Panel {
        title: "Zig".into(),
        icon: '\u{e6a9}',
        border_color: BRIGHT_YELLOW,
        content: lines,
    }
}

fn java_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("gradle ", GREEN),
        s("build", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":compileJava", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":processResources", WHITE),
        s(" UP-TO-DATE", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":classes", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":compileTestJava", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":test", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("4 tests completed, ", GREEN),
        sb("1 failed", RED),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("FAILURE: ", RED),
        s("Build failed with an exception.", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("BUILD FAILED", RED),
        s(" in 6s", BRIGHT_BLACK),
    ]));

    Panel {
        title: "Java".into(),
        icon: '\u{e738}',
        border_color: RED,
        content: lines,
    }
}

fn git_log_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("git log --oneline --graph --all", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("* ", RED),
        sb("a1b2c3d ", YELLOW),
        s("(", WHITE),
        sb("HEAD -> ", BRIGHT_CYAN),
        sb("main", GREEN),
        s(") ", WHITE),
        s("feat: add search endpoint", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("* ", RED),
        sb("e4f5g6h ", YELLOW),
        s("refactor: extract validation logic", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("*   ", RED),
        sb("i7j8k9l ", YELLOW),
        s("Merge branch 'feat/auth'", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("|\\", RED),
    ]));
    lines.push(Line::from(vec![
        sb("| * ", RED),
        sb("m0n1o2p ", YELLOW),
        s("feat: add JWT middleware", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("| * ", RED),
        sb("q3r4s5t ", YELLOW),
        s("feat: add login endpoint", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("|/", RED),
    ]));
    lines.push(Line::from(vec![
        sb("* ", RED),
        sb("u6v7w8x ", YELLOW),
        s("(", WHITE),
        sb("tag: ", BRIGHT_CYAN),
        sb("v0.1.0", BRIGHT_CYAN),
        s(") ", WHITE),
        s("chore: initial release", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("* ", RED),
        sb("y9z0a1b ", YELLOW),
        s("fix: handle null response body", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("* ", RED),
        sb("c2d3e4f ", YELLOW),
        s("(", WHITE),
        sb("origin/main", BRIGHT_RED),
        s(") ", WHITE),
        s("initial commit", WHITE),
    ]));

    Panel {
        title: "Git Log".into(),
        icon: '\u{f1d3}',
        border_color: BRIGHT_RED,
        content: lines,
    }
}

fn cpp_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("cmake ", BLUE),
        s("--build build/", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("[1/4] ", BRIGHT_BLACK),
        s("Building CXX object src/main.cpp.o", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("[2/4] ", BRIGHT_BLACK),
        s("Building CXX object src/utils.cpp.o", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("[3/4] ", BRIGHT_BLACK),
        s("Building CXX object src/server.cpp.o", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("src/server.cpp:28:15: ", WHITE),
        sb("error: ", RED),
        s("no matching function for call to 'listen'", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    server.", WHITE),
        sb("listen", BRIGHT_WHITE),
        s("(", WHITE),
        s("port", CYAN),
        s(", ", WHITE),
        s("backlog", CYAN),
        s(");", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        sb("^~~~~~", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("src/server.h:12:10: ", WHITE),
        s("note: ", BRIGHT_CYAN),
        s("candidate function not viable: requires 1 argument, but 2 were provided", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    void ", WHITE),
        sb("listen", BRIGHT_WHITE),
        s("(int port);", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("1 error", RED),
        s(" generated.", WHITE),
    ]));

    Panel {
        title: "C/C++".into(),
        icon: '\u{e61d}',
        border_color: BLUE,
        content: lines,
    }
}

fn git_diff_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("git diff src/auth.rs", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("diff --git a/src/auth.rs b/src/auth.rs", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- a/src/auth.rs", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("+++ b/src/auth.rs", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("@@ -14,8 +14,12 @@", CYAN),
        s(" fn validate_token(token: &str) -> Result<Claims> {", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("     let key = get_signing_key();", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("-    decode::<Claims>(token, &key, &Validation::default())", RED),
    ]));
    lines.push(Line::from(vec![
        sb("-        .map_err(|e| AuthError::InvalidToken(e.to_string()))", RED),
    ]));
    lines.push(Line::from(vec![
        sb("+    let mut validation = Validation::default();", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("+    validation.set_audience(&[\"my-app\"]);", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("+    validation.set_required_spec_claims(&[\"exp\", \"sub\", \"aud\"]);", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("+", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("+    decode::<Claims>(token, &key, &validation)", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("+        .map_err(|e| AuthError::TokenValidation {", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("+            source: e,", GREEN),
    ]));
    lines.push(Line::from(vec![
        sb("+        })", GREEN),
    ]));
    lines.push(Line::from(vec![
        s(" }", WHITE),
    ]));

    Panel {
        title: "Git Diff".into(),
        icon: '\u{f1d3}',
        border_color: MAGENTA,
        content: lines,
    }
}

fn docker_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("docker ps --format table", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    // Header
    lines.push(Line::from(vec![
        colb("CONTAINER ID", 14, BRIGHT_WHITE),
        colb("IMAGE", 22, BRIGHT_WHITE),
        colb("STATUS", 16, BRIGHT_WHITE),
        colb("PORTS", 22, BRIGHT_WHITE),
        colb("NAMES", 14, BRIGHT_WHITE),
    ]));
    // Rows
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
