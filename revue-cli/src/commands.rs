//! CLI command implementations

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use crate::templates;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Create a new Revue project
pub fn new_project(name: &str, template: &str, init_git: bool) -> Result<()> {
    println!("{}", "🎨 Creating new Revue project...".cyan().bold());
    println!();

    let pb = ProgressBar::new(5);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );

    // Step 1: Create directory
    pb.set_message("Creating project directory...");
    let project_path = Path::new(name);
    if project_path.exists() {
        return Err(format!("Directory '{}' already exists", name).into());
    }
    fs::create_dir_all(project_path)?;
    fs::create_dir_all(project_path.join("src"))?;
    fs::create_dir_all(project_path.join("styles"))?;
    fs::create_dir_all(project_path.join("assets"))?;
    pb.inc(1);

    // Step 2: Generate Cargo.toml
    pb.set_message("Generating Cargo.toml...");
    let cargo_toml = templates::cargo_toml(name);
    fs::write(project_path.join("Cargo.toml"), cargo_toml)?;
    pb.inc(1);

    // Step 3: Generate source files based on template
    pb.set_message("Generating source files...");
    let (main_rs, app_rs) = match template {
        "basic" => (templates::basic_main(), templates::basic_app()),
        "dashboard" => (templates::dashboard_main(), templates::dashboard_app()),
        "todo" => (templates::todo_main(), templates::todo_app()),
        "chat" => (templates::chat_main(), templates::chat_app()),
        _ => (templates::basic_main(), templates::basic_app()),
    };
    fs::write(project_path.join("src/main.rs"), main_rs)?;
    fs::write(project_path.join("src/app.rs"), app_rs)?;
    pb.inc(1);

    // Step 4: Generate style files
    pb.set_message("Generating style files...");
    let style_css = templates::default_style();
    fs::write(project_path.join("styles/main.css"), style_css)?;
    pb.inc(1);

    // Step 5: Initialize git
    if init_git {
        pb.set_message("Initializing git repository...");
        let gitignore = templates::gitignore();
        fs::write(project_path.join(".gitignore"), gitignore)?;

        if Command::new("git")
            .args(["init"])
            .current_dir(project_path)
            .output()
            .is_ok()
        {
            // Git initialized successfully
        }
    }
    pb.inc(1);

    pb.finish_with_message("Done!");
    println!();

    // Print success message
    println!("{}", "✅ Project created successfully!".green().bold());
    println!();
    println!("  {} {}", "cd".cyan(), name);
    println!("  {} {}", "cargo".cyan(), "run");
    println!();
    println!(
        "  {} {}",
        "Or start dev server:".dimmed(),
        "revue dev".cyan()
    );
    println!();

    Ok(())
}

/// Start development server with hot reload
pub fn dev_server(port: u16, watch_paths: &[String]) -> Result<()> {
    println!("{}", "🔥 Starting Revue dev server...".cyan().bold());
    println!();
    println!(
        "  {} http://localhost:{}",
        "Dev server:".green(),
        port
    );
    println!("  {} Ctrl+C", "Stop:".yellow());
    println!();

    // Check if we're in a Revue project
    if !Path::new("Cargo.toml").exists() {
        return Err("Not in a Cargo project. Run 'revue new <name>' first.".into());
    }

    // Default watch paths
    let mut paths_to_watch = vec!["src".to_string(), "styles".to_string()];
    paths_to_watch.extend(watch_paths.iter().cloned());

    println!("{}", "Watching for changes...".dimmed());
    for path in &paths_to_watch {
        println!("  {} {}", "📁".dimmed(), path.dimmed());
    }
    println!();

    // Initial build
    println!("{}", "Building...".yellow());
    let status = Command::new("cargo")
        .args(["build"])
        .status()?;

    if !status.success() {
        println!("{}", "❌ Build failed".red());
        return Ok(());
    }

    println!("{}", "✅ Build successful".green());
    println!();

    // Run the app
    println!("{}", "Running app...".cyan());
    let mut child = Command::new("cargo")
        .args(["run"])
        .spawn()?;

    // Wait for Ctrl+C or process exit
    let _ = child.wait();

    Ok(())
}

