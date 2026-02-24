use ratatui::text::Line;

use super::*;

pub(super) fn git_log_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            s("$ ", WHITE),
            s("git log --graph --oneline --all --decorate", BRIGHT_WHITE),
        ]),
        blank(),
        Line::from(vec![
            s("* ", YELLOW),
            s("a1b2c3d ", YELLOW),
            sb("(HEAD -> main) ", GREEN),
            s(
                "fix: handle edge case in JSON parser for nested arrays",
                WHITE,
            ),
        ]),
        Line::from(vec![
            s("| ", RED),
            s("Author: Alice Chen ", WHITE),
            s("<alice@dev.io>", BRIGHT_BLACK),
            s(" (", BRIGHT_BLACK),
            s("2 hours ago", CYAN),
            s(")", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("e4f5a6b ", YELLOW),
            s(
                "feat: add JSON output format with pretty-printing support",
                WHITE,
            ),
        ]),
        Line::from(vec![
            s("| ", RED),
            s("Author: Bob Martinez ", WHITE),
            s("<bob@dev.io>", BRIGHT_BLACK),
            s(" (", BRIGHT_BLACK),
            s("5 hours ago", CYAN),
            s(")", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            s("*   ", YELLOW),
            s("b7c8d9e ", YELLOW),
            sb("(origin/main) ", RED),
            s("Merge branch 'feature/auth' into main", WHITE),
        ]),
        Line::from(vec![sb("|\\  ", RED)]),
        Line::from(vec![
            s("| * ", RED),
            s("f0a1b2c ", YELLOW),
            sb("(origin/feature/auth) ", RED),
            s(
                "feat: implement JWT token validation and refresh flow",
                WHITE,
            ),
        ]),
        Line::from(vec![
            s("| * ", RED),
            s("3d4e5f6 ", YELLOW),
            s("feat: add login endpoint with rate limiting", WHITE),
        ]),
        Line::from(vec![
            s("| * ", RED),
            s("7a8b9c0 ", YELLOW),
            s("feat: create auth middleware with role-based access", WHITE),
        ]),
        Line::from(vec![
            s("| * ", RED),
            s("c2d3e4f ", YELLOW),
            s("test: add integration tests for auth flow", WHITE),
        ]),
        Line::from(vec![sb("|/  ", RED)]),
        Line::from(vec![
            s("*   ", YELLOW),
            s("d1e2f3a ", YELLOW),
            s("Merge branch 'refactor/config' into main", WHITE),
        ]),
        Line::from(vec![sb("|\\  ", RED)]),
        Line::from(vec![
            s("| * ", MAGENTA),
            s("5e6f7a8 ", YELLOW),
            s("refactor: extract config module into separate crate", WHITE),
        ]),
        Line::from(vec![
            s("| * ", MAGENTA),
            s("9b0c1d2 ", YELLOW),
            s("refactor: replace env vars with typed config struct", WHITE),
        ]),
        Line::from(vec![sb("|/  ", MAGENTA)]),
        Line::from(vec![
            s("* ", YELLOW),
            s("4b5c6d7 ", YELLOW),
            sb("(tag: v1.2.0) ", MAGENTA),
            s("chore: release version 1.2.0", WHITE),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("8e9f0a1 ", YELLOW),
            s(
                "docs: update README with API examples and curl snippets",
                WHITE,
            ),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("2b3c4d5 ", YELLOW),
            s("fix: prevent SQL injection in search endpoint", WHITE),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("6f7a8b9 ", YELLOW),
            s("perf: add composite database index for user lookups", WHITE),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("a3b4c5d ", YELLOW),
            sb("(tag: v1.1.0) ", MAGENTA),
            s("chore: release version 1.1.0", WHITE),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("e6f7a8b ", YELLOW),
            s("feat: add health check endpoint at /api/v1/health", WHITE),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("1c2d3e4 ", YELLOW),
            s(
                "ci: add GitHub Actions workflow for automated testing",
                WHITE,
            ),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("f5a6b7c ", YELLOW),
            s("fix: correct timezone handling in event scheduling", WHITE),
        ]),
        Line::from(vec![
            s("* ", YELLOW),
            s("8d9e0f1 ", YELLOW),
            sb("(tag: v1.0.0) ", MAGENTA),
            s("chore: initial release", WHITE),
        ]),
    ];

    Panel {
        title: "Git Log".into(),
        icon: '\u{e725}',
        border_color: YELLOW,
        content: lines,
    }
}

