// SPDX-License-Identifier: GPLv3
mod command_line;

use crate::command_line::run;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional url(s) to open, space separated
    #[arg(default_values_t = vec!["https://google.com".to_string()])]
    urls: Vec<String>,

    /// window will always be above other windows
    #[arg(long, short)]
    above: bool,

    /// cycle time between site reloads
    ///     if more then one URL was given
    ///     these URL's are cycled after that time
    #[arg(long, short, verbatim_doc_comment, default_value_t = 10)]
    cycle_sec: u64,

    /// open window in fullscreen
    #[arg(long, short, group = "options")]
    fullscreen: bool,

    /// open window maximized
    #[arg(long, short, group = "options")]
    maximized: bool,

    /// monitor number on which the window should open
    ///     This has no effect if you have only one monitor!
    ///     Android / Linux(Wayland): Unsupported
    #[arg(long, verbatim_doc_comment)]
    monitor: Option<usize>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Err(err) = run(args) {
        eprintln!("{:?}", err);
        err.chain().skip(1).for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }

    Ok(())
}
