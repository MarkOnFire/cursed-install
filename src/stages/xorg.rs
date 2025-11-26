use super::InstallationStage;
use crate::log_generator::LogGenerator;
use crate::ui::Spinner;
use colored::*;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct XorgStage;

impl XorgStage {
    fn load_xorg_modules(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        exit_check: &dyn Fn() -> bool,
    ) -> io::Result<()> {
        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Loading X server modules...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(400));

        let modules = [
            ("fb", "Framebuffer support"),
            ("dbe", "Double buffer extension"),
            ("glx", "OpenGL extension"),
            ("dri2", "Direct Rendering Infrastructure v2"),
            ("extmod", "Extension modules"),
            ("int10", "x86 emulation"),
            ("vbe", "VESA BIOS extensions"),
            ("record", "Record extension"),
        ];

        for (module, desc) in &modules {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            print!(
                "{}   [*] {} ",
                LogGenerator::timestamp().dimmed(),
                module.bright_white()
            );
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rng.gen_range(120..280)));
            println!("{}", format!("({})", desc).dimmed());
        }

        Ok(())
    }

    fn initialize_glx(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        _exit_check: &dyn Fn() -> bool,
    ) -> io::Result<()> {
        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Initializing GLX (OpenGL Extension)...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(rng.gen_range(400..700)));

        let gl_versions = ["4.6", "4.5", "4.3"];
        let gl_version = gl_versions[rng.gen_range(0..gl_versions.len())];

        println!(
            "{}   ├─ GLX version: {}",
            LogGenerator::timestamp().dimmed(),
            "1.4".bright_white()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   ├─ OpenGL version: {}",
            LogGenerator::timestamp().dimmed(),
            gl_version.bright_white()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   ├─ Mesa driver: {}",
            LogGenerator::timestamp().dimmed(),
            "24.1.7".bright_white()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   ├─ GLSL version: {}",
            LogGenerator::timestamp().dimmed(),
            "4.60".bright_white()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   └─ Direct rendering: {}",
            LogGenerator::timestamp().dimmed(),
            "Yes".bright_green()
        );
        thread::sleep(Duration::from_millis(300));

        Ok(())
    }

    fn probe_gpu(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        exit_check: &dyn Fn() -> bool,
    ) -> io::Result<&'static str> {
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Initializing PCI bus enumeration...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(rng.gen_range(300..600)));

        let gpu_configs = [
            (
                "Intel UHD Graphics 630",
                "i915",
                "00:02.0",
                "8086:9bc8",
                vec!["1920x1080", "2560x1440", "1680x1050"],
            ),
            (
                "NVIDIA GeForce RTX 3080",
                "nvidia",
                "01:00.0",
                "10de:2206",
                vec!["3840x2160", "2560x1440", "1920x1080"],
            ),
            (
                "AMD Radeon RX 6800 XT",
                "amdgpu",
                "01:00.0",
                "1002:73bf",
                vec!["3840x2160", "2560x1440", "1920x1080"],
            ),
            (
                "VirtualBox Graphics Adapter",
                "vboxvideo",
                "00:02.0",
                "80ee:beef",
                vec!["1920x1080", "1280x720"],
            ),
        ];

        let (gpu_name, driver, pci_slot, device_id, modes) =
            &gpu_configs[rng.gen_range(0..gpu_configs.len())];

        println!(
            "{}   └─ Scanning PCI device {}",
            LogGenerator::timestamp().dimmed(),
            pci_slot.bright_white()
        );
        thread::sleep(Duration::from_millis(250));

        println!(
            "{}      └─ Device ID: {} {}",
            LogGenerator::timestamp().dimmed(),
            device_id.bright_white(),
            "[VGA compatible controller]".dimmed()
        );
        thread::sleep(Duration::from_millis(200));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("  Detected: {}", gpu_name).bright_green().bold()
        );
        thread::sleep(Duration::from_millis(300));

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("Loading DRM/KMS driver: {}", driver).bright_cyan()
        );
        thread::sleep(Duration::from_millis(rng.gen_range(400..700)));

        println!(
            "{}   ├─ Initializing kernel mode setting (KMS)... {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(300));

        println!(
            "{}   ├─ Allocating framebuffer memory (256 MB)... {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(250));

        println!(
            "{}   ├─ Enabling DPMS (Display Power Management)... {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(200));

        println!(
            "{}   └─ GPU acceleration: {}",
            LogGenerator::timestamp().dimmed(),
            "Enabled".bright_green()
        );
        thread::sleep(Duration::from_millis(300));

        let vram = if driver == &"nvidia" || driver == &"amdgpu" {
            rng.gen_range(8..=16)
        } else if driver == &"vboxvideo" {
            128
        } else {
            rng.gen_range(4..=8)
        };
        let vram_unit = if vram >= 1024 { "GB" } else { "MB" };
        let vram_display = if vram >= 1024 { vram / 1024 } else { vram };

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Querying video memory...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(400));
        println!(
            "{}   ├─ Total VRAM: {} {}",
            LogGenerator::timestamp().dimmed(),
            format!("{} {}", vram_display, vram_unit).bright_white(),
            format!("(dedicated)").dimmed()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   └─ Memory type: {}",
            LogGenerator::timestamp().dimmed(),
            if driver == &"nvidia" || driver == &"amdgpu" {
                "GDDR6"
            } else {
                "Shared"
            }
            .bright_white()
        );
        thread::sleep(Duration::from_millis(250));

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Enumerating display outputs...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(500));

        let outputs = if driver == &"nvidia" || driver == &"amdgpu" {
            vec!["DisplayPort-0", "HDMI-0", "DVI-D-0"]
        } else if driver == &"vboxvideo" {
            vec!["Virtual-1"]
        } else {
            vec!["eDP-1", "HDMI-1", "DP-1"]
        };

        for (i, output) in outputs.iter().enumerate() {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let connected = i == 0;
            let status = if connected {
                "Connected".bright_green()
            } else {
                "Disconnected".dimmed()
            };
            println!(
                "{}   ├─ {}: {}",
                LogGenerator::timestamp().dimmed(),
                output.bright_white(),
                status
            );

            if connected {
                let mode = modes[rng.gen_range(0..modes.len())];
                let refresh = if mode == "3840x2160" {
                    60
                } else if rng.gen_bool(0.7) {
                    144
                } else {
                    60
                };
                thread::sleep(Duration::from_millis(200));
                println!(
                    "{}   │  ├─ Preferred mode: {}@{}Hz",
                    LogGenerator::timestamp().dimmed(),
                    mode.bright_white(),
                    refresh
                );
                println!(
                    "{}   │  ├─ Color depth: {} {}",
                    LogGenerator::timestamp().dimmed(),
                    "24-bit".bright_white(),
                    "(TrueColor)".dimmed()
                );
                println!(
                    "{}   │  └─ EDID checksum: {}",
                    LogGenerator::timestamp().dimmed(),
                    "Valid".bright_green()
                );
            }
            thread::sleep(Duration::from_millis(150));
        }

        Ok(gpu_name)
    }

    fn load_extensions(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        exit_check: &dyn Fn() -> bool,
    ) -> io::Result<()> {
        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Loading X server extensions...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(400));

        let extensions = [
            ("MIT-SHM", "Shared memory support"),
            ("Composite", "Window composition"),
            ("RENDER", "Anti-aliased rendering"),
            ("RANDR", "Resolution and rotation"),
            ("GLX", "OpenGL acceleration"),
            ("XVideo", "Hardware video acceleration"),
            ("XINERAMA", "Multi-monitor support"),
            ("DRI3", "Direct Rendering Infrastructure"),
            ("Present", "Advanced frame presentation"),
            ("DAMAGE", "Screen damage notification"),
        ];

        for (ext, desc) in &extensions {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            print!(
                "{}   ├─ {} ",
                LogGenerator::timestamp().dimmed(),
                ext.bright_white()
            );
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rng.gen_range(100..300)));
            println!(
                "{} {}",
                "[LOADED]".bright_green(),
                format!("({})", desc).dimmed()
            );
        }
        thread::sleep(Duration::from_millis(200));

        Ok(())
    }

    fn detect_input_devices(
        &self,
        rng: &mut rand::rngs::ThreadRng,
        exit_check: &dyn Fn() -> bool,
    ) -> io::Result<()> {
        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Detecting input devices...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(500));

        let devices = [
            ("AT Translated Set 2 keyboard", "event0", "keyboard"),
            ("SynPS/2 Synaptics TouchPad", "event1", "touchpad"),
            ("Logitech USB Gaming Mouse", "event2", "mouse"),
            ("HD Pro Webcam C920", "event3", "camera"),
        ];

        for (i, (device, event, device_type)) in devices.iter().enumerate() {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let tree_char = if i == devices.len() - 1 {
                "└─"
            } else {
                "├─"
            };
            println!(
                "{}   {} /dev/input/{} → {}",
                LogGenerator::timestamp().dimmed(),
                tree_char,
                event.bright_white(),
                device.dimmed()
            );
            thread::sleep(Duration::from_millis(200));

            let sub_tree_char = if i == devices.len() - 1 { " " } else { "│" };
            println!(
                "{}   {}  └─ Driver: {} {}",
                LogGenerator::timestamp().dimmed(),
                sub_tree_char,
                "libinput".bright_white(),
                format!("[{}]", device_type).dimmed()
            );
            thread::sleep(Duration::from_millis(rng.gen_range(150..300)));
        }

        Ok(())
    }
}

