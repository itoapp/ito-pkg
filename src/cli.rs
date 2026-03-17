use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build and package a plugin into an .ito file
    Pack {
        /// Path to the plugin directory
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },
    /// Verify an .ito plugin
    Verify {
        /// Path to the .ito file
        path: PathBuf,
    },
    /// Build a static repository from a folder of .ito files
    Repo {
        /// Path to the directory containing .ito files
        #[arg(short, long, default_value = ".")]
        input: PathBuf,
        /// Path to the output directory for the repository
        #[arg(short, long, default_value = "public")]
        output: PathBuf,
        /// Name of the repository
        #[arg(long, default_value = "Ito Repository")]
        name: String,
        /// URL of the repository
        #[arg(long)]
        url: String,
    },
    /// Serve a directory over HTTP (for dev usage)
    Serve {
        /// Path to the directory to serve
        #[arg(long, default_value = "public")]
        path: PathBuf,
        /// Port to bind to
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Scaffold a new plugin
    New {
        /// Name of the plugin project (e.g. 'my-novel-plugin')
        name: String,
        /// Type of the plugin: manga, anime, or novel
        #[arg(short, long, default_value = "manga")]
        plugin_type: String,
    },
}
