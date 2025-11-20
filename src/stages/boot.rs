use super::InstallationStage;
use crate::config::BootConfig;
use crate::kernel_logs::KernelLogs;
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct BootStage {
    config: BootConfig,
    kernel_logs: KernelLogs,
}

impl BootStage {
    pub fn new(config: BootConfig) -> Self {
        Self {
            config,
            kernel_logs: KernelLogs::load(),
        }
    }
}

impl InstallationStage for BootStage {
    fn name(&self) -> &'static str {
        "Kernel Boot Sequence"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        let log_count = rng.gen_range(self.config.log_count_range.clone());
        let logs = self.kernel_logs.random_batch(log_count);

        for log in logs {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            println!("{}", log.dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(self.config.log_delay_range.clone())));
        }

        println!();
        thread::sleep(Duration::from_millis(self.config.final_delay));

        Ok(())
    }
}
