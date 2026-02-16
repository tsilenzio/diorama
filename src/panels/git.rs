use ratatui::text::Line;

use super::*;

pub(super) fn git_log_panel() -> Panel {
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

pub(super) fn git_diff_panel() -> Panel {
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