pub(super) fn git_diff_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            s("$ ", WHITE),
            s("git diff HEAD~1 --stat", BRIGHT_WHITE),
        ]),
        Line::from(vec![
            s(" src/handler.rs | ", WHITE),
            sb("14 ", GREEN),
            s("++++++++", GREEN),
            s("---", RED),
        ]),
        Line::from(vec![
            s(" src/config.rs  |  ", WHITE),
            sb("4 ", GREEN),
            s("++", GREEN),
        ]),
        Line::from(vec![
            s(" src/error.rs   |  ", WHITE),
            sb("8 ", GREEN),
            s("+++++", GREEN),
            s("-", RED),
        ]),
        Line::from(vec![
            s(" 3 files changed, ", WHITE),
            sb("19 insertions(+)", GREEN),
            s(", ", WHITE),
            sb("4 deletions(-)", RED),
        ]),
        blank(),
        Line::from(vec![sb(
            "diff --git a/src/handler.rs b/src/handler.rs",
            WHITE,
        )]),
        Line::from(vec![s("index 3a4b5c6..7d8e9f0 100644", BRIGHT_BLACK)]),
        Line::from(vec![sb("--- a/src/handler.rs", RED)]),
        Line::from(vec![sb("+++ b/src/handler.rs", GREEN)]),
        Line::from(vec![
            sb("@@ -12,7 +12,14 @@", CYAN),
            s(" impl Handler {", WHITE),
        ]),
        Line::from(vec![s(
            "     pub fn handle_request(&self, req: &Request) -> Result<Response> {",
            WHITE,
        )]),
        Line::from(vec![s("         let body = req.body();", WHITE)]),
        Line::from(vec![s(
            "         let headers = req.headers().clone();",
            WHITE,
        )]),
        Line::from(vec![sb("-        let response = process(body);", RED)]),
        Line::from(vec![sb(
            "+        let response = match self.process(body) {",
            GREEN,
        )]),
        Line::from(vec![sb("+            Ok(resp) => resp,", GREEN)]),
        Line::from(vec![sb(
            "+            Err(AppError::NotFound(msg)) => {",
            GREEN,
        )]),
        Line::from(vec![sb(
            "+                tracing::warn!(\"Resource not found: {msg}\");",
            GREEN,
        )]),
        Line::from(vec![sb(
            "+                return Ok(Response::not_found(&msg));",
            GREEN,
        )]),
        Line::from(vec![sb("+            }", GREEN)]),
        Line::from(vec![sb("+            Err(e) => {", GREEN)]),
        Line::from(vec![sb(
            "+                tracing::error!(\"Internal error: {e:#}\");",
            GREEN,
        )]),
        Line::from(vec![sb(
            "+                return Ok(Response::internal_error());",
            GREEN,
        )]),
        Line::from(vec![sb("+            }", GREEN)]),
        Line::from(vec![sb("+        };", GREEN)]),
        Line::from(vec![s(
            "         Ok(response.with_headers(headers))",
            WHITE,
        )]),
        Line::from(vec![s("     }", WHITE)]),
        blank(),
        Line::from(vec![sb(
            "diff --git a/src/config.rs b/src/config.rs",
            WHITE,
        )]),
        Line::from(vec![s("index 1a2b3c4..5d6e7f8 100644", BRIGHT_BLACK)]),
        Line::from(vec![sb("--- a/src/config.rs", RED)]),
        Line::from(vec![sb("+++ b/src/config.rs", GREEN)]),
        Line::from(vec![
            sb("@@ -5,6 +5,10 @@", CYAN),
            s(" pub struct Config {", WHITE),
        ]),
        Line::from(vec![s("     pub host: String,", WHITE)]),
        Line::from(vec![s("     pub port: u16,", WHITE)]),
        Line::from(vec![sb(
            "+    /// Maximum number of concurrent connections",
            GREEN,
        )]),
        Line::from(vec![sb("+    pub max_connections: usize,", GREEN)]),
        Line::from(vec![sb(
            "+    /// Request timeout in milliseconds (0 = no timeout)",
            GREEN,
        )]),
        Line::from(vec![sb("+    pub timeout_ms: u64,", GREEN)]),
        Line::from(vec![s("     pub database_url: String,", WHITE)]),
        Line::from(vec![s("     pub log_level: String,", WHITE)]),
        Line::from(vec![s(" }", WHITE)]),
        blank(),
        Line::from(vec![sb("diff --git a/src/error.rs b/src/error.rs", WHITE)]),
        Line::from(vec![s("index 8c9d0e1..2f3a4b5 100644", BRIGHT_BLACK)]),
        Line::from(vec![sb("--- a/src/error.rs", RED)]),
        Line::from(vec![sb("+++ b/src/error.rs", GREEN)]),
        Line::from(vec![
            sb("@@ -1,8 +1,14 @@", CYAN),
            s(" use thiserror::Error;", WHITE),
        ]),
        blank(),
        Line::from(vec![s(" #[derive(Debug, Error)]", WHITE)]),
        Line::from(vec![s(" pub enum AppError {", WHITE)]),
        Line::from(vec![sb("-    #[error(\"internal error\")]", RED)]),
        Line::from(vec![sb("-    Internal,", RED)]),
        Line::from(vec![sb("+    #[error(\"internal error: {0}\")]", GREEN)]),
        Line::from(vec![sb("+    Internal(String),", GREEN)]),
        Line::from(vec![sb("+", GREEN)]),
        Line::from(vec![sb("+    #[error(\"not found: {0}\")]", GREEN)]),
        Line::from(vec![sb("+    NotFound(String),", GREEN)]),
        Line::from(vec![sb("+", GREEN)]),
        Line::from(vec![sb(
            "+    #[error(\"request timeout after {0}ms\")]",
            GREEN,
        )]),
        Line::from(vec![sb("+    Timeout(u64),", GREEN)]),
        Line::from(vec![s(" }", WHITE)]),
    ];

    Panel {
        title: "Git Diff".into(),
        icon: '\u{e728}',
        border_color: GREEN,
        content: lines,
    }
}