impl InstallationStage for XorgStage {
    fn name(&self) -> &'static str {
        "X Window System Setup"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();
        let mut spinner = Spinner::new();

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Installing X.Org Server packages...".bright_white().bold()
        );
        thread::sleep(Duration::from_millis(400));
        println!();

        let packages = [
            ("xserver-xorg-core", "1.21.1-7"),
            ("xserver-xorg-input-libinput", "1.4.0-1"),
            ("xserver-xorg-input-wacom", "1.2.0-2"),
            ("xserver-xorg-video-intel", "2.99.917-9"),
            ("xserver-xorg-video-nouveau", "1.0.17-2"),
            ("xserver-xorg-video-amdgpu", "23.0.0-1"),
            ("libgl1-mesa-dri", "24.1.7-1"),
            ("libglx-mesa0", "24.1.7-1"),
            ("libgl1-mesa-glx", "24.1.7-1"),
            ("mesa-vulkan-drivers", "24.1.7-1"),
            ("xserver-xorg-legacy", "1.21.1-7"),
            ("xfonts-base", "1.0.5"),
        ];

        for (package, version) in &packages {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            println!(
                "{}   [+] {} {}",
                LogGenerator::timestamp().dimmed(),
                package.bright_white(),
                format!("({})", version).dimmed()
            );
            thread::sleep(Duration::from_millis(rng.gen_range(150..400)));
        }

        println!();
        spinner.animate(
            "Configuring X server security policies...",
            1200,
            exit_check,
        )?;

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "═══ Graphics Hardware Detection ═══".bright_yellow().bold()
        );
        thread::sleep(Duration::from_millis(300));
        println!();

        let _gpu_name = self.probe_gpu(&mut rng, exit_check)?;

        self.load_xorg_modules(&mut rng, exit_check)?;
        self.initialize_glx(&mut rng, exit_check)?;
        self.load_extensions(&mut rng, exit_check)?;
        self.detect_input_devices(&mut rng, exit_check)?;

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Configuring screen parameters...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(500));

        let dpi = rng.gen_range(90..=110);
        println!(
            "{}   ├─ Physical size: {} × {} mm",
            LogGenerator::timestamp().dimmed(),
            "508".bright_white(),
            "285".bright_white()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   ├─ DPI: {}",
            LogGenerator::timestamp().dimmed(),
            format!("{} × {}", dpi, dpi).bright_white()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   └─ Virtual size: {}",
            LogGenerator::timestamp().dimmed(),
            "3840 × 2160".bright_white()
        );
        thread::sleep(Duration::from_millis(300));

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Scanning font directories...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(400));

        let font_dirs = [
            "/usr/share/fonts/X11/misc",
            "/usr/share/fonts/X11/100dpi",
            "/usr/share/fonts/X11/75dpi",
            "/usr/share/fonts/X11/Type1",
            "/usr/share/fonts/truetype",
        ];

        for (i, dir) in font_dirs.iter().enumerate() {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            let tree_char = if i == font_dirs.len() - 1 {
                "└─"
            } else {
                "├─"
            };
            let font_count = rng.gen_range(12..156);
            println!(
                "{}   {} {} {} {}",
                LogGenerator::timestamp().dimmed(),
                tree_char,
                dir.bright_white(),
                format!("[{} fonts]", font_count).dimmed(),
                "[OK]".bright_green()
            );
            thread::sleep(Duration::from_millis(rng.gen_range(100..250)));
        }

        println!();
        spinner.animate("Building font cache (fc-cache)...", 1800, exit_check)?;

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Loading cursor theme...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(400));
        println!(
            "{}   └─ Theme: {} {}",
            LogGenerator::timestamp().dimmed(),
            "Adwaita".bright_white(),
            "(24px)".dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Writing configuration files...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(400));

        let config_files = [
            "/etc/X11/xorg.conf",
            "/etc/X11/xorg.conf.d/10-monitor.conf",
            "/etc/X11/xorg.conf.d/20-intel.conf",
            "/etc/X11/xorg.conf.d/40-libinput.conf",
            "/etc/X11/xorg.conf.d/50-synaptics.conf",
        ];

        for (i, file) in config_files.iter().enumerate() {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            let tree_char = if i == config_files.len() - 1 {
                "└─"
            } else {
                "├─"
            };
            print!(
                "{}   {} {} ",
                LogGenerator::timestamp().dimmed(),
                tree_char,
                file.bright_white()
            );
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rng.gen_range(200..400)));
            println!("{}", "[CREATED]".bright_green());
        }

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Generating xorg.conf sections...".bright_cyan()
        );
        thread::sleep(Duration::from_millis(600));

        println!(
            "{}   ├─ Section \"ServerLayout\" {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   │  └─ Setting default screen to 0",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(150));
        println!(
            "{}   ├─ Section \"InputDevice\" {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   │  ├─ Keyboard: CoreKeyboard",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(150));
        println!(
            "{}   │  └─ Pointer: CorePointer",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(150));
        println!(
            "{}   ├─ Section \"Monitor\" {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   │  └─ HorizSync: 30.0 - 83.0 kHz",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(150));
        println!(
            "{}   ├─ Section \"Device\" {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}   │  ├─ Option \"AccelMethod\" \"sna\"",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(150));
        println!(
            "{}   │  └─ Option \"TearFree\" \"true\"",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(150));
        println!(
            "{}   └─ Section \"Screen\" {}",
            LogGenerator::timestamp().dimmed(),
            "[OK]".bright_green()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{}      └─ DefaultDepth: 24",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "X Window System configured successfully!"
                .bright_green()
                .bold()
        );
        thread::sleep(Duration::from_millis(400));

        Ok(())
    }
}
