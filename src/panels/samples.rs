use ratatui::style::Color;
use ratatui::text::Line;

use ansi_to_tui::IntoText;

use super::*;
use crate::scaffold::DetectedTools;

// Build a panel from captured tool output. The samples are real ANSI dumps
// produced by scripts/generate-samples and embedded at compile time. The
// captures are just the command's output, so we prepend the shell prompt with
// the command typed after it, then the output.
fn sample_panel(
    tools: &DetectedTools,
    prompt_lang: &str,
    command: &str,
    title: &str,
    icon: char,
    color: Color,
    ansi: &[u8],
) -> Panel {
    let mut content = prompt_with_command(tools, prompt_lang, command);
    match ansi.into_text() {
        Ok(text) => content.extend(text.lines),
        Err(_) => content.push(Line::from(s("(failed to parse sample)", RED))),
    }
    Panel {
        title: title.into(),
        icon,
        border_color: color,
        content,
    }
}

pub(super) fn python_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "Python",
        "pytest --cov=src -v --tb=short tests/",
        "Python (real)",
        '\u{e73c}',
        YELLOW,
        include_bytes!("../../samples/python-pytest.ansi"),
    )
}

pub(super) fn node_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "Node.js",
        "npx vitest run",
        "Node.js (real)",
        '\u{e718}',
        GREEN,
        include_bytes!("../../samples/node-vitest.ansi"),
    )
}

pub(super) fn rust_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "Rust",
        "cargo test",
        "Rust (real)",
        '\u{e7a8}',
        RED,
        include_bytes!("../../samples/rust-cargo-test.ansi"),
    )
}

pub(super) fn java_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "Java",
        "gradle test",
        "Java (real)",
        '\u{e738}',
        RED,
        include_bytes!("../../samples/java-gradle.ansi"),
    )
}

pub(super) fn zig_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "Zig",
        "zig build test",
        "Zig (real)",
        '\u{e6a9}',
        YELLOW,
        include_bytes!("../../samples/zig-test.ansi"),
    )
}

pub(super) fn cpp_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "C/C++",
        "cmake -B build && cmake --build build",
        "C/C++ (real)",
        '\u{e61d}',
        BLUE,
        include_bytes!("../../samples/cpp-cmake-error.ansi"),
    )
}

pub(super) fn ruby_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "Ruby",
        "bundle exec rspec --format documentation --color",
        "Ruby (real)",
        '\u{e791}',
        RED,
        include_bytes!("../../samples/ruby-rspec.ansi"),
    )
}

pub(super) fn lua_real_panel(tools: &DetectedTools) -> Panel {
    sample_panel(
        tools,
        "Lua",
        "busted --verbose spec/",
        "Lua (real)",
        '\u{e620}',
        BLUE,
        include_bytes!("../../samples/lua-busted.ansi"),
    )
}
