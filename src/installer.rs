use crate::cli::{Flavor, Stage};
use crate::escalation::{zalgo_light, EscalationEngine, Tier, tier_color};
use crate::messages::{EASTER_EGGS, RETRY_MESSAGES, WARNINGS};
use crate::scanner::ScanResult;
use crate::stages::selected_stages;
use crate::ui::Spinner;
use colored::*;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::io;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Installer {
    rng: rand::rngs::ThreadRng,
    selected_stages: Vec<Stage>,
    scan: Option<Arc<ScanResult>>,
    flavor: Flavor,
}

impl Installer {
    pub fn new(stages: Vec<Stage>, scan: Option<Arc<ScanResult>>, flavor: Flavor) -> Self {
        Self {
            rng: rand::thread_rng(),
            selected_stages: stages,
            scan,
            flavor,
        }
    }

    fn check_exit(&self) -> bool {
        if event::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(key_event)) = event::read() {
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers.contains(event::KeyModifiers::CONTROL)
                {
                    return true;
                }
            }
        }
        false
    }

    fn print_header(&self) {
        println!(
            "{}",
            "=================================================================".bright_cyan()
        );
        println!(
            "{}",
            "         UNIVERSAL SYSTEM INSTALLER v3.2.1 (Build 1999)"
                .bright_white()
                .bold()
        );
        println!(
            "{}",
            "=================================================================".bright_cyan()
        );
        println!();
        thread::sleep(Duration::from_millis(1500));
    }

    fn show_easter_egg(&mut self, tier: Tier) -> io::Result<()> {
        let prob = tier.message_probability();

        if !self.rng.gen_bool(prob) {
            return Ok(());
        }

        // Try creepy message first if we have scan data and tier > Baseline
        if tier != Tier::Baseline {
            if let Some(scan) = &self.scan {
                let mut engine = EscalationEngine::new(scan, self.flavor);
                if let Some(msg) = engine.select_easter_egg(tier) {
                    println!();
                    let display = if tier == Tier::Cosmic {
                        zalgo_light(&msg)
                    } else {
                        msg
                    };
                    let mut spinner = Spinner::new();
                    let colored_msg = tier_color(&display, tier).to_string();
                    spinner.animate(&colored_msg, 1500, &|| self.check_exit())?;
                    println!();
                    return Ok(());
                }
            }
        }

        // Fallback to original easter eggs
        println!();
        let egg = EASTER_EGGS[self.rng.gen_range(0..EASTER_EGGS.len())];
        let mut spinner = Spinner::new();
        spinner.animate(egg, 1500, &|| self.check_exit())?;
        println!();
        Ok(())
    }

    fn show_warning(&mut self, tier: Tier) {
        let prob = tier.message_probability();

        if !self.rng.gen_bool(prob) {
            return;
        }

        // Try creepy warning first if we have scan data and tier > Baseline
        if tier != Tier::Baseline {
            if let Some(scan) = &self.scan {
                let mut engine = EscalationEngine::new(scan, self.flavor);
                if let Some(msg) = engine.select_warning(tier) {
                    let display = if tier == Tier::Cosmic {
                        zalgo_light(&msg)
                    } else {
                        msg
                    };
                    println!("\n{}", tier_color(&display, tier));
                    thread::sleep(Duration::from_millis(1000));
                    if tier == Tier::Cosmic {
                        println!("{}", "...".bright_red().dimmed());
                    } else {
                        println!("{}", "Continuing anyway...".dimmed());
                    }
                    println!();
                    return;
                }
            }
        }

        // Fallback to original warnings
        let warning = WARNINGS[self.rng.gen_range(0..WARNINGS.len())];
        println!("\n{}", warning.yellow());
        thread::sleep(Duration::from_millis(1000));
        println!("{}", "Continuing anyway...".dimmed());
        println!();
    }

    fn show_retry(&mut self) -> io::Result<()> {
        if self.rng.gen_bool(0.1) {
            let message = RETRY_MESSAGES[self.rng.gen_range(0..RETRY_MESSAGES.len())];
            println!("\n{}", message.yellow());
            thread::sleep(Duration::from_millis(800));

            let mut spinner = Spinner::new();
            spinner.animate("Reconnecting to mirror.oldsoft.org", 1200, &|| {
                self.check_exit()
            })?;
            println!();
        }
        Ok(())
    }

    fn show_cycle_header(&mut self, cycle: u32, tier: Tier) {
        if tier == Tier::Cosmic {
            if let Some(scan) = &self.scan {
                let mut engine = EscalationEngine::new(scan, self.flavor);
                if let Some(header) = engine.select_cycle_header(tier, cycle) {
                    let display = zalgo_light(&header);
                    println!(
                        "\n{}",
                        "═══════════════════════════════════════════════════════════════"
                            .bright_red()
                    );
                    println!("{}", display.bright_red().bold());
                    println!(
                        "{}",
                        "═══════════════════════════════════════════════════════════════"
                            .bright_red()
                    );
                    thread::sleep(Duration::from_millis(1000));
                    return;
                }
            }
        }

        // Normal cycle header
        println!(
            "\n{}",
            "═══════════════════════════════════════════════════════════════"
                .bright_magenta()
        );
        println!(
            "{}",
            format!("Beginning installation cycle #{}...", cycle)
                .bright_magenta()
                .bold()
        );
        println!(
            "{}",
            "═══════════════════════════════════════════════════════════════"
                .bright_magenta()
        );
        thread::sleep(Duration::from_millis(1000));
    }

    fn show_completion(&mut self, tier: Tier) {
        if tier != Tier::Baseline {
            if let Some(scan) = &self.scan {
                let mut engine = EscalationEngine::new(scan, self.flavor);
                if let Some(msg) = engine.select_completion(tier) {
                    let display = if tier == Tier::Cosmic {
                        zalgo_light(&msg)
                    } else {
                        msg
                    };
                    println!("\n{}", tier_color(&display, tier).bold());
                    thread::sleep(Duration::from_millis(2000));
                    return;
                }
            }
        }

        // Normal completion message
        println!(
            "\n{}",
            "Installation complete! Restarting installation process..."
                .bright_green()
                .bold()
        );
        thread::sleep(Duration::from_millis(2000));
    }

    pub fn run(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(
            io::stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        terminal::disable_raw_mode()?;

        self.print_header();

        println!(
            "{}",
            "Initializing installation environment...".bright_white()
        );
        thread::sleep(Duration::from_millis(1000));

        let mut spinner = Spinner::new();
        spinner.animate("Detecting hardware configuration...", 1500, &|| {
            self.check_exit()
        })?;
        println!();

        let mut cycle: u32 = 0;
        loop {
            cycle += 1;
            let tier = Tier::from_cycle(cycle);

            if cycle > 1 {
                self.show_cycle_header(cycle, tier);
            }

            let stages = selected_stages(&self.selected_stages);

            for stage in stages {
                if self.check_exit() {
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
                }

                self.show_easter_egg(tier)?;
                self.show_warning(tier);
                self.show_retry()?;

                stage.run(&|| self.check_exit())?;

                thread::sleep(Duration::from_millis(self.rng.gen_range(300..800)));
            }

            self.show_completion(tier);
        }
    }
}

impl Default for Installer {
    fn default() -> Self {
        Self::new(Stage::all(), None, Flavor::Opsec)
    }
}
