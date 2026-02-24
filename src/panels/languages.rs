use ratatui::text::Line;

use super::*;
use crate::scaffold::DetectedTools;

pub(super) fn python_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Python");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("pytest --cov=src -v --tb=short tests/", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(sb(
        "================================= test session starts ==================================",
        WHITE,
    )));
    lines.push(Line::from(vec![
        s("platform darwin -- Python ", WHITE),
        s("3.12.0", CYAN),
        s(", pytest-", WHITE),
        s("8.1.1", CYAN),
        s(", pluggy-", WHITE),
        s("1.4.0", CYAN),
        s(" -- /usr/local/bin/python3", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("cachedir: .pytest_cache", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![
        s("rootdir: ", WHITE),
        s("/Users/alice/projects/my-python-app", BLUE),
    ]));
    lines.push(Line::from(vec![
        s("configfile: ", WHITE),
        s("pyproject.toml", CYAN),
    ]));
    lines.push(Line::from(vec![
        s("plugins: ", WHITE),
        s("cov-5.0.0", CYAN),
        s(", ", WHITE),
        s("asyncio-0.23.5", CYAN),
        s(", ", WHITE),
        s("mock-3.12.0", CYAN),
    ]));
    lines.push(Line::from(vec![
        s("collected ", WHITE),
        sb("18 items", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("tests/test_auth.py::test_login_valid_credentials ", WHITE),
        sb("PASSED", GREEN),
        sd("                               [ 5%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_auth.py::test_login_invalid_password ", WHITE),
        sb("PASSED", GREEN),
        sd("                               [11%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_auth.py::test_token_refresh ", WHITE),
        sb("PASSED", GREEN),
        sd("                                       [16%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_auth.py::test_token_expiry ", WHITE),
        sb("PASSED", GREEN),
        sd(
            "                                        [22%]",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_models.py::test_user_creation ", WHITE),
        sb("PASSED", GREEN),
        sd("                                    [27%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_models.py::test_user_serialization ", WHITE),
        sb("FAILED", RED),
        sd("                              [33%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_models.py::test_user_validation ", WHITE),
        sb("PASSED", GREEN),
        sd("                                 [38%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_models.py::test_post_model ", WHITE),
        sb("PASSED", GREEN),
        sd("                                      [44%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_api.py::test_get_users ", WHITE),
        sb("PASSED", GREEN),
        sd(
            "                                          [50%]",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_api.py::test_create_user ", WHITE),
        sb("PASSED", GREEN),
        sd(
            "                                        [55%]",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_api.py::test_delete_user ", WHITE),
        sb("PASSED", GREEN),
        sd(
            "                                        [61%]",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_api.py::test_update_user ", WHITE),
        sb("PASSED", GREEN),
        sd(
            "                                        [66%]",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_api.py::test_list_users_pagination ", WHITE),
        sb("PASSED", GREEN),
        sd("                              [72%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_api.py::test_search_users ", WHITE),
        sb("PASSED", GREEN),
        sd("                                       [77%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_db.py::test_connection_pool ", WHITE),
        sb("PASSED", GREEN),
        sd("                                     [83%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_db.py::test_migration_rollback ", WHITE),
        sb("PASSED", GREEN),
        sd("                                  [88%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_db.py::test_transaction_isolation ", WHITE),
        sb("PASSED", GREEN),
        sd("                               [94%]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("tests/test_db.py::test_query_builder ", WHITE),
        sb("PASSED", GREEN),
        sd("                                      [100%]", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(sb(
        "======================================= FAILURES =======================================",
        RED,
    )));
    lines.push(Line::from(sb(
        "_____________________________ test_user_serialization ______________________________",
        RED,
    )));
    lines.push(Line::from(vec![
        s("tests/test_models.py", WHITE),
        s(":42: in ", BRIGHT_BLACK),
        s("test_user_serialization", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("def ", BLUE),
        s("test_user_serialization", BRIGHT_WHITE),
        s("(self):", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("        user = User(", WHITE),
        s("name", CYAN),
        s("=", WHITE),
        s("\"Alice\"", GREEN),
        s(", ", WHITE),
        s("email", CYAN),
        s("=", WHITE),
        s("\"alice@example.com\"", GREEN),
        s(", ", WHITE),
        s("age", CYAN),
        s("=", WHITE),
        s("30", MAGENTA),
        s(")", WHITE),
    ]));
    lines.push(Line::from(vec![
        s(">       ", WHITE),
        sb("assert ", BLUE),
        s("user.to_dict() == {", WHITE),
        s("\"name\"", GREEN),
        s(": ", WHITE),
        s("\"Alice\"", GREEN),
        s(", ", WHITE),
        s("\"email\"", GREEN),
        s(": ", WHITE),
        s("\"alice@example.com\"", GREEN),
        s(", ", WHITE),
        s("\"age\"", GREEN),
        s(": ", WHITE),
        s("30", MAGENTA),
        s("}", WHITE),
    ]));
    lines.push(Line::from(sb("E       AssertionError: assert {'name': 'Alice', 'email': 'alice@example.com', 'age': '30'} == ...", RED)));
    lines.push(Line::from(sb("E         Full diff:", RED)));
    lines.push(Line::from(vec![
        sb(
            "E           - {'name': 'Alice', 'email': 'alice@example.com', 'age': ",
            RED,
        ),
        sb("30", GREEN),
        sb("}", RED),
    ]));
    lines.push(Line::from(vec![
        sb(
            "E           + {'name': 'Alice', 'email': 'alice@example.com', 'age': ",
            RED,
        ),
        sb("'30'", RED),
        sb("}", RED),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("---------- coverage: ", WHITE),
        sb("87%", GREEN),
        s(" ----------", WHITE),
    ]));
    lines.push(Line::from(vec![s(
        "Name                      Stmts   Miss  Branch  BrPart  Cover",
        BRIGHT_BLACK,
    )]));
    lines.push(Line::from(s(
        "─────────────────────────────────────────────────────────────────",
        BRIGHT_BLACK,
    )));
    lines.push(Line::from(vec![
        s(
            "src/auth.py                  42      3       8       1    ",
            WHITE,
        ),
        sb("93%", GREEN),
    ]));
    lines.push(Line::from(vec![
        s(
            "src/models.py                38      8      12       4    ",
            WHITE,
        ),
        sb("72%", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s(
            "src/api.py                   56      4      18       2    ",
            WHITE,
        ),
        sb("91%", GREEN),
    ]));
    lines.push(Line::from(vec![
        s(
            "src/db.py                    64      2      14       1    ",
            WHITE,
        ),
        sb("96%", GREEN),
    ]));
    lines.push(Line::from(vec![
        s(
            "src/middleware.py             28      6       6       3    ",
            WHITE,
        ),
        sb("71%", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s(
            "src/utils.py                 18      0       4       0    ",
            WHITE,
        ),
        sb("100%", GREEN),
    ]));
    lines.push(Line::from(s(
        "─────────────────────────────────────────────────────────────────",
        BRIGHT_BLACK,
    )));
    lines.push(Line::from(vec![
        s(
            "TOTAL                       246     23      62      11    ",
            WHITE,
        ),
        sb("87%", GREEN),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("================================ short test summary info =================================", RED),
    ]));
    lines.push(Line::from(vec![
        sb("FAILED ", RED),
        s(
            "tests/test_models.py::test_user_serialization - AssertionError: assert ...",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        sb("=== ", RED),
        sb("1 failed", RED),
        s(", ", WHITE),
        sb("17 passed", GREEN),
        s(" in ", WHITE),
        s("4.72s", CYAN),
        sb(" ===", RED),
    ]));

    Panel {
        title: "Python".into(),
        icon: '\u{e73c}',
        border_color: YELLOW,
        content: lines,
    }
}

pub(super) fn node_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Node.js");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("npm test -- --verbose --coverage", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("> ", BRIGHT_BLACK),
        s("my-node-app@1.0.0", WHITE),
        s(" test", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("> ", BRIGHT_BLACK),
        s("jest --verbose --coverage", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sfgbg(" PASS ", BLACK, GREEN),
        s(" ", WHITE),
        s("src/services/__tests__/auth.service.test.ts", WHITE),
    ]));
    lines.push(Line::from(vec![s("  ", WHITE), sb("Auth Service", WHITE)]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should authenticate valid credentials ", WHITE),
        sd("(4ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should reject invalid token format ", WHITE),
        sd("(2ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s(
            "should refresh expired token and return new access token ",
            WHITE,
        ),
        sd("(1ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should revoke all sessions for user ", WHITE),
        sd("(3ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s(
            "should enforce rate limiting after 5 failed attempts ",
            WHITE,
        ),
        sd("(15ms)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sfgbg(" PASS ", BLACK, GREEN),
        s(" ", WHITE),
        s("src/services/__tests__/order.service.test.ts", WHITE),
    ]));
    lines.push(Line::from(vec![s("  ", WHITE), sb("Order Service", WHITE)]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should create order with valid items ", WHITE),
        sd("(7ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should calculate total with tax and shipping ", WHITE),
        sd("(2ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should apply discount code SAVE20 correctly ", WHITE),
        sd("(3ms)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sfgbg(" FAIL ", BRIGHT_WHITE, RED),
        s(" ", WHITE),
        s("src/controllers/__tests__/user.controller.test.ts", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        sb("User Controller", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should create user with valid payload ", WHITE),
        sd("(12ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should return 400 for missing required fields ", WHITE),
        sd("(4ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✕ ", RED),
        s("should handle duplicate email registration ", WHITE),
        sd("(8ms)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("✓ ", GREEN),
        s("should update user profile ", WHITE),
        sd("(6ms)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("  ● ", RED),
        sb("User Controller", WHITE),
        s(" › ", BRIGHT_BLACK),
        s("should handle duplicate email registration", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("    expect(", WHITE),
        s("received", RED),
        s(").toThrow(", WHITE),
        s("expected", GREEN),
        s(")", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("    Expected: ", WHITE),
        s("\"DuplicateEmailError\"", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("    Received: ", WHITE),
        s(
            "\"GenericError: constraint violation on column 'email'\"",
            RED,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("      ", WHITE),
        sd(
            "at Object.<anonymous> (src/controllers/__tests__/user.controller.test.ts:47:23)",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("Test Suites: ", WHITE),
        sb("1 failed", RED),
        s(", ", WHITE),
        sb("2 passed", GREEN),
        s(", 3 total", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("Tests:       ", WHITE),
        sb("1 failed", RED),
        s(", ", WHITE),
        sb("12 passed", GREEN),
        s(", 13 total", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("Snapshots:   ", WHITE),
        s("0 total", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("Time:        ", WHITE),
        s("3.142s", CYAN),
        s(", estimated 4s", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("Ran all test suites.", BRIGHT_BLACK)]));

    Panel {
        title: "Node.js".into(),
        icon: '\u{e718}',
        border_color: GREEN,
        content: lines,
    }
}

pub(super) fn rust_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Rust");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("cargo build --release 2>&1", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("   Compiling ", GREEN),
        s("libc v0.2.153", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("   Compiling ", GREEN),
        s("serde v1.0.197", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("   Compiling ", GREEN),
        s("tokio v1.36.0", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("   Compiling ", GREEN),
        s(
            "my-rust-app v0.1.0 (/Users/alice/projects/my-rust-app)",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        sb("warning", YELLOW),
        s("[unused_variable]", WHITE),
        s(": unused variable: ", WHITE),
        sb("`config`", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        sb("--> ", BLUE),
        s("src/main.rs", WHITE),
        s(":14:9", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("   ", WHITE), sb("|", BLUE)]));
    lines.push(Line::from(vec![
        sb("14 ", BLUE),
        sb("|", BLUE),
        s(
            "     let config = Config::load(\"config/production.toml\")?;",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("   ", WHITE),
        sb("|", BLUE),
        s("         ", WHITE),
        sb("^^^^^^", YELLOW),
        s(" ", WHITE),
        s("help: ", YELLOW),
        s("if this is intentional, prefix with underscore: ", WHITE),
        sb("`_config`", WHITE),
    ]));
    lines.push(Line::from(vec![s("   ", WHITE), sb("|", BLUE)]));
    lines.push(Line::from(vec![
        s("   ", WHITE),
        sb("= ", BLUE),
        s("note: ", WHITE),
        s("`#[warn(unused_variables)]` on by default", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("warning", YELLOW),
        s("[dead_code]", WHITE),
        s(": function `parse_duration` is never used", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        sb("--> ", BLUE),
        s("src/utils.rs", WHITE),
        s(":23:8", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("   ", WHITE), sb("|", BLUE)]));
    lines.push(Line::from(vec![
        sb("23 ", BLUE),
        sb("|", BLUE),
        s(
            "     pub fn parse_duration(input: &str) -> Result<Duration, ParseError> {",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("   ", WHITE),
        sb("|", BLUE),
        s("            ", WHITE),
        sb("^^^^^^^^^^^^^^", YELLOW),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("error", RED),
        s("[E0308]", WHITE),
        s(": mismatched types", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        sb("--> ", BLUE),
        s("src/handler.rs", WHITE),
        s(":28:12", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("   ", WHITE), sb("|", BLUE)]));
    lines.push(Line::from(vec![
        sb("26 ", BLUE),
        sb("|", BLUE),
        s(
            "     pub fn process(&self, req: &Request) -> Result<String, AppError> {",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("   ", WHITE),
        sb("|", BLUE),
        s(
            "                                                     ",
            WHITE,
        ),
        sb("------", WHITE),
        s(" expected `String` because of return type", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("27 ", BLUE),
        sb("|", BLUE),
        s(
            "         let response = self.client.get(req.url()).send().await?;",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        sb("28 ", BLUE),
        sb("|", BLUE),
        s("         Ok(response.body())", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("   ", WHITE),
        sb("|", BLUE),
        s("            ", WHITE),
        sb("^^^^^^^^^^^^^^^", RED),
        s(" expected `", WHITE),
        sb("String", CYAN),
        s("`, found `", WHITE),
        sb("&str", CYAN),
        s("`", WHITE),
    ]));
    lines.push(Line::from(vec![s("   ", WHITE), sb("|", BLUE)]));
    lines.push(Line::from(vec![
        s("   ", WHITE),
        sb("= ", BLUE),
        s("help: ", CYAN),
        s("try using a conversion method: `response.body()", WHITE),
        sb(".to_string()", GREEN),
        s("`", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("error", RED),
        s("[E0599]", WHITE),
        s(
            ": no method named `send` found for struct `RequestBuilder` in the current scope",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        sb("--> ", BLUE),
        s("src/handler.rs", WHITE),
        s(":27:56", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("   ", WHITE), sb("|", BLUE)]));
    lines.push(Line::from(vec![
        sb("27 ", BLUE),
        sb("|", BLUE),
        s(
            "         let response = self.client.get(req.url()).send().await?;",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("   ", WHITE),
        sb("|", BLUE),
        s(
            "                                                        ",
            WHITE,
        ),
        sb("^^^^", RED),
        s(" method not found in `RequestBuilder`", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("warning", YELLOW),
        s(
            ": `my-rust-app` (bin \"my-rust-app\") generated 2 warnings",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        sb("error", RED),
        s(": could not compile `my-rust-app` (bin \"my-rust-app\") due to 2 previous errors; 2 warnings emitted", WHITE),
    ]));

    Panel {
        title: "Rust".into(),
        icon: '\u{e7a8}',
        border_color: RED,
        content: lines,
    }
}

pub(super) fn go_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Go");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("go test -v -race -count=1 ./...", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestHandleRequest", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestHandleRequest/valid_GET_returns_200", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestHandleRequest/valid_GET_returns_200 ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s(
            "TestHandleRequest/valid_POST_creates_resource",
            BRIGHT_WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestHandleRequest/valid_POST_creates_resource ", WHITE),
        sd("(0.01s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s(
            "TestHandleRequest/PUT_updates_existing_resource",
            BRIGHT_WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestHandleRequest/PUT_updates_existing_resource ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestHandleRequest/DELETE_removes_resource", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestHandleRequest/DELETE_removes_resource ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestHandleRequest/invalid_method_returns_405", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- FAIL: ", RED),
        s("TestHandleRequest/invalid_method_returns_405 ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("        handler_test.go:67: expected status code ", WHITE),
        sb("405", GREEN),
        s(", got ", WHITE),
        sb("500", RED),
    ]));
    lines.push(Line::from(vec![
        s("        handler_test.go:68: response body: ", WHITE),
        s("{\"error\": \"internal server error\"}", RED),
    ]));
    lines.push(Line::from(vec![
        sb("--- FAIL: ", RED),
        s("TestHandleRequest ", WHITE),
        sd("(0.02s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestParseConfig", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestParseConfig ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestParseConfig/loads_from_yaml", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestParseConfig/loads_from_yaml ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestParseConfig/env_overrides_file", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestParseConfig/env_overrides_file ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("=== RUN   ", WHITE),
        s("TestDatabaseMigration", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("--- PASS: ", GREEN),
        s("TestDatabaseMigration ", WHITE),
        sd("(0.12s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![sb("FAIL", RED)]));
    lines.push(Line::from(vec![
        s(
            "FAIL    github.com/alice/my-go-app/internal/handler    ",
            WHITE,
        ),
        s("0.024s", CYAN),
    ]));
    lines.push(Line::from(vec![
        s(
            "ok      github.com/alice/my-go-app/internal/config     ",
            WHITE,
        ),
        s("0.003s", CYAN),
    ]));
    lines.push(Line::from(vec![
        s(
            "ok      github.com/alice/my-go-app/internal/db         ",
            WHITE,
        ),
        s("0.128s", CYAN),
    ]));
    lines.push(Line::from(vec![sb("FAIL", RED)]));

    Panel {
        title: "Go".into(),
        icon: '\u{e627}',
        border_color: CYAN,
        content: lines,
    }
}

pub(super) fn cpp_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "C/C++");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("cmake --build build/ --parallel $(nproc)", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        s("[", BRIGHT_BLACK),
        s("  1/8", GREEN),
        s("] ", BRIGHT_BLACK),
        s(
            "Building CXX object src/CMakeFiles/app.dir/main.cpp.o",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("src/main.cpp", WHITE),
        s(":", BRIGHT_BLACK),
        s("18", WHITE),
        s(":", BRIGHT_BLACK),
        s("15", WHITE),
        s(": ", BRIGHT_BLACK),
        sb("warning: ", YELLOW),
        s("unused parameter ", WHITE),
        sb("'argc'", WHITE),
        s(" [-Wunused-parameter]", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s("   18 | ", BLUE),
        s("int ", GREEN),
        s("main(", WHITE),
        s("int ", GREEN),
        su("argc", YELLOW),
        s(", ", WHITE),
        s("char", GREEN),
        s("* argv[])", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("      |               ", BLUE),
        sb("^~~~", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s("[", BRIGHT_BLACK),
        s("  2/8", GREEN),
        s("] ", BRIGHT_BLACK),
        s(
            "Building CXX object src/CMakeFiles/app.dir/config.cpp.o",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("[", BRIGHT_BLACK),
        s("  3/8", GREEN),
        s("] ", BRIGHT_BLACK),
        s(
            "Building CXX object src/CMakeFiles/app.dir/parser.cpp.o",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("src/parser.cpp", WHITE),
        s(":", BRIGHT_BLACK),
        s("42", WHITE),
        s(":", BRIGHT_BLACK),
        s("28", WHITE),
        s(": ", BRIGHT_BLACK),
        sb("error: ", RED),
        s("no matching function for call to ", WHITE),
        sb("'parse'", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("   42 | ", BLUE),
        s("    auto result = ", WHITE),
        sb("parse", RED),
        s("(input, ", WHITE),
        s("Options", GREEN),
        s("{.", WHITE),
        s("strict", CYAN),
        s(" = ", WHITE),
        s("true", MAGENTA),
        s("});", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("      |                            ", BLUE),
        sb("^~~~~", RED),
    ]));
    lines.push(Line::from(vec![
        s("src/parser.h", WHITE),
        s(":", BRIGHT_BLACK),
        s("12", WHITE),
        s(":", BRIGHT_BLACK),
        s("5", WHITE),
        s(": ", BRIGHT_BLACK),
        s("note: ", CYAN),
        s(
            "candidate function not viable: requires 1 argument, but 2 were provided",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("   12 | ", BLUE),
        s("    Result ", GREEN),
        s("parse(std::string_view input);", WHITE),
    ]));
    lines.push(Line::from(vec![s("      |     ", BLUE), s("^", CYAN)]));
    lines.push(Line::from(vec![
        s("src/parser.h", WHITE),
        s(":", BRIGHT_BLACK),
        s("18", WHITE),
        s(":", BRIGHT_BLACK),
        s("5", WHITE),
        s(": ", BRIGHT_BLACK),
        s("note: ", CYAN),
        s(
            "candidate function not viable: no known conversion from ",
            WHITE,
        ),
        sb("'Options'", CYAN),
        s(" to ", WHITE),
        sb("'bool'", CYAN),
    ]));
    lines.push(Line::from(vec![
        s("   18 | ", BLUE),
        s("    Result ", GREEN),
        s("parse(std::string_view input, bool validate);", WHITE),
    ]));
    lines.push(Line::from(vec![s("      |     ", BLUE), s("^", CYAN)]));
    lines.push(Line::from(vec![
        s("[", BRIGHT_BLACK),
        s("  4/8", GREEN),
        s("] ", BRIGHT_BLACK),
        s(
            "Building CXX object src/CMakeFiles/app.dir/server.cpp.o",
            WHITE,
        ),
    ]));
    lines.push(Line::from(vec![
        s("src/server.cpp", WHITE),
        s(":", BRIGHT_BLACK),
        s("91", WHITE),
        s(":", BRIGHT_BLACK),
        s("12", WHITE),
        s(": ", BRIGHT_BLACK),
        sb("warning: ", YELLOW),
        s("implicit conversion loses integer precision: ", WHITE),
        sb("'size_t'", CYAN),
        s(" to ", WHITE),
        sb("'int'", CYAN),
        s(" [-Wshorten-64-to-32]", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s("   91 | ", BLUE),
        s("    int count = buffer.size();", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("      |         ", BLUE),
        sb("~~~~~   ^~~~~~~~~~~~~", YELLOW),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![s(
        "2 warnings and 1 error generated.",
        WHITE,
    )]));
    lines.push(Line::from(vec![
        sb("ninja", RED),
        s(": build stopped: subcommand failed.", RED),
    ]));

    Panel {
        title: "C/C++".into(),
        icon: '\u{e61d}',
        border_color: BLUE,
        content: lines,
    }
}

pub(super) fn zig_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Zig");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("zig build -Doptimize=ReleaseSafe 2>&1", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        s("src/main.zig", WHITE),
        s(":", BRIGHT_BLACK),
        s("28", WHITE),
        s(":", BRIGHT_BLACK),
        s("21", WHITE),
        s(": ", BRIGHT_BLACK),
        sb("error: ", RED),
        s("expected type ", WHITE),
        sb("'[]const u8'", CYAN),
        s(", found ", WHITE),
        sb("'u8'", CYAN),
    ]));
    lines.push(Line::from(vec![
        s("    const msg = buffer[0..len]", WHITE),
        sb(";", RED),
    ]));
    lines.push(Line::from(vec![
        s("                      ", WHITE),
        sb("^~~~~~~~~~", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s("src/main.zig", WHITE),
        s(":", BRIGHT_BLACK),
        s("14", WHITE),
        s(":", BRIGHT_BLACK),
        s("5", WHITE),
        s(": ", BRIGHT_BLACK),
        s("note: ", CYAN),
        s("called from here:", WHITE),
    ]));
    lines.push(Line::from(vec![s(
        "    const result = try processMessage(stream, &read_buf);",
        WHITE,
    )]));
    lines.push(Line::from(vec![
        s("                       ", WHITE),
        sb("^~~~~~~~~~~~~~", CYAN),
    ]));
    lines.push(Line::from(vec![
        s("src/main.zig", WHITE),
        s(":", BRIGHT_BLACK),
        s("8", WHITE),
        s(":", BRIGHT_BLACK),
        s("29", WHITE),
        s(": ", BRIGHT_BLACK),
        s("note: ", CYAN),
        s("called from here:", WHITE),
    ]));
    lines.push(Line::from(vec![s(
        "    while (server.accept()) |conn| {",
        WHITE,
    )]));
    lines.push(Line::from(vec![
        s("                             ", WHITE),
        sb("^~~~~~", CYAN),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("src/net.zig", WHITE),
        s(":", BRIGHT_BLACK),
        s("7", WHITE),
        s(":", BRIGHT_BLACK),
        s("13", WHITE),
        s(": ", BRIGHT_BLACK),
        sb("warning: ", YELLOW),
        s("unused capture of lambda; consider using ", WHITE),
        sb("'_'", CYAN),
        s(" instead", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    for (items) ", WHITE),
        sb("|item|", YELLOW),
        s(" {", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("                 ", WHITE),
        sb("^~~~~", YELLOW),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("src/net.zig", WHITE),
        s(":", BRIGHT_BLACK),
        s("34", WHITE),
        s(":", BRIGHT_BLACK),
        s("9", WHITE),
        s(": ", BRIGHT_BLACK),
        sb("warning: ", YELLOW),
        s("local variable 'addr' is never read", WHITE),
    ]));
    lines.push(Line::from(vec![s(
        "    var addr = try std.net.Address.resolveIp(\"0.0.0.0\", port);",
        WHITE,
    )]));
    lines.push(Line::from(vec![s("        ", WHITE), sb("^~~~", YELLOW)]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("Build Summary: ", WHITE),
        s("1/4 steps succeeded; ", WHITE),
        sb("1 failed", RED),
    ]));
    lines.push(Line::from(vec![
        s("  zig ", WHITE),
        s("build-exe ", CYAN),
        s("my-zig-app", WHITE),
        s("... ", BRIGHT_BLACK),
        sb("FAILED", RED),
    ]));
    lines.push(Line::from(vec![
        s("  install ", CYAN),
        s("(skipped)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        sb("error: ", RED),
        s("the following command terminated with exit code 1:", WHITE),
    ]));
    lines.push(Line::from(vec![s(
        "  /usr/local/bin/zig build-exe src/main.zig -OReleaseSafe",
        BRIGHT_BLACK,
    )]));

    Panel {
        title: "Zig".into(),
        icon: '\u{e6a9}',
        border_color: YELLOW,
        content: lines,
    }
}

pub(super) fn java_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Java");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("./gradlew build --console=rich", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":compileJava", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("src/main/java/com/myapp/App.java", WHITE),
        s(":", BRIGHT_BLACK),
        s("15", WHITE),
        s(": ", BRIGHT_BLACK),
        sb("warning: ", YELLOW),
        s("[unchecked] unchecked cast", WHITE),
    ]));
    lines.push(Line::from(vec![s(
        "        List<String> items = (List<String>) rawData.get(\"items\");",
        WHITE,
    )]));
    lines.push(Line::from(vec![
        s("                            ", WHITE),
        sb("^", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s("  required: ", WHITE),
        s("List<String>", CYAN),
    ]));
    lines.push(Line::from(vec![
        s("  found:    ", WHITE),
        s("Object", CYAN),
    ]));
    lines.push(Line::from(vec![s("1 warning generated", YELLOW)]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":processResources", WHITE),
        sd(" UP-TO-DATE", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":compileTestJava", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":processTestResources", WHITE),
        sd(" UP-TO-DATE", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("> Task ", BRIGHT_BLACK),
        s(":test", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("com.myapp.AppTest > ", WHITE),
        s("testGreeting", BRIGHT_WHITE),
        sb(" PASSED", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("com.myapp.AppTest > ", WHITE),
        s("testNullInput", BRIGHT_WHITE),
        sb(" PASSED", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("com.myapp.AppTest > ", WHITE),
        s("testEmptyString", BRIGHT_WHITE),
        sb(" FAILED", RED),
    ]));
    lines.push(Line::from(vec![
        s("com.myapp.ServiceTest > ", WHITE),
        s("testCreateUser", BRIGHT_WHITE),
        sb(" PASSED", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("com.myapp.ServiceTest > ", WHITE),
        s("testDeleteUser", BRIGHT_WHITE),
        sb(" PASSED", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("com.myapp.ServiceTest > ", WHITE),
        s("testListUsers", BRIGHT_WHITE),
        sb(" PASSED", GREEN),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("java.lang.AssertionError", RED),
        s(": Expected <\"\">, got <null>", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("        at ", BRIGHT_BLACK),
        s("org.junit.jupiter.api.Assertions.assertEquals", WHITE),
        s("(Assertions.java:198)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("        at ", BRIGHT_BLACK),
        s("com.myapp.AppTest.testEmptyString", WHITE),
        s("(AppTest.java:28)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("6 tests completed, ", WHITE),
        sb("1 failed", RED),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("FAILURE: ", RED),
        s("Build failed with an exception.", WHITE),
    ]));
    lines.push(Line::from(vec![s("* What went wrong:", WHITE)]));
    lines.push(Line::from(vec![
        s("  Execution failed for task ", WHITE),
        sb("':test'", CYAN),
        s(".", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("BUILD FAILED", RED),
        s(" in 6s", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s(
        "4 actionable tasks: 3 executed, 1 up-to-date",
        BRIGHT_BLACK,
    )]));

    Panel {
        title: "Java".into(),
        icon: '\u{e738}',
        border_color: RED,
        content: lines,
    }
}

pub(super) fn csharp_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "C#");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s(
            "dotnet test --verbosity normal --logger console",
            BRIGHT_WHITE,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![s(
        "  Determining projects to restore...",
        BRIGHT_BLACK,
    )]));
    lines.push(Line::from(vec![
        s("  Restored ", WHITE),
        s("/Users/alice/projects/my-csharp-app/App.csproj", CYAN),
        s(" (in 0.5s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("  App -> ", WHITE),
        s(
            "/Users/alice/projects/my-csharp-app/bin/Debug/net8.0/App.dll",
            CYAN,
        ),
    ]));
    lines.push(Line::from(vec![
        s("  App.Tests -> ", WHITE),
        s(
            "/Users/alice/projects/my-csharp-app/tests/bin/Debug/net8.0/App.Tests.dll",
            CYAN,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![s(
        "Starting test execution, please wait...",
        WHITE,
    )]));
    lines.push(Line::from(vec![s(
        "A total of 1 test files matched the specified pattern.",
        BRIGHT_BLACK,
    )]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("  Passed ", GREEN),
        s("AppTests.MathService.TestAddition", WHITE),
        sd(" [12ms]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("  Passed ", GREEN),
        s("AppTests.MathService.TestSubtraction", WHITE),
        sd(" [1ms]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("  Passed ", GREEN),
        s("AppTests.MathService.TestMultiplication", WHITE),
        sd(" [< 1ms]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("  Passed ", GREEN),
        s("AppTests.StringService.TestConcatenation", WHITE),
        sd(" [3ms]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("  Passed ", GREEN),
        s("AppTests.StringService.TestTrimWhitespace", WHITE),
        sd(" [1ms]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("  Failed ", RED),
        s("AppTests.MathService.TestDivisionByZero", WHITE),
        sd(" [23ms]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        s("Expected: typeof(System.DivideByZeroException)", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        s("Actual:   no exception was thrown", RED),
    ]));
    lines.push(Line::from(vec![s("  Stack Trace:", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![
        s("    at ", BRIGHT_BLACK),
        s("AppTests.MathService.TestDivisionByZero()", WHITE),
        s(" in ", BRIGHT_BLACK),
        s("MathServiceTests.cs", CYAN),
        s(":line 34", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        sb("  Failed ", RED),
        s("AppTests.StringService.TestNullInput", WHITE),
        sd(" [5ms]", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        s("Expected: typeof(System.ArgumentNullException)", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("  ", WHITE),
        s(
            "Actual:   typeof(System.NullReferenceException) was thrown",
            RED,
        ),
    ]));
    lines.push(Line::from(vec![s("  Stack Trace:", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![
        s("    at ", BRIGHT_BLACK),
        s("App.StringService.Process(String input)", WHITE),
        s(" in ", BRIGHT_BLACK),
        s("StringService.cs", CYAN),
        s(":line 12", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("Failed!", RED),
        s("  - ", WHITE),
        s("Failed:     ", RED),
        s("2", WHITE),
        s(", Passed:     ", GREEN),
        s("5", WHITE),
        s(", Skipped:     0", BRIGHT_BLACK),
        s(", Total:     7", WHITE),
        s(", Duration: ", BRIGHT_BLACK),
        s("1.2s", CYAN),
    ]));

    Panel {
        title: "C#/.NET".into(),
        icon: '\u{f81a}',
        border_color: MAGENTA,
        content: lines,
    }
}

pub(super) fn ruby_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Ruby");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s(
            "bundle exec rspec --format documentation --color",
            BRIGHT_WHITE,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![sb("App", BRIGHT_WHITE)]));
    lines.push(Line::from(vec![s("  #greet", WHITE)]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("returns a greeting message for a given name", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s(
            "handles nil input gracefully by returning default greeting",
            GREEN,
        ),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("strips leading and trailing whitespace from name", GREEN),
    ]));
    lines.push(Line::from(vec![s("  #farewell", WHITE)]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("returns a farewell message ", GREEN),
        sd("(PENDING: not yet implemented)", YELLOW),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("raises ArgumentError for empty string", RED),
        s(" (FAILED - 1)", RED),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![sb("UserService", BRIGHT_WHITE)]));
    lines.push(Line::from(vec![s("  #create", WHITE)]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("creates a user with valid attributes", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("sends a welcome email after creation", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("raises RecordInvalid for duplicate email", GREEN),
    ]));
    lines.push(Line::from(vec![s("  #delete", WHITE)]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("soft deletes the user record", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("cancels active subscriptions before deletion", GREEN),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![s("Failures:", RED)]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("  1) ", RED),
        s("App #farewell raises ArgumentError for empty string", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("     ", WHITE),
        sb("Failure/Error: ", RED),
        s(
            "expect { app.farewell(\"\") }.to raise_error(ArgumentError)",
            WHITE,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("       expected ", WHITE),
        sb("ArgumentError", GREEN),
        s(" but nothing was raised", WHITE),
    ]));
    lines.push(Line::from(vec![s(
        "     # ./spec/app_spec.rb:24:in `block (3 levels) in <top (required)>'",
        BRIGHT_BLACK,
    )]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("Pending: ", YELLOW),
        sd(
            "(Failures listed here are expected and do not affect your suite's status)",
            BRIGHT_BLACK,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("  1) ", YELLOW),
        s("App #farewell returns a farewell message", WHITE),
    ]));
    lines.push(Line::from(vec![s("     # Not yet implemented", YELLOW)]));
    lines.push(Line::from(vec![s(
        "     # ./spec/app_spec.rb:18",
        BRIGHT_BLACK,
    )]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("Finished in ", WHITE),
        s("0.4821", CYAN),
        s(" seconds (files took ", WHITE),
        s("1.23", CYAN),
        s(" seconds to load)", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("10 examples", WHITE),
        s(", ", WHITE),
        sb("1 failure", RED),
        s(", ", WHITE),
        sb("1 pending", YELLOW),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![s("Failed examples:", RED)]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("rspec ./spec/app_spec.rb:22", RED),
        s(
            " # App #farewell raises ArgumentError for empty string",
            WHITE,
        ),
    ]));

    Panel {
        title: "Ruby".into(),
        icon: '\u{e791}',
        border_color: RED,
        content: lines,
    }
}

pub(super) fn lua_panel(tools: &DetectedTools) -> Panel {
    let mut lines = prompt_lines(tools, "Lua");
    lines.push(Line::from(vec![
        s("$ ", WHITE),
        s("busted --verbose spec/", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("●", GREEN),
        s(" ", WHITE),
        sb("http_client", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![s("  ", WHITE), sb("GET requests", WHITE)]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("returns status 200 for valid endpoint ", WHITE),
        sd("(0.02s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("decodes JSON response body correctly ", WHITE),
        sd("(0.01s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("follows redirects up to max depth ", WHITE),
        sd("(0.03s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("  ", WHITE), sb("POST requests", WHITE)]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("sends form-encoded data ", WHITE),
        sd("(0.01s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("sends JSON payload with correct content-type ", WHITE),
        sd("(0.02s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", RED),
        s("handles timeout when server is unresponsive ", WHITE),
        sd("(3.01s)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("●", GREEN),
        s(" ", WHITE),
        sb("config_parser", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![s("  ", WHITE), sb("loading", WHITE)]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("reads values from .lua config file ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("merges defaults with user overrides ", WHITE),
        sd("(0.01s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("raises error for malformed config ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![s("  ", WHITE), sb("serialization", WHITE)]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("round-trips table to string and back ", WHITE),
        sd("(0.01s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("preserves nested table structure ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("●", GREEN),
        s(" ", WHITE),
        sb("string_utils", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("trims whitespace from both ends ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("splits string by delimiter into table ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    ◼ ", GREEN),
        s("escapes pattern magic characters ", WHITE),
        sd("(0.00s)", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(sb("Failure:", RED)));
    lines.push(Line::from(vec![
        s("  spec/http_client_spec.lua", CYAN),
        s(":", BRIGHT_BLACK),
        s("47", WHITE),
        s(": ", BRIGHT_BLACK),
        s(
            "POST requests - handles timeout when server is unresponsive",
            WHITE,
        ),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("  ", WHITE),
        s("spec/http_client_spec.lua", CYAN),
        s(":48: Expected objects to be equal.", RED),
    ]));
    lines.push(Line::from(vec![s("  Passed in:", BRIGHT_BLACK)]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("Expected: ", WHITE),
        sb("nil", MAGENTA),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("Actual:   ", WHITE),
        sb("{", WHITE),
        s(" status = ", CYAN),
        s("408", MAGENTA),
        s(", body = ", CYAN),
        s("\"\"", GREEN),
        sb(" }", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(s(
        "────────────────────────────────────────────────────",
        BRIGHT_BLACK,
    )));
    lines.push(Line::from(vec![
        sb("14 ", WHITE),
        s("successes / ", WHITE),
        sb("1 ", RED),
        s("failure / ", WHITE),
        sb("0 ", YELLOW),
        s("errors / ", WHITE),
        sb("0 ", BRIGHT_BLACK),
        s("pending : ", WHITE),
        sb("3.127", CYAN),
        s(" seconds", WHITE),
    ]));

    Panel {
        title: "Lua".into(),
        icon: '\u{e620}',
        border_color: BLUE,
        content: lines,
    }
}
