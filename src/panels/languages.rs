use ratatui::text::Line;

use super::*;

pub(super) fn python_panel() -> Panel {
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

pub(super) fn node_panel() -> Panel {
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

pub(super) fn rust_panel() -> Panel {
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

pub(super) fn go_panel() -> Panel {
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

pub(super) fn cpp_panel() -> Panel {
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

pub(super) fn zig_panel() -> Panel {
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

pub(super) fn java_panel() -> Panel {
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

pub(super) fn csharp_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("dotnet test ", MAGENTA),
        s("--verbosity normal", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("  Determining projects to restore...", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("  Restored ", WHITE),
        s("/Users/alice/projects/my-csharp-app/App.csproj", CYAN),
        s(" (in 0.5s)", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("  App -> ", WHITE),
        s("/Users/alice/projects/my-csharp-app/bin/Debug/net8.0/App.dll", CYAN),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("Starting test execution, please wait...", WHITE),
    ]));
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
        s("AppTests.StringService.TestConcatenation", WHITE),
        sd(" [3ms]", BRIGHT_BLACK),
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
    lines.push(Line::from(vec![
        s("  Stack Trace:", BRIGHT_BLACK),
    ]));
    lines.push(Line::from(vec![
        s("    at ", BRIGHT_BLACK),
        s("AppTests.MathService.TestDivisionByZero()", WHITE),
        s(" in ", BRIGHT_BLACK),
        s("MathServiceTests.cs", CYAN),
        s(":line 34", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("Failed!", RED),
        s("  - ", WHITE),
        s("Failed:     ", RED),
        s("1", WHITE),
        s(", Passed:     ", GREEN),
        s("3", WHITE),
        s(", Total:     4", WHITE),
        s(", Duration: ", BRIGHT_BLACK),
        s("0.8s", CYAN),
    ]));

    Panel {
        title: "C#/.NET".into(),
        icon: '\u{f81a}',
        border_color: MAGENTA,
        content: lines,
    }
}

pub(super) fn ruby_panel() -> Panel {
    let mut lines = Vec::new();

    lines.push(Line::from(s("$ ", WHITE)));
    lines.push(Line::from(vec![
        s("bundle exec rspec ", RED),
        s("--format documentation --color", BRIGHT_WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        sb("App", BRIGHT_WHITE),
    ]));
    lines.push(Line::from(vec![
        s("  #greet", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("returns a greeting message for a given name", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("    ", WHITE),
        s("handles nil input gracefully by returning default greeting", GREEN),
    ]));
    lines.push(Line::from(vec![
        s("  #farewell", WHITE),
    ]));
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
    lines.push(Line::from(vec![
        s("Failures:", RED),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("  1) ", RED),
        s("App #farewell raises ArgumentError for empty string", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("     ", WHITE),
        sb("Failure/Error: ", RED),
        s("expect { app.farewell(\"\") }.to raise_error(ArgumentError)", WHITE),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("       expected ", WHITE),
        sb("ArgumentError", GREEN),
        s(" but nothing was raised", WHITE),
    ]));
    lines.push(Line::from(vec![
        s("     # ./spec/app_spec.rb:24:in `block (3 levels) in <top (required)>'", BRIGHT_BLACK),
    ]));
    lines.push(blank());
    lines.push(Line::from(vec![
        s("Finished in ", WHITE),
        s("0.4821", CYAN),
        s(" seconds", WHITE),
    ]));
    lines.push(Line::from(vec![
        sb("5 examples", WHITE),
        s(", ", WHITE),
        sb("1 failure", RED),
        s(", ", WHITE),
        sb("1 pending", YELLOW),
    ]));

    Panel {
        title: "Ruby".into(),
        icon: '\u{e791}',
        border_color: RED,
        content: lines,
    }
}
