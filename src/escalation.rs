use crate::cli::Flavor;
use crate::creepy_messages;
use crate::occult_messages;
use crate::scanner::ScanResult;
use colored::*;
use rand::Rng;

// ── Tier ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Baseline, // cycle 1
    Ambient,  // cycles 2-3
    Familiar, // cycles 4-5
    Invasive, // cycles 6-8
    Cosmic,   // cycles 9+
}

impl Tier {
    pub fn from_cycle(cycle: u32) -> Self {
        match cycle {
            1 => Tier::Baseline,
            2..=3 => Tier::Ambient,
            4..=5 => Tier::Familiar,
            6..=8 => Tier::Invasive,
            _ => Tier::Cosmic,
        }
    }

    /// Message probability for this tier.
    pub fn message_probability(&self) -> f64 {
        match self {
            Tier::Baseline => 0.15,
            Tier::Ambient => 0.20,
            Tier::Familiar => 0.25,
            Tier::Invasive => 0.30,
            Tier::Cosmic => 0.35,
        }
    }
}

// ── Color helper ────────────────────────────────────────────────────────────

pub fn tier_color(text: &str, tier: Tier) -> ColoredString {
    match tier {
        Tier::Baseline => text.normal(),
        Tier::Ambient => text.dimmed(),
        Tier::Familiar => text.white(),
        Tier::Invasive => text.yellow(),
        Tier::Cosmic => text.bright_red(),
    }
}

// ── Zalgo ───────────────────────────────────────────────────────────────────

const ZALGO_CHARS: &[char] = &[
    '\u{0335}', // combining short stroke overlay
    '\u{0336}', // combining long stroke overlay
    '\u{0337}', // combining short solidus overlay
    '\u{0338}', // combining long solidus overlay
    '\u{0339}', // combining right half ring below
    '\u{033a}', // combining inverted bridge below
    '\u{0346}', // combining bridge above
    '\u{034e}', // combining upwards arrow below
];

/// Adds Unicode combining characters randomly to ~30-40% of characters.
/// The result looks glitchy but remains readable.
pub fn zalgo_light(text: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut out = String::with_capacity(text.len() * 2);

    for ch in text.chars() {
        out.push(ch);
        if !ch.is_whitespace() && rng.gen_bool(0.35) {
            let idx = rng.gen_range(0..ZALGO_CHARS.len());
            out.push(ZALGO_CHARS[idx]);
        }
    }
    out
}

// ── EscalationEngine ────────────────────────────────────────────────────────

pub struct EscalationEngine<'a> {
    scan: &'a ScanResult,
    flavor: Flavor,
    rng: rand::rngs::ThreadRng,
}

impl<'a> EscalationEngine<'a> {
    pub fn new(scan: &'a ScanResult, flavor: Flavor) -> Self {
        Self {
            scan,
            flavor,
            rng: rand::thread_rng(),
        }
    }

