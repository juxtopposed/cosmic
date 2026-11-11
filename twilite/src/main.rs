use serde::Deserialize;
use std::{
    fs,
    process::Command,
    thread,
    time::Duration,
};

#[derive(Deserialize)]
struct Config {
    light: String,
    dark: String,
}

fn config_path() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(".config/twilite/config.toml")
}

fn load_config() -> Config {
    let path = config_path();

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!(
                "\nTwilite\n\nConfiguration file not found:\n{}\n",
                path.display()
            );
            std::process::exit(1);
        }
    };

    match toml::from_str(&content) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("\nInvalid config.toml\n\n{}\n", err);
            std::process::exit(1);
        }
    }
}

fn is_dark() -> bool {
    let out = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
        .expect("Failed to run gsettings");

    String::from_utf8_lossy(&out.stdout).contains("dark")
}

fn ensure_swww() {
    let out = Command::new("pgrep")
        .arg("swww-daemon")
        .output()
        .expect("Failed to check for swww-daemon");

    if out.stdout.is_empty() {
        if Command::new("swww-daemon").spawn().is_err() {
            eprintln!(
                "\nTwilite\n\nCouldn't start swww-daemon.\nPlease make sure swww is installed.\n"
            );
            std::process::exit(1);
        }

        thread::sleep(Duration::from_millis(500));
    }
}

fn set_wallpaper(path: &str) {
    let status = Command::new("swww")
        .args(["img", path])
        .status();

    if status.is_err() {
        eprintln!("Failed to set wallpaper.");
    }
}

fn main() {
    println!("Twilite started.");

    ensure_swww();

    let mut last_theme = None;

    loop {
        let config = load_config();

        let dark = is_dark();

        if last_theme != Some(dark) {
            if dark {
                println!("Applying dark wallpaper...");
                set_wallpaper(&config.dark);
            } else {
                println!("Applying light wallpaper...");
                set_wallpaper(&config.light);
            }

            last_theme = Some(dark);
        }

        thread::sleep(Duration::from_secs(2));
    }
}