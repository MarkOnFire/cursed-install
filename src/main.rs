mod build_logs;
mod cli;
mod config;
mod creepy_messages;
mod deno_logs;
mod escalation;
mod installer;
mod kernel_logs;
mod log_generator;
mod messages;
mod scanner;
mod stages;
mod ui;

use clap::Parser;
use cli::Cli;
use colored::*;
use escalation::EscalationEngine;
use installer::Installer;
use rand::seq::SliceRandom;
use std::io;
use std::sync::Arc;

fn main() {
    let cli = Cli::parse();

    let scan = if cli.normal_mode {
        None
    } else {
        Some(Arc::new(scanner::scan()))
    };

    if let Err(e) = run_installer(&cli, scan.clone()) {
        handle_error(e, scan.as_deref());
    }
}

fn run_installer(cli: &Cli, scan: Option<Arc<scanner::ScanResult>>) -> io::Result<()> {
    let mut stages = cli.get_stages();

    let mut rng = rand::thread_rng();
    stages.shuffle(&mut rng);

    let mut installer = Installer::new(stages, scan);
    installer.run()
}

fn handle_error(e: io::Error, scan: Option<&scanner::ScanResult>) {
    if e.kind() == io::ErrorKind::Interrupted {
        // Check if we should show a creepy exit message
        if let Some(scan_data) = scan {
            let mut engine = EscalationEngine::new(scan_data);
            if let Some(msg) = engine.select_exit_message() {
                println!(
                    "\n\n{}",
                    "═══════════════════════════════════════".bright_red()
                );
                println!("{}", msg.bright_red());
                println!(
                    "{}",
                    "═══════════════════════════════════════".bright_red()
                );
                return;
            }
        }

        // Normal exit message
        println!(
            "\n\n{}",
            "═══════════════════════════════════════".bright_cyan()
        );
        println!("{}", "Installation cancelled by user.".bright_white());
        println!(
            "{}",
            "Thank you for using Universal System Installer!".bright_white()
        );
        println!(
            "{}",
            "═══════════════════════════════════════".bright_cyan()
        );
    } else {
        eprintln!("\n{} {:?}", "Error:".bright_red(), e);
        std::process::exit(1);
    }
}