    /// Select the message pool based on the active flavor.
    fn pick_pool(
        &self,
        opsec: &'static [&'static str],
        occult: &'static [&'static str],
    ) -> &'static [&'static str] {
        match self.flavor {
            Flavor::Opsec => opsec,
            Flavor::Occult => occult,
        }
    }

    // ── Public selectors ────────────────────────────────────────────────

    /// Pick a random easter-egg template for the given tier, interpolate it,
    /// and return `None` if no template can be fully resolved.
    pub fn select_easter_egg(&mut self, tier: Tier) -> Option<String> {
        let pool = match tier {
            Tier::Baseline => return None, // Baseline uses original messages
            Tier::Ambient => self.pick_pool(
                creepy_messages::AMBIENT_EASTER_EGGS,
                occult_messages::AMBIENT_EASTER_EGGS,
            ),
            Tier::Familiar => self.pick_pool(
                creepy_messages::FAMILIAR_EASTER_EGGS,
                occult_messages::FAMILIAR_EASTER_EGGS,
            ),
            Tier::Invasive => self.pick_pool(
                creepy_messages::INVASIVE_EASTER_EGGS,
                occult_messages::INVASIVE_EASTER_EGGS,
            ),
            Tier::Cosmic => self.pick_pool(
                creepy_messages::COSMIC_EASTER_EGGS,
                occult_messages::COSMIC_EASTER_EGGS,
            ),
        };
        self.pick_and_interpolate(pool)
    }

    /// Pick a random warning for the given tier.
    pub fn select_warning(&mut self, tier: Tier) -> Option<String> {
        let pool = match tier {
            Tier::Baseline => return None,
            Tier::Ambient => self.pick_pool(
                creepy_messages::AMBIENT_WARNINGS,
                occult_messages::AMBIENT_WARNINGS,
            ),
            Tier::Familiar => self.pick_pool(
                creepy_messages::FAMILIAR_WARNINGS,
                occult_messages::FAMILIAR_WARNINGS,
            ),
            Tier::Invasive => self.pick_pool(
                creepy_messages::INVASIVE_WARNINGS,
                occult_messages::INVASIVE_WARNINGS,
            ),
            Tier::Cosmic => self.pick_pool(
                creepy_messages::COSMIC_WARNINGS,
                occult_messages::COSMIC_WARNINGS,
            ),
        };
        self.pick_and_interpolate(pool)
    }

    /// Pick a tier-appropriate "installation complete" variant.
    pub fn select_completion(&mut self, tier: Tier) -> Option<String> {
        let pool = match tier {
            Tier::Baseline => return None,
            Tier::Ambient => self.pick_pool(
                creepy_messages::AMBIENT_COMPLETION,
                occult_messages::AMBIENT_COMPLETION,
            ),
            Tier::Familiar => self.pick_pool(
                creepy_messages::FAMILIAR_COMPLETION,
                occult_messages::FAMILIAR_COMPLETION,
            ),
            Tier::Invasive => self.pick_pool(
                creepy_messages::INVASIVE_COMPLETION,
                occult_messages::INVASIVE_COMPLETION,
            ),
            Tier::Cosmic => self.pick_pool(
                creepy_messages::COSMIC_COMPLETION,
                occult_messages::COSMIC_COMPLETION,
            ),
        };
        self.pick_and_interpolate(pool)
    }

    /// Creepy cycle headers — only used at Cosmic tier.
    pub fn select_cycle_header(&mut self, tier: Tier, cycle: u32) -> Option<String> {
        if tier != Tier::Cosmic {
            return None;
        }
        let pool = self.pick_pool(
            creepy_messages::COSMIC_CYCLE_HEADERS,
            occult_messages::COSMIC_CYCLE_HEADERS,
        );
        let raw = self.pick_and_interpolate(pool)?;
        Some(raw.replace("{cycle}", &cycle.to_string()))
    }

    /// Exit message for Ctrl+C — only used at Cosmic tier.
    pub fn select_exit_message(&mut self) -> Option<String> {
        let pool = self.pick_pool(
            creepy_messages::COSMIC_EXIT_MESSAGES,
            occult_messages::COSMIC_EXIT_MESSAGES,
        );
        self.pick_and_interpolate(pool)
    }

    // ── Interpolation ───────────────────────────────────────────────────

    /// Replace `{placeholder}` tokens with data from the scan.
    /// Returns `None` if any required placeholder cannot be resolved so the
    /// caller can skip this template and try another.
    fn interpolate(&mut self, template: &str) -> Option<String> {
        let mut result = template.to_string();
        let mut cursor = 0;

        while let Some(open) = result[cursor..].find('{') {
            let open = cursor + open;
            if let Some(close) = result[open..].find('}') {
                let close = open + close;
                let key = &result[open + 1..close];
                let replacement = self.resolve_placeholder(key)?;
                result.replace_range(open..=close, &replacement);
                cursor = open + replacement.len();
            } else {
                break;
            }
        }
        Some(result)
    }

    // ── Internals ───────────────────────────────────────────────────────

    /// Try every template in the pool (in random order) until one fully
    /// interpolates, or return None.
    fn pick_and_interpolate(&mut self, pool: &[&str]) -> Option<String> {
        if pool.is_empty() {
            return None;
        }

        // Shuffle indices so we don't always try templates in the same order
        let mut indices: Vec<usize> = (0..pool.len()).collect();
        for i in (1..indices.len()).rev() {
            let j = self.rng.gen_range(0..=i);
            indices.swap(i, j);
        }

        for &idx in &indices {
            if let Some(interpolated) = self.interpolate(pool[idx]) {
                return Some(interpolated);
            }
        }
        None
    }

    /// Resolve a single placeholder key to its value.
    fn resolve_placeholder(&mut self, key: &str) -> Option<String> {
        match key {
            "hostname" => opt_non_empty(&self.scan.hostname),
            "username" => opt_non_empty(&self.scan.username),
            "os" => opt_non_empty(&self.scan.os_name),
            "project" => self.pick_random_vec(&self.scan.project_names),
            "git_repo" => self.pick_random_vec(&self.scan.git_repos),
            "ssh_key" => self.pick_random_vec(&self.scan.ssh_key_names),
            "browser" => self.pick_random_vec(&self.scan.browser_profiles),
            "cloud" => self.pick_random_vec(&self.scan.cloud_configs),
            "desktop_count" => self.scan.desktop_count.map(|c| c.to_string()),
            "downloads_count" => self.scan.downloads_count.map(|c| c.to_string()),
            "scan_time" => Some(self.scan.scan_timestamp.clone()),
            "dotfile" => self.pick_random_vec(&self.scan.dotfile_names),
            "env_count" => Some(self.scan.env_file_count.to_string()),
            "history_lines" => self.scan.shell_history_lines.map(|l| l.to_string()),
            "files_scanned" => Some(self.scan.files_scanned.to_string()),
            // {cycle} is handled specially in select_cycle_header
            "cycle" => Some("{cycle}".to_string()),
            _ => None,
        }
    }

    /// Pick a random element from a `Vec<String>`, returning `None` if empty.
    fn pick_random_vec(&mut self, items: &[String]) -> Option<String> {
        if items.is_empty() {
            None
        } else {
            let idx = self.rng.gen_range(0..items.len());
            Some(items[idx].clone())
        }
    }
}

