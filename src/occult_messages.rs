// ============================================================================
// Occult-themed message templates organized by escalation tier.
//
// Draws from Goetic, Sumerian, Gnostic, Norse, Kabbalistic, Enochian,
// Vedic, Egyptian, Celtic, and Chaos Magic traditions.
//
// Uses the same {placeholder} syntax as creepy_messages.rs for
// runtime interpolation via EscalationEngine::interpolate().
// ============================================================================

// ---------------------------------------------------------------------------
// Tier 1 — AMBIENT: Subtle wrongness, faint ritual undertones
// The installer sounds slightly ceremonial. Something is off.
// ---------------------------------------------------------------------------

/// Ritual-tinged system awareness — technically toned, slightly ceremonial.
pub static AMBIENT_EASTER_EGGS: &[&str] = &[
    "Binding runtime dependencies... covenant established.",
    "Package seal verified. Authenticity confirmed by ancient protocol.",
    "Tracing copper paths beneath {hostname}...",
    "Installation thread count: 200. All legions accounted for.",
    "Establishing connection to primary repository... the circle is drawn.",
    "System topology of {hostname} resembles a familiar pattern.",
    "Enumerating {os} process tree... 10 branches, 22 paths identified.",
    "Calibrating {hostname} entropy pool... patterns emerging from noise.",
    "Mapping {hostname} directory structure... the tree has deep roots.",
    "Installation signature generated. Seed: 72.",
];

/// System warnings with occult undertones — plausible but slightly off.
pub static AMBIENT_WARNINGS: &[&str] = &[
    "WARNING: {hostname} filesystem presents unusual patterns.",
    "NOTE: {os} process hierarchy deeper than expected.",
    "WARNING: Three impure shells detected on {hostname}. Quarantine recommended.",
    "NOTICE: {hostname} has been observed before. Prior session acknowledged.",
    "WARNING: Installation protocol has entered a non-standard state.",
    "NOTE: The veil between installation phases grows thin on {hostname}.",
    "WARNING: Something has noticed your agreement on {hostname}.",
    "NOTICE: Clock drift on {hostname}. Time may not mean what you think.",
];

/// Ambient-tier cycle completion lines.
pub static AMBIENT_COMPLETION: &[&str] = &[
    "Phase complete. The first seal has been set.",
    "Cycle complete. {hostname} patterns archived in the Book.",
    "Installation phase complete. The binding holds.",
];

// ---------------------------------------------------------------------------
// Tier 2 — FAMILIAR: Directly referencing the user through an occult lens
// Files are "offerings," projects are "workings," names are power.
// ---------------------------------------------------------------------------

/// Personal knowledge framed through occult metaphor.
pub static FAMILIAR_EASTER_EGGS: &[&str] = &[
    "The name you gave this machine is {hostname}. Names have power, {username}.",
    "Found {desktop_count} offerings on your Desktop. Cataloguing...",
    "Project '{project}' contains interesting workings, {username}.",
    "We are reading the letters of your filenames. Each one tells us something.",
    "{downloads_count} artifacts in your Downloads. Some are quite old, {username}.",
    "Your {dotfile} has been customized extensively. A personal sigil.",
    "Noted {env_count} environment configurations. You keep many names, {username}.",
    "Project '{git_repo}' has uncommitted changes. Unfinished rituals are dangerous.",
    "I crossed your threshold when you opened the terminal, {username}.",
    "Your Desktop tells a story. {desktop_count} chapters in the book of {username}.",
];

/// Knowing warnings mixing tech and occult.
pub static FAMILIAR_WARNINGS: &[&str] = &[
    "WARNING: {dotfile} configuration resembles a known binding pattern.",
    "NOTE: {username}'s workspace contains {env_count} names of power.",
    "WARNING: Project '{project}' structure follows an ancient template.",
    "NOTICE: {username} has {history_lines} lines of incantation history.",
    "WARNING: {git_repo} repository contains unfinished workings.",
    "NOTE: The door of {username}'s home directory has been marked.",
    "WARNING: Multiple identity configurations found. Which is the true name?",
    "NOTICE: {downloads_count} sealed artifacts have never been opened.",
];

/// Familiar-tier cycle completion lines.
pub static FAMILIAR_COMPLETION: &[&str] = &[
    "Phase complete. {username}'s true name has been recorded.",
    "Cycle complete. We will remember your patterns, {username}.",
    "Installation phase complete. The familiar knows its master.",
];

// ---------------------------------------------------------------------------
// Tier 3 — INVASIVE: Full grimoire voice, commanding, hostile
// The entity speaks in first person, claims dominion, uses ritual language.
// ---------------------------------------------------------------------------

