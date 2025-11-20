use super::InstallationStage;
use crate::config::AiConfig;
use crate::log_generator::LogGenerator;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct AiStage {
    config: AiConfig,
}

impl AiStage {
    pub fn new(config: AiConfig) -> Self {
        Self { config }
    }
}

impl InstallationStage for AiStage {
    fn name(&self) -> &'static str {
        "AI Model Loading"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_magenta().bold());
        println!();

        let mut rng = rand::thread_rng();

        println!("{} Initializing HuggingFace Hub client...", LogGenerator::timestamp().dimmed());
        thread::sleep(Duration::from_millis(600));
        
        let model_name = "Llama-3-70B-Instruct-v1";
        println!("{} Found model {} (size: 140GB)", LogGenerator::timestamp().dimmed(), model_name.cyan());

        if rng.gen_bool(self.config.failure_rate_network) {
            println!("{} Error: HuggingFace Hub: 502 Bad Gateway", LogGenerator::timestamp().red());
            println!("{} Retrying connection in 3s...", LogGenerator::timestamp().yellow());
            thread::sleep(Duration::from_millis(3000));
            println!("{} Connection established.", LogGenerator::timestamp().dimmed());
        }

        println!("{} Downloading model weights...", LogGenerator::timestamp().dimmed());
        let progress = ProgressBar::new(ProgressStyle::Block);
        progress.animate("Downloading", rng.gen_range(self.config.model_download_speed_range.clone()), exit_check)?;

        println!("{} Verifying SHA256 checksums...", LogGenerator::timestamp().dimmed());
        thread::sleep(Duration::from_millis(rng.gen_range(self.config.checksum_delay_range.clone())));
        if rng.gen_bool(self.config.failure_rate_checksum) {
             println!("{} Warning: Checksum mismatch for shard 03, re-downloading...", LogGenerator::timestamp().yellow());
             thread::sleep(Duration::from_millis(1000));
        }
        println!("{} Integrity check passed.", LogGenerator::timestamp().dimmed());

        println!("{} Initializing CUDA context...", LogGenerator::timestamp().dimmed());
        thread::sleep(Duration::from_millis(500));

        println!("{} Compiling custom CUDA kernels (FlashAttention-v2)...", LogGenerator::timestamp().dimmed());
        let progress = ProgressBar::new(ProgressStyle::Block);
        progress.animate("Compiling", rng.gen_range(self.config.compilation_speed_range.clone()), exit_check)?;

        if rng.gen_bool(self.config.failure_rate_kernel_panic) {
             println!("{} Error: illegal memory access in kernel 'fused_rotary_embedding'", LogGenerator::timestamp().red());
             println!("{} Resetting CUDA context and recompiling...", LogGenerator::timestamp().yellow());
             thread::sleep(Duration::from_millis(2000));
        }

        println!("{} Allocating tensors...", LogGenerator::timestamp().dimmed());
        
        if rng.gen_bool(self.config.failure_rate_oom) {
            println!("{} Error: CUDA out of memory. Tried to allocate 24.5GB", LogGenerator::timestamp().red());
            println!("{} Reducing batch size to 1 and offloading optimizer state...", LogGenerator::timestamp().yellow());
            thread::sleep(Duration::from_millis(1500));
        }

        let layers = 12;
        for i in 1..=layers {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            
            let layer_type = if i % 2 == 0 { "FeedForward" } else { "Attention" };
            println!(
                "{} Loading layer {}/{} ({})...", 
                LogGenerator::timestamp().dimmed(), 
                i, 
                layers, 
                layer_type.cyan()
            );
            
            thread::sleep(Duration::from_millis(rng.gen_range(self.config.layer_load_delay_range.clone())));
        }
        
        println!("{} Model loaded successfully.", LogGenerator::timestamp().dimmed());
        println!("{} Quantization: INT8", LogGenerator::timestamp().dimmed());
        println!("{} Inference engine ready.", LogGenerator::timestamp().dimmed());

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}