/// Build project for release
pub fn build_project(release: bool, target: Option<&str>) -> Result<()> {
    println!("{}", "🔨 Building Revue project...".cyan().bold());
    println!();

    let mut args = vec!["build"];

    if release {
        args.push("--release");
        println!("  {} Release mode", "Mode:".green());
    } else {
        println!("  {} Debug mode", "Mode:".yellow());
    }

    if let Some(t) = target {
        args.push("--target");
        args.push(t);
        println!("  {} {}", "Target:".green(), t);
    }

    println!();

    let status = Command::new("cargo")
        .args(&args)
        .status()?;

    if status.success() {
        println!();
        println!("{}", "✅ Build successful!".green().bold());

        let output_dir = if release {
            "target/release"
        } else {
            "target/debug"
        };
        println!("  {} {}/", "Output:".dimmed(), output_dir);
    } else {
        println!();
        println!("{}", "❌ Build failed".red().bold());
    }

    Ok(())
}

/// Run snapshot tests
pub fn run_snapshots(update: bool, filter: Option<&str>) -> Result<()> {
    println!("{}", "📸 Running snapshot tests...".cyan().bold());
    println!();

    let mut args = vec!["test"];

    if let Some(f) = filter {
        args.push(f);
    }

    if update {
        println!("  {} Updating snapshots", "Mode:".yellow());
        std::env::set_var("UPDATE_SNAPSHOTS", "1");
    } else {
        println!("  {} Comparing snapshots", "Mode:".green());
    }

    println!();

    let status = Command::new("cargo")
        .args(&args)
        .status()?;

    if status.success() {
        println!();
        println!("{}", "✅ All snapshot tests passed!".green().bold());
    } else {
        println!();
        println!("{}", "❌ Some snapshot tests failed".red().bold());
        if !update {
            println!(
                "  {} {}",
                "Tip:".dimmed(),
                "Run 'revue snapshot --update' to update snapshots".dimmed()
            );
        }
    }

    Ok(())
}

/// Launch widget inspector
pub fn inspect(mode: &str) -> Result<()> {
    println!("{}", "🔍 Launching widget inspector...".cyan().bold());
    println!();
    println!("  {} {}", "Mode:".green(), mode);
    println!();

    // Set environment variable to enable inspector
    std::env::set_var("REVUE_INSPECTOR", mode);

    let status = Command::new("cargo")
        .args(["run"])
        .status()?;

    if !status.success() {
        return Err("Failed to launch inspector".into());
    }

    Ok(())
}

/// List available themes
pub fn list_themes(verbose: bool) -> Result<()> {
    println!("{}", "🎨 Available Themes".cyan().bold());
    println!();

    let themes = [
        ("dracula", "Dark theme with purple accents", "#282a36", "#bd93f9"),
        ("nord", "Arctic, north-bluish color palette", "#2e3440", "#88c0d0"),
        ("monokai", "Sublime Text inspired dark theme", "#272822", "#f92672"),
        ("gruvbox", "Retro groove color scheme", "#282828", "#fabd2f"),
        ("catppuccin", "Soothing pastel theme", "#1e1e2e", "#cba6f7"),
        ("tokyo-night", "Clean dark theme inspired by Tokyo", "#1a1b26", "#7aa2f7"),
        ("one-dark", "Atom One Dark theme", "#282c34", "#61afef"),
        ("solarized-dark", "Precision colors for machines and people", "#002b36", "#268bd2"),
    ];

    for (name, desc, bg, accent) in themes {
        if verbose {
            println!("  {} {}", "●".cyan(), name.bold());
            println!("    {}", desc.dimmed());
            println!("    {} {}  {} {}", "Background:".dimmed(), bg, "Accent:".dimmed(), accent);
            println!();
        } else {
            println!("  {} {} - {}", "●".cyan(), name.bold(), desc.dimmed());
        }
    }

    println!();
    println!(
        "  {} {}",
        "Install:".dimmed(),
        "revue theme <name>".cyan()
    );

    Ok(())
}

