use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use ansi_to_tui::IntoText;
use color_eyre::Result;
use ratatui::text::Text;
use tempfile::TempDir;

#[derive(Debug, Clone)]
pub enum PromptEngine {
    Starship,
    OhMyPosh,
    ShellFramework,
    RawShell(String),
    Fallback,
}

#[derive(Debug, Clone)]
pub struct DetectedTools {
    #[allow(dead_code)]
    pub prompt_engine: PromptEngine,
    pub prompts: Vec<(String, Text<'static>)>,
}

struct ProjectSpec {
    name: &'static str,
    dir_name: &'static str,
    files: Vec<(&'static str, &'static str)>,
}

fn project_specs() -> Vec<ProjectSpec> {
    vec![
        ProjectSpec {
            name: "Python",
            dir_name: "my-python-app",
            files: vec![
                ("pyproject.toml", "[project]\nname = \"my-python-app\"\nversion = \"0.1.0\"\nrequires-python = \">=3.12\"\n\n[build-system]\nrequires = [\"hatchling\"]\nbuild-backend = \"hatchling.build\"\n"),
                (".python-version", "3.12.0\n"),
                ("src/main.py", "def main():\n    print(\"Hello, world!\")\n\nif __name__ == \"__main__\":\n    main()\n"),
            ],
        },
        ProjectSpec {
            name: "Node.js",
            dir_name: "my-node-app",
            files: vec![
                ("package.json", "{\n  \"name\": \"my-node-app\",\n  \"version\": \"1.0.0\",\n  \"type\": \"module\",\n  \"scripts\": {\n    \"test\": \"jest\"\n  }\n}\n"),
                ("node_modules/.package-lock.json", "{}\n"),
                ("src/index.ts", "export function greet(name: string): string {\n  return `Hello, ${name}!`;\n}\n"),
            ],
        },
        ProjectSpec {
            name: "Rust",
            dir_name: "my-rust-app",
            files: vec![
                ("Cargo.toml", "[package]\nname = \"my-rust-app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
                ("src/main.rs", "fn main() {\n    println!(\"Hello, world!\");\n}\n"),
            ],
        },
        ProjectSpec {
            name: "Go",
            dir_name: "my-go-app",
            files: vec![
                ("go.mod", "module github.com/user/my-go-app\n\ngo 1.22\n"),
                ("main.go", "package main\n\nimport \"fmt\"\n\nfunc main() {\n\tfmt.Println(\"Hello, world!\")\n}\n"),
            ],
        },
        ProjectSpec {
            name: "C/C++",
            dir_name: "my-cpp-app",
            files: vec![
                ("CMakeLists.txt", "cmake_minimum_required(VERSION 3.20)\nproject(my-cpp-app LANGUAGES CXX)\nadd_executable(app src/main.cpp)\n"),
                ("src/main.cpp", "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, world!\" << std::endl;\n    return 0;\n}\n"),
            ],
        },
        ProjectSpec {
            name: "Zig",
            dir_name: "my-zig-app",
            files: vec![
                ("build.zig", "const std = @import(\"std\");\n\npub fn build(b: *std.Build) void {\n    const exe = b.addExecutable(.{\n        .name = \"my-zig-app\",\n        .root_source_file = b.path(\"src/main.zig\"),\n        .target = b.standardTargetOptions(.{}),\n    });\n    b.installArtifact(exe);\n}\n"),
                ("src/main.zig", "const std = @import(\"std\");\n\npub fn main() !void {\n    const stdout = std.io.getStdOut().writer();\n    try stdout.print(\"Hello, world!\\n\", .{});\n}\n"),
            ],
        },
        ProjectSpec {
            name: "Java",
            dir_name: "my-java-app",
            files: vec![
                ("build.gradle", "plugins {\n    id 'application'\n}\n\napplication {\n    mainClass = 'App'\n}\n"),
                ("src/main/java/App.java", "public class App {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, world!\");\n    }\n}\n"),
            ],
        },
        ProjectSpec {
            name: "C#",
            dir_name: "my-csharp-app",
            files: vec![
                ("App.csproj", "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net8.0</TargetFramework>\n  </PropertyGroup>\n</Project>\n"),
                ("Program.cs", "Console.WriteLine(\"Hello, World!\");\n"),
            ],
        },
        ProjectSpec {
            name: "Ruby",
            dir_name: "my-ruby-app",
            files: vec![
                ("Gemfile", "source \"https://rubygems.org\"\n\ngem \"rspec\"\n"),
                ("lib/app.rb", "class App\n  def greet(name)\n    \"Hello, #{name}!\"\n  end\nend\n"),
            ],
        },
    ]
}

fn detect_prompt_engine() -> PromptEngine {
    // Tier 1: standalone engines
    if which::which("starship").is_ok() {
        return PromptEngine::Starship;
    }
    if which::which("oh-my-posh").is_ok() {
        return PromptEngine::OhMyPosh;
    }

    // Tier 2: shell frameworks
    let home = dirs_home();
    if home.join(".p10k.zsh").exists() {
        return PromptEngine::ShellFramework;
    }
    if std::env::var("ZSH").is_ok() || home.join(".oh-my-zsh").exists() {
        return PromptEngine::ShellFramework;
    }

    // Tier 3: raw shell
    if let Ok(shell) = std::env::var("SHELL") {
        let shell_name = Path::new(&shell)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("bash")
            .to_string();
        return PromptEngine::RawShell(shell_name);
    }

    PromptEngine::Fallback
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

fn capture_prompt(engine: &PromptEngine, dir: &Path, width: u16) -> Option<Vec<u8>> {
    match engine {
        PromptEngine::Starship => {
            let output = Command::new("starship")
                .arg("prompt")
                .arg("--path")
                .arg(dir)
                .arg("-w")
                .arg(width.to_string())
                .env("STARSHIP_SHELL", "")
                .output()
                .ok()?;
            if output.status.success() && !output.stdout.is_empty() {
                Some(output.stdout)
            } else {
                None
            }
        }
        PromptEngine::OhMyPosh => {
            let output = Command::new("oh-my-posh")
                .arg("print")
                .arg("primary")
                .arg("--pwd")
                .arg(dir)
                .arg("--shell")
                .arg("zsh")
                .output()
                .ok()?;
            if output.status.success() && !output.stdout.is_empty() {
                Some(output.stdout)
            } else {
                None
            }
        }
        PromptEngine::ShellFramework => {
            let output = Command::new("zsh")
                .arg("-i")
                .arg("-c")
                .arg(format!(
                    "cd {} && print -P \"$PS1\"",
                    dir.display()
                ))
                .stderr(std::process::Stdio::null())
                .output()
                .ok()?;
            if output.status.success() && !output.stdout.is_empty() {
                Some(output.stdout)
            } else {
                None
            }
        }
        PromptEngine::RawShell(shell_name) => {
            let output = match shell_name.as_str() {
                "zsh" => Command::new("zsh")
                    .arg("-i")
                    .arg("-c")
                    .arg(format!("cd {} && print -P \"$PS1\"", dir.display()))
                    .stderr(std::process::Stdio::null())
                    .output()
                    .ok()?,
                "fish" => Command::new("fish")
                    .arg("-c")
                    .arg(format!("cd {} && fish_prompt", dir.display()))
                    .stderr(std::process::Stdio::null())
                    .output()
                    .ok()?,
                _ => Command::new("bash")
                    .arg("-i")
                    .arg("-c")
                    .arg(format!(
                        "cd {} && printf \"%s\" \"${{PS1@P}}\"",
                        dir.display()
                    ))
                    .stderr(std::process::Stdio::null())
                    .output()
                    .ok()?,
            };
            if output.status.success() && !output.stdout.is_empty() {
                Some(output.stdout)
            } else {
                None
            }
        }
        PromptEngine::Fallback => None,
    }
}

fn ansi_bytes_to_text(bytes: &[u8]) -> Text<'static> {
    bytes
        .into_text()
        .unwrap_or_else(|_| Text::raw(String::from_utf8_lossy(bytes).into_owned()))
}

fn fallback_prompt(dir_name: &str) -> Text<'static> {
    use ratatui::style::{Color, Style};
    use ratatui::text::{Line, Span};

    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
    let host = hostname();

    Line::from(vec![
        Span::styled(
            format!("{user}@{host}"),
            Style::default().fg(Color::Indexed(2)),
        ),
        Span::styled(":", Style::default().fg(Color::Indexed(7))),
        Span::styled(
            format!("~/{dir_name}"),
            Style::default().fg(Color::Indexed(4)),
        ),
        Span::styled("$ ", Style::default().fg(Color::Indexed(7))),
    ])
    .into()
}

fn hostname() -> String {
    Command::new("hostname")
        .arg("-s")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "host".into())
}

