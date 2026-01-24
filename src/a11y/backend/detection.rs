//! Screen reader backend - platform detection helpers

/// Check if VoiceOver is running on macOS
pub(crate) fn is_voiceover_running() -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        // Check if VoiceOver process is running
        if let Ok(output) = Command::new("pgrep").arg("-x").arg("VoiceOver").output() {
            return output.status.success();
        }

        // Alternative: check defaults
        if let Ok(output) = Command::new("defaults")
            .arg("read")
            .arg("com.apple.universalaccess")
            .arg("voiceOverOnOffKey")
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                return stdout.trim() == "1";
            }
        }
    }

    false
}

/// Check if a Windows screen reader is running
pub(crate) fn is_windows_screen_reader_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // Check for common screen readers
        let screen_readers = ["nvda", "narrator", "jfw"]; // JAWS

        for sr in &screen_readers {
            if let Ok(output) = Command::new("tasklist")
                .arg("/FI")
                .arg(format!("IMAGENAME eq {}.exe", sr))
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    if stdout.contains(sr) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Check if AT-SPI is available on Linux
pub(crate) fn is_atspi_available() -> bool {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        // Check if speech-dispatcher is available
        if let Ok(output) = Command::new("which").arg("spd-say").output() {
            if output.status.success() {
                return true;
            }
        }

        // Check for Orca
        if let Ok(output) = Command::new("pgrep").arg("-x").arg("orca").output() {
            return output.status.success();
        }

        // Check D-Bus for AT-SPI registry
        if let Ok(output) = Command::new("dbus-send")
            .arg("--session")
            .arg("--print-reply")
            .arg("--dest=org.a11y.Bus")
            .arg("/org/a11y/bus")
            .arg("org.freedesktop.DBus.Properties.Get")
            .arg("string:org.a11y.Status")
            .arg("string:IsEnabled")
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                return stdout.contains("true");
            }
        }
    }

    false
}
