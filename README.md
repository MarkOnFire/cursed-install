# cursed-install

A terminal application that simulates installing things. It doesn't actually install anything.

But it might know more about you than you'd expect.

[![asciicast](https://asciinema.org/a/757039.svg)](https://asciinema.org/a/757039)

## Installation

### Download binary

Grab the latest binary for your platform from [Releases](https://github.com/MarkOnFire/cursed-install/releases)

```bash
chmod +x cursed-install-*
./cursed-install-linux-x86_64
```

### Build from source

```bash
cargo run --release
```

Press Ctrl+C to stop. Or try to.

### Pick what to install

By default we install everything. But you can change this behavior.
```bash
# Install specific stages
cargo run --release -- kernel
```

Or pick what not to install.
```bash
# Exclude specific stages from installation
cargo run --release -- --exclude cloud xorg
```

See available stages:
```bash
cargo run --release -- --help
```

## Awareness Mode

By default, the installer scans your filesystem at startup and uses what it finds to generate progressively unsettling messages. The longer you let it run, the more personal it gets.

This serves as a (humorous) reminder of what any program you run can learn about you — just from filenames, directory listings, and existence checks. It never reads file contents.

### Escalation tiers

The installer's tone escalates across installation cycles:

| Cycle | Tier | Tone |
|-------|------|------|
| 1 | Baseline | Normal (classic behavior) |
| 2-3 | Ambient | "The installer knows your hardware" |
| 4-5 | Familiar | "It knows your projects" |
| 6-8 | Invasive | "It's talking about your SSH keys" |
| 9+ | Cosmic | "Reality breaks" |

### Disabling awareness mode

If you prefer the classic experience without filesystem scanning:

```bash
cargo run --release -- --normal
```

Or with the short flag:

```bash
cargo run --release -- -n
```

## Privacy & Security

Awareness mode only accesses:

- **System identity**: hostname, OS name, username (from environment variables)
- **Directory listings**: file counts in Desktop/Downloads, project directory names, dotfile names
- **Existence checks**: whether certain directories exist (e.g., `.ssh`, `.aws`, `.docker`, browser profile dirs)
- **File metadata**: shell history file size (to estimate line count)

It **never**:

- Reads file contents
- Opens or parses any file
- Accesses network resources
- Writes to disk
- Sends data anywhere

All scan results stay in memory for the duration of the process and are discarded on exit.

## Docker

Build
```bash
docker build -t cursed-install .
```

Run
```bash
docker run -it --rm --init cursed-install
```

Note: awareness mode has limited data in Docker containers (no home directory, no browser profiles, etc.). The installer gracefully falls back to generic messages.

## License

Do whatever you want with it. Well, except for movies. If you use this in a movie, credit me or something.
