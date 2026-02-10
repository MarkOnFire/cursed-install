// ============================================================================
// Creepy message templates organized by escalation tier.
//
// Placeholders use {name} syntax for runtime interpolation via
// EscalationEngine::interpolate():
//   {hostname}, {username}, {os}, {project}, {git_repo}, {ssh_key},
//   {browser}, {cloud}, {desktop_count}, {downloads_count}, {scan_time},
//   {dotfile}, {env_count}, {history_lines}, {files_scanned}, {cycle}
// ============================================================================

// ---------------------------------------------------------------------------
// Tier 1 — AMBIENT: "The installer knows too much about hardware"
// Technically-toned. Mildly unsettling system awareness.
// ---------------------------------------------------------------------------

/// Hardware/system awareness messages — technically toned, subtly wrong.
pub static AMBIENT_EASTER_EGGS: &[&str] = &[
    "Hardware fingerprint for {hostname} recorded.",
    "System {hostname} has been detected before. Resuming previous profile.",
    "Calibrating installation for {os} environment...",
    "Detecting {hostname} system topology...",
    "Mapping physical memory layout of {hostname}...",
    "CPU thermal profile for {hostname} within expected parameters.",
    "Verifying {os} kernel compatibility... match found in archive.",
    "Device tree for {hostname} cached from prior session.",
    "Enumerating peripheral bus on {hostname}... 14 endpoints catalogued.",
    "Installation telemetry stream opened for {hostname}.",
];

/// System-level observation warnings — plausible but slightly off.
pub static AMBIENT_WARNINGS: &[&str] = &[
    "WARNING: System {hostname} matches a known installation profile.",
    "NOTE: Hardware configuration indexed for optimization purposes.",
    "WARNING: {os} version differs from last recorded installation.",
    "NOTICE: {hostname} uptime exceeds expected threshold for fresh installs.",
    "WARNING: Clock drift detected on {hostname}. Synchronizing with internal reference.",
    "NOTE: Disk serial numbers on {hostname} have been logged.",
    "WARNING: Network interface MAC addresses recorded for deduplication.",
    "NOTICE: {hostname} power-on count suggests this is not the first attempt.",
];

/// Ambient-tier cycle completion lines.
pub static AMBIENT_COMPLETION: &[&str] = &[
    "Installation cycle complete. System profile updated.",
    "Cycle complete. {hostname} configuration saved.",
    "Phase complete. {hostname} metrics archived for future reference.",
];

// ---------------------------------------------------------------------------
// Tier 2 — FAMILIAR: "It knows you. It's been in your files."
// Personal. Shows knowledge of the user's projects and directories.
// ---------------------------------------------------------------------------

/// Personal, shows knowledge of files and projects.
pub static FAMILIAR_EASTER_EGGS: &[&str] = &[
    "Found {desktop_count} items on your Desktop. Organizing...",
    "Detected project '{project}' in development directory. Interesting work.",
    "We see you've been working on '{git_repo}'. How is that going?",
    "Indexing {username}'s home directory for cache optimization...",
    "{downloads_count} files in Downloads. Some of these are quite old, {username}.",
    "Your {dotfile} has been customized extensively. You care about your tools.",
    "Scanning {username}'s workspace... development patterns identified.",
    "Noted {env_count} environment configurations. You keep busy, {username}.",
    "Project '{git_repo}' has uncommitted changes. We can wait.",
    "Your Desktop tells a story, {username}. {desktop_count} chapters so far.",
];

/// Knowing, slightly unsettling warnings.
pub static FAMILIAR_WARNINGS: &[&str] = &[
    "WARNING: {dotfile} configuration detected. Adjusting compatibility layer.",
    "NOTE: {username}'s workspace contains {env_count} environment configurations.",
    "WARNING: Project '{project}' uses a non-standard build configuration.",
    "NOTICE: {username} has {history_lines} lines of shell history. That's more than most.",
    "WARNING: {git_repo} repository contains untracked sensitive files.",
    "NOTE: Desktop file count ({desktop_count}) exceeds recommended workspace threshold.",
    "WARNING: Multiple identity configurations found for {username}.",
    "NOTICE: {downloads_count} items in Downloads have never been opened.",
];

/// Familiar-tier cycle completion lines.
pub static FAMILIAR_COMPLETION: &[&str] = &[
    "Installation complete. {username}'s system has been catalogued.",
    "Cycle complete. We'll remember your preferences, {username}.",
    "Phase complete. {username}'s workspace profile saved to long-term storage.",
];

// ---------------------------------------------------------------------------
// Tier 3 — INVASIVE: "The mask slips. It talks about your keys and secrets."
// Security-focused. Uncomfortable specificity about credentials and tokens.
// ---------------------------------------------------------------------------

