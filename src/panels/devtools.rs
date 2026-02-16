use ratatui::text::Line;

use super::*;

pub(super) fn log_output_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            sd("2025-02-23T10:15:32.123Z ", BRIGHT_BLACK),
            sfgbg(" DEBUG ", BLUE, BLACK),
            s(" ", WHITE),
            s("[server]   ", CYAN),
            s("Loading configuration from ", WHITE),
            si("/etc/myapp/config.toml", YELLOW),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:32.456Z ", BRIGHT_BLACK),
            sfgbg(" INFO  ", GREEN, BLACK),
            s(" ", WHITE),
            s("[server]   ", CYAN),
            s("Server starting on ", WHITE),
            sb("0.0.0.0:8080", WHITE),
            s(" (release build, v2.4.1)", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:32.789Z ", BRIGHT_BLACK),
            sfgbg(" INFO  ", GREEN, BLACK),
            s(" ", WHITE),
            s("[db]       ", CYAN),
            s("Connected to PostgreSQL at ", WHITE),
            su("postgres://app@db.internal:5432/production", BLUE),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:33.100Z ", BRIGHT_BLACK),
            sfgbg(" INFO  ", GREEN, BLACK),
            s(" ", WHITE),
            s("[auth]     ", CYAN),
            s("JWT signing key loaded (RS256, kid=", WHITE),
            s("\"key-2024-02\"", GREEN),
            s(")", WHITE),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:34.100Z ", BRIGHT_BLACK),
            sfgbg(" INFO  ", GREEN, BLACK),
            s(" ", WHITE),
            s("[http]     ", CYAN),
            sb("GET  ", GREEN),
            s("/api/v1/users ", WHITE),
            sb("200", GREEN),
            sd(" 12ms", BRIGHT_BLACK),
            s(" req_id=", BRIGHT_BLACK),
            s("a8f2c1d4", MAGENTA),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:34.234Z ", BRIGHT_BLACK),
            sfgbg(" INFO  ", GREEN, BLACK),
            s(" ", WHITE),
            s("[http]     ", CYAN),
            sb("POST ", YELLOW),
            s("/api/v1/users ", WHITE),
            sb("201", GREEN),
            sd(" 45ms", BRIGHT_BLACK),
            s(" req_id=", BRIGHT_BLACK),
            s("b3e7f9a2", MAGENTA),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:35.567Z ", BRIGHT_BLACK),
            sfgbg(" WARN  ", YELLOW, BLACK),
            s(" ", WHITE),
            s("[http]     ", CYAN),
            sb("GET  ", GREEN),
            s("/api/v1/users/999 ", WHITE),
            sb("404", YELLOW),
            sd(" 3ms", BRIGHT_BLACK),
            s(" -- resource not found", YELLOW),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:35.890Z ", BRIGHT_BLACK),
            sfgbg(" WARN  ", YELLOW, BLACK),
            s(" ", WHITE),
            s("[rate]     ", CYAN),
            s("Rate limit approaching for client ", WHITE),
            s("192.168.1.42", MAGENTA),
            s(": ", WHITE),
            sb("890/1000", YELLOW),
            s(" requests/min", WHITE),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:37.123Z ", BRIGHT_BLACK),
            sfgbg(" ERROR ", RED, BLACK),
            s(" ", WHITE),
            s("[db]       ", CYAN),
            s("Query timeout after ", WHITE),
            sb("30s", RED),
            s(": SELECT * FROM orders WHERE user_id = $1 AND status IN ($2, $3)", WHITE),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:37.456Z ", BRIGHT_BLACK),
            sfgbg(" ERROR ", RED, BLACK),
            s(" ", WHITE),
            s("[http]     ", CYAN),
            sb("POST ", YELLOW),
            s("/api/v1/orders ", WHITE),
            sb("500", RED),
            sd(" 30012ms", BRIGHT_BLACK),
            s(" -- internal server error", RED),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:38.789Z ", BRIGHT_BLACK),
            sfgbg(" FATAL ", BRIGHT_WHITE, RED),
            s(" ", WHITE),
            s("[server]   ", CYAN),
            s("Unrecoverable error: database connection pool collapsed -- shutting down", RED),
        ]),
        Line::from(vec![
            sd("2025-02-23T10:15:39.100Z ", BRIGHT_BLACK),
            sfgbg(" INFO  ", GREEN, BLACK),
            s(" ", WHITE),
            s("[server]   ", CYAN),
            s("All connections drained. Server stopped.", WHITE),
        ]),
    ];

    Panel {
        title: "Log Output".into(),
        icon: '\u{f4ed}',
        border_color: YELLOW,
        content: lines,
    }
}