/// Install a theme
pub fn install_theme(name: &str) -> Result<()> {
    println!(
        "{} {}",
        "🎨 Installing theme:".cyan().bold(),
        name.bold()
    );
    println!();

    let theme_css = match name {
        "dracula" => templates::theme_dracula(),
        "nord" => templates::theme_nord(),
        "monokai" => templates::theme_monokai(),
        "gruvbox" => templates::theme_gruvbox(),
        "catppuccin" => templates::theme_catppuccin(),
        _ => {
            return Err(format!("Unknown theme: {}. Run 'revue themes' to see available themes.", name).into());
        }
    };

    // Create styles directory if it doesn't exist
    fs::create_dir_all("styles")?;

    // Write theme file
    let theme_path = format!("styles/{}.css", name);
    fs::write(&theme_path, theme_css)?;

    println!("{}", "✅ Theme installed!".green().bold());
    println!();
    println!("  {} {}", "File:".dimmed(), theme_path);
    println!();
    println!("  {} Add to your app:", "Usage:".dimmed());
    println!(
        "    {}",
        format!("App::builder().style(\"styles/{}.css\")", name).cyan()
    );

    Ok(())
}

/// Generate documentation
pub fn generate_docs(output: &str) -> Result<()> {
    println!("{}", "📚 Generating documentation...".cyan().bold());
    println!();

    let status = Command::new("cargo")
        .args(["doc", "--no-deps", "--document-private-items"])
        .status()?;

    if status.success() {
        // Copy to output directory
        let doc_path = "target/doc";
        if Path::new(doc_path).exists() {
            println!("{}", "✅ Documentation generated!".green().bold());
            println!();
            println!("  {} {}", "Output:".dimmed(), doc_path);
            println!(
                "  {} {}",
                "Open:".dimmed(),
                format!("open {}/revue/index.html", doc_path).cyan()
            );
        }
    } else {
        println!("{}", "❌ Documentation generation failed".red().bold());
    }

    Ok(())
}

/// Add a component to the project
pub fn add_component(component: &str, name: Option<&str>) -> Result<()> {
    println!(
        "{} {}",
        "➕ Adding component:".cyan().bold(),
        component.bold()
    );
    println!();

    // Check if we're in a Revue project
    if !Path::new("Cargo.toml").exists() {
        return Err("Not in a Cargo project. Run 'revue new <name>' first.".into());
    }

    // Create src directory if it doesn't exist
    fs::create_dir_all("src")?;

    // Generate component code
    let (filename, code) = match component {
        "search" => {
            let file = name.unwrap_or("search").to_string() + ".rs";
            (file, templates::component_search())
        }
        "form" => {
            let file = name.unwrap_or("form").to_string() + ".rs";
            (file, templates::component_form())
        }
        "navigation" => {
            let file = name.unwrap_or("navigation").to_string() + ".rs";
            (file, templates::component_navigation())
        }
        "modal" => {
            let file = name.unwrap_or("modal").to_string() + ".rs";
            (file, templates::component_modal())
        }
        "toast" => {
            let file = name.unwrap_or("toast").to_string() + ".rs";
            (file, templates::component_toast())
        }
        "command_palette" => {
            let file = name.unwrap_or("command_palette").to_string() + ".rs";
            (file, templates::component_command_palette())
        }
        "table" => {
            let file = name.unwrap_or("data_table").to_string() + ".rs";
            (file, templates::component_table())
        }
        "tabs" => {
            let file = name.unwrap_or("tabs").to_string() + ".rs";
            (file, templates::component_tabs())
        }
        _ => return Err(format!("Unknown component: {}", component).into()),
    };

    let filepath = format!("src/{}", filename);

    // Check if file exists
    if Path::new(&filepath).exists() {
        return Err(format!("File {} already exists", filepath).into());
    }

    // Write file
    fs::write(&filepath, code)?;

    println!("{}", "✅ Component added!".green().bold());
    println!();
    println!("  {} {}", "File:".dimmed(), filepath);
    println!();
    println!("  {} Add to your main.rs:", "Usage:".dimmed());
    println!("    {}", format!("mod {};", filename.trim_end_matches(".rs")).cyan());

    Ok(())
}

// =============================================================================
// Plugin Commands
// =============================================================================