fn init_git_repo(dir: &Path) -> Result<()> {
    Command::new("git")
        .args(["init", "-b", "main"])
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    Command::new("git")
        .args(["add", "-A"])
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    Command::new("git")
        .args([
            "-c", "user.name=Dev",
            "-c", "user.email=dev@example.com",
            "-c", "commit.gpgsign=false",
            "commit", "-m", "initial commit",
        ])
        .current_dir(dir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;

    // Dirty state for realistic prompts
    fs::write(dir.join("TODO.md"), "# TODO\n- [ ] Add tests\n")?;

    Ok(())
}

pub fn offline_tools() -> DetectedTools {
    let specs = project_specs();
    let prompts = specs
        .iter()
        .map(|spec| (spec.name.to_string(), fallback_prompt(spec.dir_name)))
        .collect();

    DetectedTools {
        prompt_engine: PromptEngine::Fallback,
        prompts,
    }
}

pub fn setup(offline: bool) -> Result<(TempDir, DetectedTools)> {
    let scaffold_dir = tempfile::Builder::new()
        .prefix("diorama-")
        .tempdir()?;

    if offline {
        let tools = offline_tools();
        return Ok((scaffold_dir, tools));
    }

    let engine = detect_prompt_engine();
    let specs = project_specs();
    let term_width = crossterm::terminal::size().map(|(w, _)| w).unwrap_or(80);

    let mut prompts = Vec::new();

    for spec in &specs {
        let project_dir = scaffold_dir.path().join(spec.dir_name);
        fs::create_dir_all(&project_dir)?;

        for (rel_path, content) in &spec.files {
            let full_path = project_dir.join(rel_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&full_path, content)?;
        }

        init_git_repo(&project_dir)?;

        let prompt_text =
            capture_prompt(&engine, &project_dir, term_width)
                .map(|bytes| ansi_bytes_to_text(&bytes))
                .unwrap_or_else(|| fallback_prompt(spec.dir_name));

        prompts.push((spec.name.to_string(), prompt_text));
    }

    let tools = DetectedTools {
        prompt_engine: engine,
        prompts,
    };

    Ok((scaffold_dir, tools))
}
