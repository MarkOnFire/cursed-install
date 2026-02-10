use chrono::Local;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ScanResult {
    // System identity
    pub hostname: Option<String>,
    pub os_name: Option<String>,
    pub username: Option<String>,

    // File system overview
    pub home_dir: Option<PathBuf>,
    pub desktop_count: Option<usize>,
    pub downloads_count: Option<usize>,
    pub documents_exists: bool,

    // Development artifacts
    pub project_names: Vec<String>,
    pub git_repos: Vec<String>,
    pub dotfile_names: Vec<String>,

    // Security-sensitive indicators
    pub ssh_key_names: Vec<String>,
    pub browser_profiles: Vec<String>,
    pub cloud_configs: Vec<String>,
    pub env_file_count: usize,
    pub shell_history_lines: Option<usize>,
    pub docker_present: bool,

    // Metadata
    pub files_scanned: usize,
    pub scan_timestamp: String,
}

pub fn scan() -> ScanResult {
    let hostname = get_hostname();
    let os_name = get_os_name();
    let username = get_username();
    let home_dir = dirs_home();

    let mut files_scanned: usize = 0;

    let desktop_count = home_dir
        .as_ref()
        .and_then(|h| count_dir_entries(&h.join("Desktop"), &mut files_scanned));
    let downloads_count = home_dir
        .as_ref()
        .and_then(|h| count_dir_entries(&h.join("Downloads"), &mut files_scanned));
    let documents_exists = home_dir
        .as_ref()
        .map(|h| h.join("Documents").is_dir())
        .unwrap_or(false);

    let dotfile_names = home_dir
        .as_ref()
        .map(|h| list_dotfiles(h, &mut files_scanned))
        .unwrap_or_default();

    let (project_names, git_repos, env_file_count) = home_dir
        .as_ref()
        .map(|h| scan_project_dirs(h, &mut files_scanned))
        .unwrap_or_default();

    let ssh_key_names = home_dir
        .as_ref()
        .map(|h| scan_ssh_keys(&h.join(".ssh"), &mut files_scanned))
        .unwrap_or_default();

    let browser_profiles = home_dir
        .as_ref()
        .map(|h| detect_browsers(h))
        .unwrap_or_default();

    let cloud_configs = home_dir
        .as_ref()
        .map(|h| detect_cloud_configs(h))
        .unwrap_or_default();

    let shell_history_lines = home_dir
        .as_ref()
        .and_then(|h| estimate_history_lines(h));

    let docker_present = home_dir
        .as_ref()
        .map(|h| h.join(".docker").is_dir())
        .unwrap_or(false);

    let scan_timestamp = Local::now().format("%H:%M:%S").to_string();

    ScanResult {
        hostname,
        os_name,
        username,
        home_dir,
        desktop_count,
        downloads_count,
        documents_exists,
        project_names,
        git_repos,
        dotfile_names,
        ssh_key_names,
        browser_profiles,
        cloud_configs,
        env_file_count,
        shell_history_lines,
        docker_present,
        files_scanned,
        scan_timestamp,
    }
}

fn get_hostname() -> Option<String> {
    sysinfo::System::host_name()
}

fn get_os_name() -> Option<String> {
    sysinfo::System::long_os_version()
}

fn get_username() -> Option<String> {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .ok()
}

fn dirs_home() -> Option<PathBuf> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
        .filter(|p| p.is_dir())
}

fn count_dir_entries(path: &Path, scanned: &mut usize) -> Option<usize> {
    let entries = fs::read_dir(path).ok()?;
    let count = entries.filter_map(|e| e.ok()).count();
    *scanned += count;
    Some(count)
}

fn list_dotfiles(home: &Path, scanned: &mut usize) -> Vec<String> {
    let mut dotfiles = Vec::new();
    if let Ok(entries) = fs::read_dir(home) {
        for entry in entries.filter_map(|e| e.ok()) {
            *scanned += 1;
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with('.') && name.len() > 1 && name != ".DS_Store" {
                    dotfiles.push(name.to_string());
                }
            }
        }
    }
    dotfiles
}

