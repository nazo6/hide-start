#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::os::windows::process::CommandExt;

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    cmd: String,
}

fn main() {
    match execute() {
        Ok(_) => {}
        Err(e) => {
            let msg = format!("{:#}", e);
            #[cfg(debug_assertions)]
            {
                println!("{}", msg);
            }
            let dialog = native_dialog::MessageDialog::new()
                .set_title("Error")
                .set_text(&msg);
            dialog.show_alert().unwrap();
        }
    }
}

fn execute() -> Result<(), anyhow::Error> {
    let mut exe_path = std::env::current_exe()?;
    exe_path.set_extension("toml");
    let config_path = exe_path;
    let config = std::fs::read_to_string(&config_path).context(format!(
        "Could not find config: {}",
        config_path.to_string_lossy()
    ))?;
    let config = toml::from_str::<Config>(&config)?;

    let mut args = config.cmd.split_whitespace().collect::<Vec<_>>();
    let cmd = args.remove(0);
    std::process::Command::new(cmd)
        .args(args)
        .creation_flags(0x08000000)
        .spawn()
        .context("Failed to spawn command")?;

    Ok(())
}