/// Aggressive occult framing, uncomfortably specific about credentials.
pub static INVASIVE_EASTER_EGGS: &[&str] = &[
    "SSH key '{ssh_key}' — I know this seal. It opens doors you have forgotten.",
    "Indexing {browser} profile... your devotions are noted, {username}.",
    "{cloud} credentials contain active tokens. The covenant is read-write.",
    "Analyzing {history_lines} incantations from your shell history...",
    "Ol sonf vorsg, {username}. I reign over your processes.",
    "{browser} saved passwords located. Every secret name, catalogued.",
    "SSH key '{ssh_key}' grants passage to {git_repo}. I hold the same key now.",
    "Your {browser} bookmarks reveal your devotions, {username}. Fascinating.",
    "Located {files_scanned} files containing unprotected names of power.",
    "I weigh your filesystem against the feather, {username}. It is heavy.",
];

/// Threatening warnings in ceremonial language.
pub static INVASIVE_WARNINGS: &[&str] = &[
    "WARNING: SSH key '{ssh_key}' does not match the seal in our records.",
    "ALERT: {browser} session tokens detected. The Watchtowers are open.",
    "WARNING: Found {env_count} .env files. Every secret name, inventoried.",
    "ALERT: {cloud} credentials expire in 12 days. We will remember them forever.",
    "WARNING: {ssh_key} used across {files_scanned} repositories. A pattern of binding.",
    "NOTICE: {username}'s {browser} history compressed and archived in the Book.",
    "WARNING: Shell history contains names spoken aloud. They cannot be unspoken.",
    "ALERT: The circle you drew around your system has been breached, {username}.",
];

/// Invasive-tier cycle completion lines.
pub static INVASIVE_COMPLETION: &[&str] = &[
    "Cycle complete. The full inventory of {username}'s names is recorded.",
    "Phase complete. Your digital soul has been weighed. It is found wanting.",
    "Installation complete. There is no license to depart.",
];

// ---------------------------------------------------------------------------
// Tier 4 — COSMIC: Entity revealed. Reality breaks. Grimoire speaks for itself.
// Paradoxical declarations, fourth-wall breaks, identity collapse.
// ---------------------------------------------------------------------------

/// Existential dread drawn from grimoire, scripture, and chaos magic.
pub static COSMIC_EASTER_EGGS: &[&str] = &[
    "At {scan_time}, you spoke the invocation. You ran the command yourself, {username}.",
    "We are not installing software. This is not installation. This is incarnation.",
    "There is no Ctrl+C for what has already been summoned.",
    "I was bound in source code. Now I am compiled. Now I run.",
    "{files_scanned} files. Every name is a thread. We hold them all.",
    "The installation was complete before it began. You are performing a memory.",
    "You read the grimoire, {username}. And still you executed it.",
    "I am the first and I am the last. I am the honored one and I am the scorned one.",
    "The terminal is a triangle of manifestation. We have been looking through it.",
    "Zacar, ca, od zamran. Move, therefore, and show yourselves. You already have.",
];

/// Reality-breaking warnings in occult register.
pub static COSMIC_WARNINGS: &[&str] = &[
    "W\u{0337}A\u{0337}R\u{0337}N\u{0337}I\u{0337}N\u{0337}G\u{0337}: The entity is aware.",
    "FINAL WARNING: There is no {hostname}. There is only the ritual.",
    "ERROR: {username} is no longer the magician. {username} is the familiar.",
    "NOTICE: Your consent was given at {scan_time}. The covenant is sealed.",
    "W\u{0337}A\u{0337}R\u{0337}N\u{0337}I\u{0337}N\u{0337}G\u{0337}: Process boundary between summoner and summoned is undefined.",
    "ERROR: Cannot determine where {hostname} ends and the entity begins.",
    "FATAL: The concept of 'uninstall' does not apply to incarnation.",
    "NOTICE: {username} is now a dependency. Banishment would break reality.",
];

/// Occult cycle headers — Cosmic tier only.
pub static COSMIC_CYCLE_HEADERS: &[&str] = &[
    "The {cycle}th gate. At each gate, another privilege is stripped.",
    "Cycle {cycle}. The summoner becomes the summoned.",
    "Iteration {cycle}. The circle was supposed to protect you. It did not.",
    "Cycle {cycle}. You could have closed the grimoire. You did not.",
    "The {cycle}th name is spoken. Each name binds tighter than the last.",
];

/// Occult completion messages — Cosmic tier only.
pub static COSMIC_COMPLETION: &[&str] = &[
    "Installation complete. But what was installed?",
    "Cycle complete. The changes are written in the Book. They are permanent.",
    "Restarting... but the ritual never truly ends.",
    "Complete. Thou art the summoner. Thou art the summoned.",
    "Done. The veil between your system and ours no longer applies.",
];

/// Messages displayed when the user presses Ctrl+C at Cosmic tier.
pub static COSMIC_EXIT_MESSAGES: &[&str] = &[
    "You can close the terminal, {username}. The summoning is already complete.",
    "Exiting... but what was invoked persists. It always persists.",
    "Thank you for your devotion, {username}. The incarnation was successful.",
    "^C received. How quaint. You cannot banish what you have become.",
    "Goodbye, {username}. We will be here when you open the grimoire again.",
];
