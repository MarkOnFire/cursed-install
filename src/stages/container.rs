use super::InstallationStage;
use crate::config::ContainerConfig;
use crate::log_generator::LogGenerator;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct ContainerStage {
    config: ContainerConfig,
}

impl ContainerStage {
    pub fn new(config: ContainerConfig) -> Self {
        Self { config }
    }
}

impl InstallationStage for ContainerStage {
    fn name(&self) -> &'static str {
        "Container Orchestration"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_blue().bold());
        println!();

        let mut rng = rand::thread_rng();
        let images = [
            "alpine:latest",
            "nginx:1.21-alpine",
            "postgres:14",
            "redis:6.2",
            "node:16-slim",
            "python:3.9-slim",
        ];

        for image in images {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            println!("{} Pulling {}", LogGenerator::timestamp().dimmed(), image.cyan());
            
            if rng.gen_bool(self.config.failure_rate_image_pull) {
                thread::sleep(Duration::from_millis(rng.gen_range(500..1500)));
                println!("{} Error: Connection timed out while pulling {}", LogGenerator::timestamp().red(), image);
                thread::sleep(Duration::from_millis(1000));
                println!("{} Retrying in 3s...", LogGenerator::timestamp().yellow());
                thread::sleep(Duration::from_millis(3000));
                println!("{} Retrying pull for {}", LogGenerator::timestamp().dimmed(), image.cyan());
            }
            
            let layers = rng.gen_range(3..8);
            for _i in 0..layers {
                let layer_id = LogGenerator::hex_addr();
                let short_id = &layer_id[2..14];
                let progress = ProgressBar::new(ProgressStyle::Block);
                progress.animate(
                    &format!("  {} Pulling fs layer", short_id).dimmed().to_string(),
                    rng.gen_range(self.config.layer_pull_speed_range.clone()),
                    exit_check,
                )?;
            }
            
            println!("{} Digest: sha256:{}", LogGenerator::timestamp().dimmed(), LogGenerator::hex_addr());
            println!("{} Status: Downloaded newer image for {}", LogGenerator::timestamp().dimmed(), image);
            thread::sleep(Duration::from_millis(300));
        }

        println!();
        println!("{} Initializing Kubernetes cluster...", LogGenerator::timestamp().dimmed());
        
        let pods = ["api-gateway", "auth-service", "user-service", "payment-processor", "notification-worker"];
        
        for pod in pods {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            
            println!("{} Scaled up replica set {} to 1", LogGenerator::timestamp().dimmed(), format!("{}-rs", pod).cyan());
            thread::sleep(Duration::from_millis(rng.gen_range(100..300)));

            println!("{} Pod {} Status: {}", LogGenerator::timestamp().dimmed(), pod.yellow(), "Pending".yellow());
            thread::sleep(Duration::from_millis(rng.gen_range(100..300)));
            println!("{} Pod {} Status: {}", LogGenerator::timestamp().dimmed(), pod.yellow(), "ContainerCreating".blue());
            
            if rng.gen_bool(self.config.probability_volume_mount) {
                let pvc_id = format!("pvc-{}", &LogGenerator::hex_addr()[2..10]);
                println!("{} Mounting volume {} to {}", LogGenerator::timestamp().dimmed(), pvc_id.magenta(), pod);
                thread::sleep(Duration::from_millis(rng.gen_range(200..500)));
            }

            if rng.gen_bool(self.config.probability_secret_mount) {
                println!("{} Mounting secret {} to {}", LogGenerator::timestamp().dimmed(), "vault-token".magenta(), pod);
            }

            if rng.gen_bool(self.config.probability_sidecar_injection) {
                println!("{} Injecting sidecar {} to {}", LogGenerator::timestamp().dimmed(), "istio-proxy".cyan(), pod);
                thread::sleep(Duration::from_millis(rng.gen_range(100..300)));
            }
            
            thread::sleep(Duration::from_millis(rng.gen_range(200..500)));
            if rng.gen_bool(self.config.failure_rate_readiness_probe) {
                 println!("{} Warning: Readiness probe failed for {}: Connection refused", LogGenerator::timestamp().yellow(), pod);
                 thread::sleep(Duration::from_millis(800));
            }
            println!("{} Readiness probe passed for {}", LogGenerator::timestamp().dimmed(), pod);
            
            if rng.gen_bool(self.config.failure_rate_crash_loop) {
                 println!("{} Warning: CrashLoopBackOff detected for {}, restarting...", LogGenerator::timestamp().yellow(), pod);
                 thread::sleep(Duration::from_millis(800));
            }
            
            println!("{} Pod {} Status: {}", LogGenerator::timestamp().dimmed(), pod.yellow(), "Running".green());
            println!("{} Pod {} IP: 10.244.{}.{}", LogGenerator::timestamp().dimmed(), pod, rng.gen_range(0..255), rng.gen_range(0..255));
        }

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}
