use super::InstallationStage;
use crate::config::CloudConfig;
use crate::log_generator::LogGenerator;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct CloudStage {
    config: CloudConfig,
}

impl CloudStage {
    pub fn new(config: CloudConfig) -> Self {
        Self { config }
    }
}

impl InstallationStage for CloudStage {
    fn name(&self) -> &'static str {
        "Cloud Infrastructure Provisioning"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_cyan().bold());
        println!();

        let mut rng = rand::thread_rng();
        
        println!("{} Initializing Terraform backend...", LogGenerator::timestamp().dimmed());
        thread::sleep(Duration::from_millis(600));

        let resources = [
            ("aws_vpc.main", "VPC"),
            ("aws_subnet.public_1", "Subnet"),
            ("aws_subnet.public_2", "Subnet"),
            ("aws_internet_gateway.gw", "Gateway"),
            ("aws_iam_role.lambda_exec", "IAM Role"),
            ("aws_security_group.allow_tls", "Security Group"),
            ("aws_instance.web_server", "EC2 Instance"),
            ("aws_db_instance.default", "RDS Instance"),
            ("aws_dynamodb_table.users", "DynamoDB"),
            ("aws_lambda_function.processor", "Lambda"),
            ("aws_kinesis_stream.events", "Kinesis"),
            ("aws_s3_bucket.assets", "S3 Bucket"),
            ("aws_route53_record.www", "Route53"),
            ("aws_cloudfront_distribution.cdn", "CloudFront"),
        ];

        for (resource, r_type) in resources {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let colored_resource = match r_type {
                "IAM Role" => resource.yellow(),
                "EC2 Instance" | "Lambda" => resource.green(),
                "RDS Instance" | "DynamoDB" => resource.blue(),
                "S3 Bucket" | "CloudFront" => resource.magenta(),
                _ => resource.cyan(),
            };

            println!("{} Creating {} ({})", LogGenerator::timestamp().dimmed(), colored_resource, r_type.dimmed());
            
            if rng.gen_bool(self.config.failure_rate_rate_limit) {
                thread::sleep(Duration::from_millis(rng.gen_range(200..500)));
                println!("{} Error: 429 Too Many Requests (RequestLimitExceeded)", LogGenerator::timestamp().red());
                println!("{} Throttling...", LogGenerator::timestamp().yellow());
                thread::sleep(Duration::from_millis(2000));
                println!("{} Resuming operation...", LogGenerator::timestamp().dimmed());
            }

            if r_type == "EC2 Instance" && rng.gen_bool(self.config.failure_rate_insufficient_capacity) {
                thread::sleep(Duration::from_millis(1000));
                println!("{} Error: InsufficientInstanceCapacity: We currently do not have sufficient capacity in the Availability Zone you requested.", LogGenerator::timestamp().red());
                println!("{} Retrying in different Availability Zone (us-east-1b)...", LogGenerator::timestamp().yellow());
                thread::sleep(Duration::from_millis(1500));
            }

            if r_type == "Lambda" && rng.gen_bool(self.config.failure_rate_dependency_violation) {
                 println!("{} Error: The role defined for the function cannot be assumed by the function.", LogGenerator::timestamp().red());
                 println!("{} Waiting for IAM propagation...", LogGenerator::timestamp().yellow());
                 thread::sleep(Duration::from_millis(2500));
            }

            if r_type == "S3 Bucket" && rng.gen_bool(self.config.failure_rate_checksum_mismatch) {
                 println!("{} Error: Checksum mismatch during upload.", LogGenerator::timestamp().red());
                 println!("{} Re-calculating hashes and retrying...", LogGenerator::timestamp().yellow());
                 thread::sleep(Duration::from_millis(1200));
            }

            let duration = rng.gen_range(self.config.provision_speed_range.clone());
            let progress = ProgressBar::new(ProgressStyle::Block);
            progress.animate("Provisioning", duration, exit_check)?;
            
            println!("{} Resource {} is Available", LogGenerator::timestamp().dimmed(), colored_resource);
        }

        println!();
        println!("{}", "Infrastructure provisioning complete.".bright_green());

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}
