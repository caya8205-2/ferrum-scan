use std::io::{self, Write};

pub fn handle_uninstall(skip_confirmation: bool) {
    if !skip_confirmation {
        println!("⚠️  This will remove ferrum-scan from your system.");
        print!("Are you sure you want to proceed? [y/N]: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("❌ Failed to read input. Uninstallation cancelled.");
            return;
        }

        let answer = input.trim().to_lowercase();
        if !answer.starts_with('y') {
            println!("❌ Uninstallation cancelled.");
            return;
        }
    }

    println!("🗑️  Uninstalling ferrum-scan...");

    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let app_dir = format!(r"{}\ferrum-scan", local_app_data);

            // Remove PATH entry using PowerShell
            let ps_script = format!(
                "$p=[Environment]::GetEnvironmentVariable('PATH','User');$p=($p -split ';'|?{{$_ -ne '{}'}})-join';';[Environment]::SetEnvironmentVariable('PATH',$p,'User')",
                app_dir.replace('\'', "''")
            );

            let _ = std::process::Command::new("powershell")
                .args(["-Command", &ps_script])
                .output();

            // Self-delete batch cleanup
            let temp_dir = std::env::temp_dir();
            let bat_path = temp_dir.join("ferrum-scan-cleanup.bat");
            let bat_content = format!(
                "@echo off\r\nping 127.0.0.1 -n 2 > nul\r\nif exist \"{}\" rmdir /s /q \"{}\"\r\ndel \"%~f0\"\r\n",
                app_dir, app_dir
            );

            if std::fs::write(&bat_path, bat_content).is_ok() {
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "/B", "", bat_path.to_str().unwrap()])
                    .spawn();
            }
        }
        println!("✅ ferrum-scan has been removed from PATH and will be cleaned up after this terminal window closes.");
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(home) = std::env::var("HOME") {
            let app_dir = format!("{}/.ferrum-scan", home);
            let _ = std::fs::remove_dir_all(app_dir);
        }
        println!("✅ ferrum-scan uninstalled successfully.");
    }
}