fn scan_project_dirs(
    home: &Path,
    scanned: &mut usize,
) -> (Vec<String>, Vec<String>, usize) {
    let mut project_names = Vec::new();
    let mut git_repos = Vec::new();
    let mut env_count: usize = 0;

    let candidate_dirs = ["Developer", "Projects", "repos", "code", "src"];

    for dir_name in &candidate_dirs {
        let dir = home.join(dir_name);
        if !dir.is_dir() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                *scanned += 1;
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }

                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    project_names.push(name.to_string());

                    if path.join(".git").exists() {
                        git_repos.push(name.to_string());
                    }

                    if path.join(".env").exists() {
                        env_count += 1;
                    }
                }
            }
        }
    }

    project_names.sort();
    project_names.dedup();
    git_repos.sort();
    git_repos.dedup();

    (project_names, git_repos, env_count)
}

fn scan_ssh_keys(ssh_dir: &Path, scanned: &mut usize) -> Vec<String> {
    let mut keys = Vec::new();
    if let Ok(entries) = fs::read_dir(ssh_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            *scanned += 1;
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("id_") || name.ends_with(".pub") || name == "authorized_keys"
                {
                    keys.push(name.to_string());
                }
            }
        }
    }
    keys
}

fn detect_browsers(home: &Path) -> Vec<String> {
    let mut found = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let app_support = home.join("Library").join("Application Support");
        let checks = [
            ("Chrome", app_support.join("Google").join("Chrome")),
            ("Firefox", app_support.join("Firefox")),
            ("Safari", home.join("Library").join("Safari")),
            ("Arc", app_support.join("Arc")),
            ("Brave", app_support.join("BraveSoftware")),
        ];
        for (name, path) in &checks {
            if path.is_dir() {
                found.push(name.to_string());
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let config = home.join(".config");
        let checks = [
            ("Chrome", config.join("google-chrome")),
            ("Firefox", home.join(".mozilla").join("firefox")),
            ("Brave", config.join("BraveSoftware")),
            ("Chromium", config.join("chromium")),
        ];
        for (name, path) in &checks {
            if path.is_dir() {
                found.push(name.to_string());
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(appdata_local) = env::var("LOCALAPPDATA").ok().map(PathBuf::from) {
            let appdata_roaming = env::var("APPDATA")
                .ok()
                .map(PathBuf::from)
                .unwrap_or_default();
            let checks = [
                ("Chrome", appdata_local.join("Google").join("Chrome")),
                ("Firefox", appdata_roaming.join("Mozilla").join("Firefox")),
                ("Brave", appdata_local.join("BraveSoftware")),
                ("Edge", appdata_local.join("Microsoft").join("Edge")),
            ];
            for (name, path) in &checks {
                if path.is_dir() {
                    found.push(name.to_string());
                }
            }
        }
    }

    found
}

fn detect_cloud_configs(home: &Path) -> Vec<String> {
    let mut found = Vec::new();

    let mut checks: Vec<(&str, PathBuf)> = vec![
        ("AWS", home.join(".aws")),
        ("Azure", home.join(".azure")),
        ("Kubernetes", home.join(".kube")),
        ("Terraform", home.join(".terraform.d")),
    ];

    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            checks.push(("gcloud", PathBuf::from(appdata).join("gcloud")));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        checks.push(("gcloud", home.join(".config").join("gcloud")));
    }

    for (name, path) in &checks {
        if path.is_dir() {
            found.push(name.to_string());
        }
    }

    found
}

fn estimate_history_lines(home: &Path) -> Option<usize> {
    let history_files = [
        home.join(".zsh_history"),
        home.join(".bash_history"),
        home.join(".local").join("share").join("fish").join("fish_history"),
    ];

    for path in &history_files {
        if let Ok(meta) = fs::metadata(path) {
            let bytes = meta.len();
            // Rough estimate: ~50 bytes per history line
            return Some((bytes as usize) / 50);
        }
    }

    None
}
