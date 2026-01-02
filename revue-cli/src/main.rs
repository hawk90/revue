//! Revue CLI - Development tools for Revue TUI framework

mod commands;
mod templates;

use clap::{Parser, Subcommand};
use colored::Colorize;

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

    /// Start development server with hot reload
    Dev {
        /// Port for the dev server
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Watch additional paths
        #[arg(short, long)]
        watch: Vec<String>,
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
        Commands::New { name, template, no_git } => {
            commands::new_project(&name, &template, !no_git)
        }
        Commands::Dev { port, watch } => {
            commands::dev_server(port, &watch)
        }
        Commands::Build { release, target } => {
            commands::build_project(release, target.as_deref())
        }
        Commands::Snapshot { update, filter } => {
            commands::run_snapshots(update, filter.as_deref())
        }
        Commands::Inspect { mode } => {
            commands::inspect(&mode)
        }
        Commands::Themes { verbose } => {
            commands::list_themes(verbose)
        }
        Commands::Theme { name } => {
            commands::install_theme(&name)
        }
        Commands::Docs { output } => {
            commands::generate_docs(&output)
        }
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
        Commands::Benchmark { filter, save } => {
            commands::run_benchmark(filter.as_deref(), save)
        }
        Commands::Plugin { action } => {
            match action {
                PluginAction::List => commands::plugin_list(),
                PluginAction::Search { query } => commands::plugin_search(&query),
                PluginAction::Install { name, version } => {
                    commands::plugin_install(&name, version.as_deref())
                }
                PluginAction::Info { name } => commands::plugin_info(&name),
                PluginAction::New { name } => commands::plugin_new(&name),
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