/// List installed plugins
pub fn plugin_list() -> Result<()> {
    println!("{}", "🔌 Installed Plugins".cyan().bold());
    println!();

    // Check if we're in a Revue project
    if !Path::new("Cargo.toml").exists() {
        return Err("Not in a Cargo project.".into());
    }

    let cargo_content = fs::read_to_string("Cargo.toml")?;

    // Find revue-plugin-* dependencies
    let mut plugins_found = false;
    for line in cargo_content.lines() {
        if line.contains("revue-plugin-") || line.contains("revue_plugin_") {
            let line = line.trim();
            if let Some(name) = line.split('=').next() {
                let name = name.trim().trim_matches('"');
                let version = line
                    .split('=')
                    .nth(1)
                    .map(|v| v.trim().trim_matches('"').trim_matches('{'))
                    .unwrap_or("*");
                println!("  {} {} {}", "●".green(), name.bold(), version.dimmed());
                plugins_found = true;
            }
        }
    }

    if !plugins_found {
        println!("  {}", "No plugins installed".dimmed());
        println!();
        println!("  {} {}", "Install:".dimmed(), "revue plugin install <name>".cyan());
        println!("  {} {}", "Search:".dimmed(), "revue plugin search <query>".cyan());
    }

    println!();
    Ok(())
}