/// Helper: return `Some(clone)` if the Option contains a non-empty string.
fn opt_non_empty(s: &Option<String>) -> Option<String> {
    s.as_ref().filter(|v| !v.is_empty()).cloned()
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_from_cycle_boundaries() {
        assert_eq!(Tier::from_cycle(1), Tier::Baseline);
        assert_eq!(Tier::from_cycle(2), Tier::Ambient);
        assert_eq!(Tier::from_cycle(3), Tier::Ambient);
        assert_eq!(Tier::from_cycle(4), Tier::Familiar);
        assert_eq!(Tier::from_cycle(5), Tier::Familiar);
        assert_eq!(Tier::from_cycle(6), Tier::Invasive);
        assert_eq!(Tier::from_cycle(8), Tier::Invasive);
        assert_eq!(Tier::from_cycle(9), Tier::Cosmic);
        assert_eq!(Tier::from_cycle(100), Tier::Cosmic);
    }

    #[test]
    fn tier_probabilities() {
        assert!((Tier::Baseline.message_probability() - 0.15).abs() < f64::EPSILON);
        assert!((Tier::Cosmic.message_probability() - 0.35).abs() < f64::EPSILON);
    }

    #[test]
    fn zalgo_preserves_readability() {
        let input = "Hello World";
        let output = zalgo_light(input);
        // Original chars must still be present in order
        let stripped: String = output
            .chars()
            .filter(|c| !ZALGO_CHARS.contains(c))
            .collect();
        assert_eq!(stripped, input);
    }

    #[test]
    fn zalgo_adds_combining_chars() {
        // Run on a long string to be statistically confident
        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = zalgo_light(input);
        assert!(output.len() > input.len(), "zalgo should add characters");
    }
}
