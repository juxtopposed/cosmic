use serde::Deserialize;
use std::{
    fs,
    process::Command,
    thread,
    time::Duration,
};

const VALID_TRANSITION_TYPES: [&str; 9] = [
    "simple",
    "left",
    "right",
    "top",
    "bottom",
    "center",
    "outer",
    "any",
    "random",
];

fn default_transition_type() -> String {
    "simple".to_string()
}

fn default_transition_step() -> u8 {
    2
}

fn default_transition_fps() -> u8 {
    30
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Config {
    light: String,
    dark: String,
    #[serde(default = "default_transition_type")]
    transition_type: String,
    #[serde(default = "default_transition_step")]
    transition_step: u8,
    #[serde(default = "default_transition_fps")]
    transition_fps: u8,
}

impl Config {
    fn validate(&self) -> Result<(), String> {
        if !VALID_TRANSITION_TYPES.contains(&self.transition_type.as_str()) {
            return Err(format!(
                "transition_type must be one of: {}",
                VALID_TRANSITION_TYPES.join(", ")
            ));
        }

        if !(1..=255).contains(&self.transition_step) {
            return Err("transition_step must be between 1 and 255".to_string());
        }

        if !(1..=255).contains(&self.transition_fps) {
            return Err("transition_fps must be between 1 and 255".to_string());
        }

        Ok(())
    }
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

    match toml::from_str::<Config>(&content) {
        Ok(cfg) => {
            if let Err(err) = cfg.validate() {
                eprintln!("\nInvalid config.toml\n\n{}\n", err);
                std::process::exit(1);
            }
            cfg
        }
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

fn ensure_awww() {
    let out = Command::new("pgrep")
        .args(["-x", "awww-daemon"])
        .output()
        .expect("Failed to check for awww-daemon");

    if out.stdout.is_empty() {
        match Command::new("awww-daemon").spawn() {
            Ok(_) => {
                thread::sleep(Duration::from_millis(500));
            }
            Err(err) => {
                eprintln!(
                    "\nTwilite\n\nCouldn't start awww-daemon.\nPlease make sure awww is installed.\nReason: {}\n",
                    err
                );
                std::process::exit(1);
            }
        }
    }
}

fn wallpaper_args(path: &str, config: &Config) -> Vec<String> {
    vec![
        "img".to_string(),
        path.to_string(),
        "--transition-type".to_string(),
        config.transition_type.clone(),
        "--transition-step".to_string(),
        config.transition_step.to_string(),
        "--transition-fps".to_string(),
        config.transition_fps.to_string(),
    ]
}

fn set_wallpaper(path: &str, config: &Config) {
    let mut command = Command::new("awww");
    command.args(wallpaper_args(path, config));

    match command.status() {
        Ok(status) if status.success() => {}
        Ok(status) => {
            eprintln!(
                "Failed to set wallpaper with awww (exit status: {}).",
                status
            );
            std::process::exit(1);
        }
        Err(err) => {
            eprintln!("Failed to set wallpaper with awww: {}", err);
            std::process::exit(1);
        }
    }
}

fn main() {
    println!("Twilite started.");

    ensure_awww();

    let mut last_theme = None;

    loop {
        let config = load_config();

        let dark = is_dark();

        if last_theme != Some(dark) {
            if dark {
                println!("Applying dark wallpaper...");
                set_wallpaper(&config.dark, &config);
            } else {
                println!("Applying light wallpaper...");
                set_wallpaper(&config.light, &config);
            }

            last_theme = Some(dark);
        }

        thread::sleep(Duration::from_secs(2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_legacy_config_without_transition_settings() {
        let config: Config = toml::from_str(
            r#"
            light = "/tmp/light.png"
            dark = "/tmp/dark.png"
            "#,
        )
        .unwrap();

        assert_eq!(config.light, "/tmp/light.png");
        assert_eq!(config.dark, "/tmp/dark.png");
        assert_eq!(config.transition_type, "simple");
        assert_eq!(config.transition_step, 2);
        assert_eq!(config.transition_fps, 30);
        config.validate().unwrap();
    }

    #[test]
    fn builds_awww_transition_args() {
        let config: Config = toml::from_str(
            r#"
            light = "/tmp/light.png"
            dark = "/tmp/dark.png"
            transition_type = "center"
            transition_step = 11
            transition_fps = 24
            "#,
        )
        .unwrap();

        assert_eq!(
            wallpaper_args("/tmp/wallpaper.png", &config),
            vec![
                "img",
                "/tmp/wallpaper.png",
                "--transition-type",
                "center",
                "--transition-step",
                "11",
                "--transition-fps",
                "24",
            ]
        );
    }

    #[test]
    fn rejects_invalid_transition_type() {
        let config: Config = toml::from_str(
            r#"
            light = "/tmp/light.png"
            dark = "/tmp/dark.png"
            transition_type = "invalid"
            transition_step = 11
            transition_fps = 24
            "#,
        )
        .unwrap();

        let err = config.validate().unwrap_err();
        assert!(err.contains("transition_type"));
    }

    #[test]
    fn rejects_out_of_range_transition_values() {
        let err = toml::from_str::<Config>(
            r#"
            light = "/tmp/light.png"
            dark = "/tmp/dark.png"
            transition_type = "simple"
            transition_step = 0
            transition_fps = 300
            "#,
        )
        .unwrap_err();

        assert!(err.to_string().contains("u8") || err.to_string().contains("expected u8"));
    }
}