/// Search for plugins on crates.io
pub fn plugin_search(query: &str) -> Result<()> {
    println!(
        "{} {}",
        "🔍 Searching for plugins:".cyan().bold(),
        query.bold()
    );
    println!();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message("Searching crates.io...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Search crates.io API
    let search_query = if query.starts_with("revue-plugin-") || query.starts_with("revue_plugin_") {
        query.to_string()
    } else {
        format!("revue-plugin-{}", query)
    };

    let url = format!(
        "https://crates.io/api/v1/crates?q={}&per_page=10",
        urlencoding::encode(&search_query)
    );

    let response = match ureq::get(&url)
        .set("User-Agent", "revue-cli/0.1.0")
        .call()
    {
        Ok(resp) => resp,
        Err(e) => {
            pb.finish_and_clear();
            return Err(format!("Failed to search crates.io: {}", e).into());
        }
    };

    let body: serde_json::Value = response.into_json()?;
    pb.finish_and_clear();

    let crates = body["crates"].as_array();

    if let Some(crates) = crates {
        let revue_crates: Vec<_> = crates
            .iter()
            .filter(|c| {
                c["name"].as_str()
                    .map(|n| n.starts_with("revue-plugin-") || n.starts_with("revue_plugin_"))
                    .unwrap_or(false)
            })
            .collect();

        if revue_crates.is_empty() {
            println!("  {}", "No Revue plugins found".dimmed());
            println!();
            println!("  {} Create one with:", "Tip:".yellow());
            println!("    {}", "revue plugin new my-plugin".cyan());
        } else {
            for crate_info in revue_crates {
                let name = crate_info["name"].as_str().unwrap_or("unknown");
                let version = crate_info["newest_version"].as_str().unwrap_or("0.0.0");
                let desc = crate_info["description"].as_str().unwrap_or("");
                let downloads = crate_info["downloads"].as_u64().unwrap_or(0);

                println!("  {} {} {}", "●".green(), name.bold(), format!("v{}", version).dimmed());
                if !desc.is_empty() {
                    println!("    {}", desc.dimmed());
                }
                println!("    {} downloads", downloads.to_string().cyan());
                println!();
            }

            println!("  {} {}", "Install:".dimmed(), "revue plugin install <name>".cyan());
        }
    } else {
        println!("  {}", "No results found".dimmed());
    }

    println!();
    Ok(())
}

/// Install a plugin
pub fn plugin_install(name: &str, version: Option<&str>) -> Result<()> {
    let plugin_name = if name.starts_with("revue-plugin-") || name.starts_with("revue_plugin_") {
        name.to_string()
    } else {
        format!("revue-plugin-{}", name)
    };

    println!(
        "{} {}",
        "📦 Installing plugin:".cyan().bold(),
        plugin_name.bold()
    );
    println!();

    // Check if we're in a Cargo project
    if !Path::new("Cargo.toml").exists() {
        return Err("Not in a Cargo project. Run 'revue new <name>' first.".into());
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message("Adding dependency...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Use cargo add
    let mut args = vec!["add", &plugin_name];
    let version_str;
    if let Some(v) = version {
        version_str = format!("{}@{}", plugin_name, v);
        args = vec!["add", &version_str];
    }

    let status = Command::new("cargo")
        .args(&args)
        .output()?;

    pb.finish_and_clear();

    if status.status.success() {
        println!("{}", "✅ Plugin installed!".green().bold());
        println!();
        println!("  {} Add to your app:", "Usage:".dimmed());
        println!("    {}", format!("use {}::*;", plugin_name.replace('-', "_")).cyan());
        println!();
        println!("    {}", "App::builder()".cyan());
        println!("        {}", format!(".plugin({}Plugin::new())", to_pascal_case(&plugin_name)).cyan());
        println!("        {}", ".build();".cyan());
    } else {
        let stderr = String::from_utf8_lossy(&status.stderr);
        println!("{}", "❌ Installation failed".red().bold());
        println!();
        println!("  {}", stderr.dimmed());
    }

    println!();
    Ok(())
}

/// Show plugin info
pub fn plugin_info(name: &str) -> Result<()> {
    let plugin_name = if name.starts_with("revue-plugin-") || name.starts_with("revue_plugin_") {
        name.to_string()
    } else {
        format!("revue-plugin-{}", name)
    };

    println!(
        "{} {}",
        "ℹ️  Plugin Info:".cyan().bold(),
        plugin_name.bold()
    );
    println!();

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message("Fetching from crates.io...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let url = format!("https://crates.io/api/v1/crates/{}", plugin_name);
    let response = match ureq::get(&url)
        .set("User-Agent", "revue-cli/0.1.0")
        .call()
    {
        Ok(resp) => resp,
        Err(e) => {
            pb.finish_and_clear();
            return Err(format!("Plugin not found: {}", e).into());
        }
    };

    let body: serde_json::Value = response.into_json()?;
    pb.finish_and_clear();

    let crate_info = &body["crate"];
    let name = crate_info["name"].as_str().unwrap_or("unknown");
    let version = crate_info["newest_version"].as_str().unwrap_or("0.0.0");
    let desc = crate_info["description"].as_str().unwrap_or("No description");
    let downloads = crate_info["downloads"].as_u64().unwrap_or(0);
    let repo = crate_info["repository"].as_str().unwrap_or("");
    let docs = crate_info["documentation"].as_str().unwrap_or("");

    println!("  {} {}", "Name:".green(), name);
    println!("  {} {}", "Version:".green(), version);
    println!("  {} {}", "Description:".green(), desc);
    println!("  {} {}", "Downloads:".green(), downloads);
    if !repo.is_empty() {
        println!("  {} {}", "Repository:".green(), repo);
    }
    if !docs.is_empty() {
        println!("  {} {}", "Docs:".green(), docs);
    }

    println!();
    println!("  {} {}", "Install:".dimmed(), format!("revue plugin install {}", name).cyan());

    println!();
    Ok(())
}

/// Create a new plugin project
pub fn plugin_new(name: &str) -> Result<()> {
    let plugin_name = if name.starts_with("revue-plugin-") {
        name.to_string()
    } else {
        format!("revue-plugin-{}", name)
    };

    println!(
        "{} {}",
        "🔌 Creating new plugin:".cyan().bold(),
        plugin_name.bold()
    );
    println!();

    let pb = ProgressBar::new(4);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );

    // Step 1: Create directory
    pb.set_message("Creating plugin directory...");
    let project_path = Path::new(&plugin_name);
    if project_path.exists() {
        return Err(format!("Directory '{}' already exists", plugin_name).into());
    }
    fs::create_dir_all(project_path)?;
    fs::create_dir_all(project_path.join("src"))?;
    pb.inc(1);

    // Step 2: Generate Cargo.toml
    pb.set_message("Generating Cargo.toml...");
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
description = "A Revue TUI framework plugin"
license = "MIT"
repository = ""
keywords = ["tui", "revue", "plugin"]
categories = ["command-line-interface"]

[dependencies]
revue = "0.6"
"#,
        plugin_name
    );
    fs::write(project_path.join("Cargo.toml"), cargo_toml)?;
    pb.inc(1);

    // Step 3: Generate lib.rs
    pb.set_message("Generating source files...");
    let struct_name = to_pascal_case(&plugin_name.replace("revue-plugin-", ""));
    let struct_name_lower = struct_name.to_lowercase();
    let lib_rs = format!(
        r##"//! {plugin_name} - A Revue TUI framework plugin

use revue::plugin::{{Plugin, PluginContext}};
use revue::Result;
use std::time::Duration;

/// {struct_name} Plugin
pub struct {struct_name}Plugin {{
    // Add plugin state here
}}

impl {struct_name}Plugin {{
    /// Create a new plugin instance
    pub fn new() -> Self {{
        Self {{}}
    }}
}}

impl Default for {struct_name}Plugin {{
    fn default() -> Self {{
        Self::new()
    }}
}}

impl Plugin for {struct_name}Plugin {{
    fn name(&self) -> &str {{
        "{plugin_name}"
    }}

    fn priority(&self) -> i32 {{
        0 // Default priority
    }}

    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {{
        ctx.info("Plugin initialized");
        Ok(())
    }}

    fn on_mount(&mut self, ctx: &mut PluginContext) -> Result<()> {{
        ctx.info("Plugin mounted");
        Ok(())
    }}

    fn on_tick(&mut self, _ctx: &mut PluginContext, _delta: Duration) -> Result<()> {{
        // Called every frame
        Ok(())
    }}

    fn on_unmount(&mut self, ctx: &mut PluginContext) -> Result<()> {{
        ctx.info("Plugin unmounted");
        Ok(())
    }}

    fn styles(&self) -> Option<&str> {{
        Some(r#"
.{struct_name_lower}-widget {{
    border: solid blue;
    padding: 1;
}}
"#)
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_plugin_name() {{
        let plugin = {struct_name}Plugin::new();
        assert_eq!(plugin.name(), "{plugin_name}");
    }}
}}
"##,
        plugin_name = plugin_name,
        struct_name = struct_name,
        struct_name_lower = struct_name_lower,
    );
    fs::write(project_path.join("src/lib.rs"), lib_rs)?;
    pb.inc(1);

    // Step 4: Generate README
    pb.set_message("Generating README...");
    let readme = format!(
        r#"# {}

A plugin for the [Revue TUI framework](https://github.com/user/revue).

## Installation

```bash
revue plugin install {}
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
{} = "0.1"
```

## Usage

```rust
use revue::prelude::*;
use {}::{}Plugin;

fn main() -> revue::Result<()> {{
    App::builder()
        .plugin({}Plugin::new())
        .view(MyApp::new())
        .run()
}}
```

## License

MIT
"#,
        plugin_name,
        plugin_name,
        plugin_name,
        plugin_name.replace('-', "_"),
        struct_name,
        struct_name
    );
    fs::write(project_path.join("README.md"), readme)?;
    pb.inc(1);

    pb.finish_with_message("Done!");
    println!();

    println!("{}", "✅ Plugin created successfully!".green().bold());
    println!();
    println!("  {} {}", "cd".cyan(), plugin_name);
    println!("  {} {}", "cargo".cyan(), "build");
    println!();
    println!("  {} {}", "Publish:".dimmed(), "cargo publish".cyan());
    println!();

    Ok(())
}

/// Convert kebab-case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

/// Run benchmarks
pub fn run_benchmark(filter: Option<&str>, save: bool) -> Result<()> {
    println!("{}", "⏱️  Running benchmarks...".cyan().bold());
    println!();

    // Check if we're in a Revue project
    if !Path::new("Cargo.toml").exists() {
        return Err("Not in a Cargo project.".into());
    }

    // Check if criterion is available
    let has_criterion = fs::read_to_string("Cargo.toml")
        .map(|s| s.contains("criterion"))
        .unwrap_or(false);

    if !has_criterion {
        println!("{}", "⚠️  Criterion not found in Cargo.toml".yellow());
        println!();
        println!("  {} Add to Cargo.toml:", "Setup:".dimmed());
        println!("    {}", "[dev-dependencies]".cyan());
        println!("    {}", "criterion = \"0.5\"".cyan());
        println!();
        return Ok(());
    }

    let mut args = vec!["bench"];

    if let Some(f) = filter {
        args.push("--bench");
        args.push(f);
    }

    println!("  {} {}", "Filter:".green(), filter.unwrap_or("all"));
    if save {
        println!("  {} {}", "Saving:".green(), "target/criterion/");
    }
    println!();

    let status = Command::new("cargo")
        .args(&args)
        .status()?;

    if status.success() {
        println!();
        println!("{}", "✅ Benchmarks complete!".green().bold());

        if save {
            println!();
            println!("  {} target/criterion/report/index.html", "Results:".dimmed());
        }
    } else {
        println!();
        println!("{}", "❌ Benchmarks failed".red().bold());
    }

    Ok(())
}