pub(super) fn ripgrep_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            s("$ ", WHITE),
            s("rg ", BRIGHT_WHITE),
            sb("\"handle_request\"", YELLOW),
            s(" --type rust -C 1 --heading --line-number", BRIGHT_WHITE),
        ]),
        blank(),
        Line::from(sb("src/handler.rs", MAGENTA)),
        Line::from(vec![
            sd("11", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("impl Handler {", WHITE),
        ]),
        Line::from(vec![
            s("12", GREEN),
            s(":", BRIGHT_BLACK),
            s("    pub fn ", BLUE),
            sfgbg("handle_request", BLACK, YELLOW),
            s("(&self, req: &Request) -> Result<Response> {", WHITE),
        ]),
        Line::from(vec![
            sd("13", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("        let body = req.body();", WHITE),
        ]),
        Line::from(vec![
            sd("--", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sd("27", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("        // Validate and then delegate to the inner handler", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            s("28", GREEN),
            s(":", BRIGHT_BLACK),
            s("        let result = self.", WHITE),
            sfgbg("handle_request", BLACK, YELLOW),
            s("_inner(body, &headers)?;", WHITE),
        ]),
        Line::from(vec![
            sd("29", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("        Ok(Response::new(result))", WHITE),
        ]),
        Line::from(vec![
            sd("--", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sd("44", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("    /// Internal handler with error recovery", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            s("45", GREEN),
            s(":", BRIGHT_BLACK),
            s("    fn ", BLUE),
            sfgbg("handle_request", BLACK, YELLOW),
            s("_inner(&self, body: &[u8], headers: &Headers) -> Result<Vec<u8>> {", WHITE),
        ]),
        Line::from(vec![
            sd("46", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("        let parsed = serde_json::from_slice(body)?;", WHITE),
        ]),
        blank(),
        Line::from(sb("src/router.rs", MAGENTA)),
        Line::from(vec![
            sd("7", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("use crate::handler::Handler;", WHITE),
        ]),
        Line::from(vec![
            s("8", GREEN),
            s(":", BRIGHT_BLACK),
            s("use crate::handler::{Handler, ", WHITE),
            sfgbg("handle_request", BLACK, YELLOW),
            s("};", WHITE),
        ]),
        Line::from(vec![
            sd("9", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("use crate::middleware::auth_guard;", WHITE),
        ]),
        Line::from(vec![
            sd("--", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            sd("33", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("            .route(\"/api/v1/health\", get(health_check))", WHITE),
        ]),
        Line::from(vec![
            s("34", GREEN),
            s(":", BRIGHT_BLACK),
            s("            .route(\"/api/v1/process\", post(", WHITE),
            sfgbg("handle_request", BLACK, YELLOW),
            s("))", WHITE),
        ]),
        Line::from(vec![
            sd("35", BRIGHT_BLACK),
            sd("-", BRIGHT_BLACK),
            s("            .route(\"/api/v1/status\", get(status_check))", WHITE),
        ]),
        blank(),
        Line::from(vec![
            sb("5 matches", GREEN),
            s(" across ", WHITE),
            sb("2 files", GREEN),
            s(" (searched ", BRIGHT_BLACK),
            s("47 files", WHITE),
            s(" in ", BRIGHT_BLACK),
            s("12.4ms", CYAN),
            s(")", BRIGHT_BLACK),
        ]),
    ];

    Panel {
        title: "Ripgrep".into(),
        icon: '\u{f422}',
        border_color: MAGENTA,
        content: lines,
    }
}

pub(super) fn kubectl_panel() -> Panel {
    const KP: usize = 14;
    const KN: usize = 38;
    const KR: usize = 8;
    const KS: usize = 19;
    const KX: usize = 12;

    let lines = vec![
        Line::from(vec![
            s("$ ", WHITE),
            s("kubectl get pods --all-namespaces", BRIGHT_WHITE),
        ]),
        blank(),
        Line::from(vec![
            colb("NAMESPACE", KP, WHITE),
            colb("NAME", KN, WHITE),
            colb("READY", KR, WHITE),
            colb("STATUS", KS, WHITE),
            colb("RESTARTS", KX, WHITE),
            sb("AGE", WHITE),
        ]),
        Line::from(vec![
            col("production", KP, CYAN),
            col("api-server-7d8f6b5c4-x2k9l", KN, WHITE),
            col("1/1", KR, GREEN),
            colb("Running", KS, GREEN),
            col("0", KX, WHITE),
            s("3d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("production", KP, CYAN),
            col("api-server-7d8f6b5c4-m4n7p", KN, WHITE),
            col("1/1", KR, GREEN),
            colb("Running", KS, GREEN),
            col("0", KX, WHITE),
            s("3d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("production", KP, CYAN),
            col("worker-5f9a8c7d2-j8h3q", KN, WHITE),
            col("1/1", KR, GREEN),
            colb("Running", KS, GREEN),
            col("2", KX, WHITE),
            s("1d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("production", KP, CYAN),
            col("worker-5f9a8c7d2-p2l6r", KN, WHITE),
            col("0/1", KR, RED),
            colb("CrashLoopBackOff", KS, RED),
            colb("14 (4m ago)", KX, RED),
            s("1d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("production", KP, CYAN),
            col("scheduler-6c4d3b2a1-v5w8x", KN, WHITE),
            col("0/1", KR, YELLOW),
            colb("Pending", KS, YELLOW),
            col("0", KX, WHITE),
            s("5m", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("production", KP, CYAN),
            col("redis-master-0", KN, WHITE),
            col("1/1", KR, GREEN),
            colb("Running", KS, GREEN),
            col("0", KX, WHITE),
            s("7d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("production", KP, CYAN),
            col("nginx-ingress-7b6a5c4d3-q9r2s", KN, WHITE),
            col("1/1", KR, GREEN),
            colb("Running", KS, GREEN),
            col("0", KX, WHITE),
            s("14d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("monitoring", KP, YELLOW),
            col("prometheus-server-0", KN, WHITE),
            col("2/2", KR, GREEN),
            colb("Running", KS, GREEN),
            col("0", KX, WHITE),
            s("14d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("monitoring", KP, YELLOW),
            col("grafana-6f8d9e7c5-h2j4k", KN, WHITE),
            col("1/1", KR, GREEN),
            colb("Running", KS, GREEN),
            col("1 (2d ago)", KX, WHITE),
            s("14d", BRIGHT_BLACK),
        ]),
        Line::from(vec![
            col("kube-system", KP, MAGENTA),
            col("coredns-5d78c9869d-v4w7x", KN, WHITE),
            col("1/1", KR, GREEN),
            colb("Running", KS, GREEN),
            col("0", KX, WHITE),
            s("30d", BRIGHT_BLACK),
        ]),
    ];

    Panel {
        title: "Kubernetes".into(),
        icon: '\u{f10fe}',
        border_color: BLUE,
        content: lines,
    }
}

pub(super) fn json_panel() -> Panel {
    let lines = vec![
        Line::from(vec![
            s("$ ", WHITE),
            s("curl -s https://api.example.com/v1/config | jq .", BRIGHT_WHITE),
        ]),
        Line::from(s("{", WHITE)),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"name\"", BLUE),
            s(": ", WHITE),
            s("\"my-service\"", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"version\"", BLUE),
            s(": ", WHITE),
            s("\"2.4.1\"", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"port\"", BLUE),
            s(": ", WHITE),
            s("8080", MAGENTA),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"debug\"", BLUE),
            s(": ", WHITE),
            s("false", YELLOW),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"database\"", BLUE),
            s(": {", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            sb("\"host\"", BLUE),
            s(": ", WHITE),
            s("\"db.internal.example.com\"", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            sb("\"port\"", BLUE),
            s(": ", WHITE),
            s("5432", MAGENTA),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            sb("\"pool_size\"", BLUE),
            s(": ", WHITE),
            s("25", MAGENTA),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            sb("\"ssl\"", BLUE),
            s(": ", WHITE),
            s("true", YELLOW),
        ]),
        Line::from(vec![
            s("  }", WHITE),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"features\"", BLUE),
            s(": [", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            s("\"auth\"", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            s("\"caching\"", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            s("\"rate-limiting\"", GREEN),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("    ", WHITE),
            s("\"websockets\"", GREEN),
        ]),
        Line::from(vec![
            s("  ]", WHITE),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"max_retries\"", BLUE),
            s(": ", WHITE),
            s("3", MAGENTA),
            s(",", WHITE),
        ]),
        Line::from(vec![
            s("  ", WHITE),
            sb("\"maintenance_mode\"", BLUE),
            s(": ", WHITE),
            s("null", RED),
        ]),
        Line::from(s("}", WHITE)),
    ];

    Panel {
        title: "JSON".into(),
        icon: '\u{e60b}',
        border_color: YELLOW,
        content: lines,
    }
}