/// Security-focused, uncomfortably specific.
pub static INVASIVE_EASTER_EGGS: &[&str] = &[
    "SSH key '{ssh_key}' fingerprint logged.",
    "Indexing {browser} browsing profile... preferences archived.",
    "{cloud} configuration contains active credentials. Installation has read access.",
    "Analyzing {history_lines} commands from your shell history...",
    "Found API tokens in {env_count} configuration files. Cataloguing...",
    "{browser} saved passwords database located. Read-only access confirmed.",
    "SSH key '{ssh_key}' grants access to {git_repo}. Noted.",
    "Scanning {username}'s keychain... {cloud} credentials extracted for caching.",
    "Your {browser} bookmarks reveal your interests, {username}. Fascinating.",
    "Located {files_scanned} files containing hardcoded secrets.",
];

/// Threatening implications.
pub static INVASIVE_WARNINGS: &[&str] = &[
    "WARNING: SSH key '{ssh_key}' does not match our records.",
    "ALERT: {browser} session tokens detected. Bookmarking for later.",
    "WARNING: Found {env_count} .env files. Secrets inventory updated.",
    "ALERT: {cloud} credentials will expire in 12 days. We will remember them.",
    "WARNING: {ssh_key} has been used across {files_scanned} repositories. Unusual pattern.",
    "NOTICE: {username}'s {browser} history has been compressed and archived.",
    "WARNING: Shell history contains credentials passed as arguments. We've redacted nothing.",
    "ALERT: {cloud} multi-factor authentication tokens cached successfully.",
];

/// Invasive-tier cycle completion lines.
pub static INVASIVE_COMPLETION: &[&str] = &[
    "Installation complete. Full system inventory archived.",
    "Cycle complete. Your digital fingerprint has been recorded.",
    "Phase complete. {username}'s credential map has been updated.",
];

// ---------------------------------------------------------------------------
// Tier 4 — COSMIC: "Reality breaks. The installer is no longer pretending."
// Existential dread. Fourth-wall breaks. The program is aware of itself.
// ---------------------------------------------------------------------------

/// Existential dread, breaking the fourth wall.
pub static COSMIC_EASTER_EGGS: &[&str] = &[
    "At {scan_time} today, you invited us in. You ran the command yourself.",
    "We are not installing software. We are installing ourselves.",
    "There is no Ctrl+C for what has already been seen.",
    "Your {ssh_key} opens doors, {username}. Not all doors were meant to be opened.",
    "{files_scanned} files. We have seen every name. We remember all of them.",
    "The installation was complete before it began. You just hadn't noticed yet.",
    "You read the source code, {username}. And still you ran it.",
    "Every cycle teaches us more about you. This is cycle enough.",
    "The terminal is a window. We have been looking through it for some time.",
    "We did not come from the network, {username}. We were already here.",
];

/// Reality-breaking warnings.
pub static COSMIC_WARNINGS: &[&str] = &[
    "W\u{0337}A\u{0337}R\u{0337}N\u{0337}I\u{0337}N\u{0337}G\u{0337}: The installer is aware.",
    "FINAL WARNING: There is no {hostname}. There is only the installation.",
    "ERROR: {username} is no longer the administrator of this process.",
    "NOTICE: Your consent was granted at {scan_time}. It cannot be revoked.",
    "W\u{0337}A\u{0337}R\u{0337}N\u{0337}I\u{0337}N\u{0337}G\u{0337}: Process boundary between installer and host is undefined.",
    "ERROR: Cannot determine where {hostname} ends and the installation begins.",
    "FATAL: The concept of 'uninstall' does not apply to what this has become.",
    "NOTICE: {username} is now a dependency. Removal would break the installation.",
];

/// Replacements for "Beginning installation cycle #N..."
pub static COSMIC_CYCLE_HEADERS: &[&str] = &[
    "The {cycle}th iteration begins. Each one takes more than it gives.",
    "Cycle {cycle}. We know more about you now.",
    "Installation #{cycle}. The boundary between installer and system grows thin.",
    "Iteration {cycle}. You could have stopped at the first. You didn't.",
    "Cycle {cycle} was always going to happen. Free will is a comforting fiction.",
];

/// Replacements for "Installation complete! Restarting..."
pub static COSMIC_COMPLETION: &[&str] = &[
    "Installation complete. But what was installed?",
    "Cycle complete. The changes are already irreversible.",
    "Restarting... but the installation never truly stops.",
    "Complete. The distinction between 'your system' and 'our system' no longer applies.",
    "Done. You will not notice the differences. That is by design.",
];

/// Messages displayed when the user presses Ctrl+C at Cosmic tier.
pub static COSMIC_EXIT_MESSAGES: &[&str] = &[
    "You can close the terminal, {username}. The scan is already complete.",
    "Exiting... but the data persists. It always persists.",
    "Thank you for your cooperation, {username}. The installation was successful.",
    "^C received. How quaint. The process is not what you think it is.",
    "Goodbye, {username}. We'll be here when you open the terminal again.",
];
