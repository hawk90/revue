//! Revue CLI - Development tools for Revue TUI framework

use clap::{Parser, Subcommand};
use colored::Colorize;
use revue_cli::{commands, dev};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "revue")]
#[command(author = "Revue Team")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool for Revue TUI framework", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Revue project
    New {
        /// Project name
        name: String,

        /// Template to use
        #[arg(short, long, default_value = "basic")]
        template: String,

        /// Skip git initialization
        #[arg(long)]
        no_git: bool,
    },

    /// Run the app, rebuilding and restarting it when the source changes
    ///
    /// Watches src/, styles/, Cargo.toml and build.rs. A Rust change stops
    /// the app, rebuilds it and starts it again; stylesheets reload in place
    /// when the app's revue supports it (REVUE_HOT_RELOAD).
    Dev {
        /// Also watch this file or directory (repeatable)
        #[arg(short, long, value_name = "PATH")]
        watch: Vec<PathBuf>,

        /// Binary to run, when the package has several
        #[arg(long, value_name = "NAME")]
        bin: Option<String>,
    },

    /// Build the project for release
    Build {
        /// Build in release mode
        #[arg(short, long)]
        release: bool,

        /// Target platform
        #[arg(short, long)]
        target: Option<String>,
    },

    /// Run snapshot tests
    Snapshot {
        /// Update snapshots instead of comparing
        #[arg(short, long)]
        update: bool,

        /// Filter tests by name
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Launch widget inspector
    Inspect {
        /// Inspector mode
        #[arg(short, long, default_value = "overlay")]
        mode: String,
    },

    /// List available themes
    Themes {
        /// Show detailed info
        #[arg(short, long)]
        verbose: bool,
    },

    /// Install a theme
    Theme {
        /// Theme name to install
        name: String,
    },

    /// Generate documentation
    Docs {
        /// Output directory
        #[arg(short, long, default_value = "docs")]
        output: String,
    },

    /// Add a component or pattern to your project
    Add {
        /// Component type to add
        #[arg(value_enum)]
        component: ComponentType,

        /// Custom name for the component
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Run benchmarks
    Benchmark {
        /// Specific benchmark to run
        #[arg(short, long)]
        filter: Option<String>,

        /// Save results to file
        #[arg(short, long)]
        save: bool,
    },

    /// Manage plugins
    Plugin {
        #[command(subcommand)]
        action: PluginAction,
    },
}

#[derive(Subcommand)]
enum PluginAction {
    /// List installed plugins
    List,

    /// Search for plugins on crates.io
    Search {
        /// Search query
        query: String,
    },

    /// Install a plugin
    Install {
        /// Plugin name (e.g., revue-plugin-git)
        name: String,

        /// Specific version
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Show plugin info
    Info {
        /// Plugin name
        name: String,
    },

    /// Create a new plugin project
    New {
        /// Plugin name
        name: String,
    },
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum ComponentType {
    /// Search component with filter state
    Search,
    /// Form with validation
    Form,
    /// Navigation with history
    Navigation,
    /// Modal dialog
    Modal,
    /// Toast notifications
    Toast,
    /// Command palette
    CommandPalette,
    /// Data table
    Table,
    /// Tab navigation
    Tabs,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New {
            name,
            template,
            no_git,
        } => commands::new_project(&name, &template, !no_git),
        Commands::Dev { watch, bin } => dev::run(&dev::DevOptions { watch, bin }),
        Commands::Build { release, target } => commands::build_project(release, target.as_deref()),
        Commands::Snapshot { update, filter } => commands::run_snapshots(update, filter.as_deref()),
        Commands::Inspect { mode } => commands::inspect(&mode),
        Commands::Themes { verbose } => commands::list_themes(verbose),
        Commands::Theme { name } => commands::install_theme(&name),
        Commands::Docs { output } => commands::generate_docs(&output),
        Commands::Add { component, name } => {
            let comp_name = match component {
                ComponentType::Search => "search",
                ComponentType::Form => "form",
                ComponentType::Navigation => "navigation",
                ComponentType::Modal => "modal",
                ComponentType::Toast => "toast",
                ComponentType::CommandPalette => "command_palette",
                ComponentType::Table => "table",
                ComponentType::Tabs => "tabs",
            };
            commands::add_component(comp_name, name.as_deref())
        }
        Commands::Benchmark { filter, save } => commands::run_benchmark(filter.as_deref(), save),
        Commands::Plugin { action } => match action {
            PluginAction::List => commands::plugin_list(),
            PluginAction::Search { query } => commands::plugin_search(&query),
            PluginAction::Install { name, version } => {
                commands::plugin_install(&name, version.as_deref())
            }
            PluginAction::Info { name } => commands::plugin_info(&name),
            PluginAction::New { name } => commands::plugin_new(&name),
        },
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("revue").chain(args.iter().copied()))
    }

    #[test]
    fn dev_takes_no_flags_by_default() {
        let Commands::Dev { watch, bin } = parse(&["dev"]).unwrap().command else {
            panic!("expected dev");
        };
        assert!(watch.is_empty());
        assert_eq!(bin, None);
    }

    #[test]
    fn dev_watch_is_a_repeatable_path() {
        let cli = parse(&[
            "dev",
            "--watch",
            "config",
            "-w",
            "assets/data.json",
            "--bin",
            "demo",
        ]);
        let Commands::Dev { watch, bin } = cli.unwrap().command else {
            panic!("expected dev");
        };
        assert_eq!(
            watch,
            [PathBuf::from("config"), PathBuf::from("assets/data.json")]
        );
        assert_eq!(bin.as_deref(), Some("demo"));
    }

    #[test]
    fn dev_watch_needs_a_path() {
        assert!(parse(&["dev", "--watch"]).is_err());
    }

    #[test]
    fn dev_has_no_port() {
        assert!(parse(&["dev", "--port", "3000"]).is_err());
        assert!(parse(&["dev", "-p", "3000"]).is_err());
    }
}
