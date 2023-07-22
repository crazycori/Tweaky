use std::fs;

fn backup_alsa() -> std::io::Result<()> {
    fs::rename("/etc/modprobe.d/alsa-base.conf", "/etc/modprobe.d/alsa-base.conf.bak")?;
    Ok(())
}

fn backup_pulse() -> std::io::Result<()> {
    fs::rename("/usr/share/pulseaudio/alsa-mixer/paths/analog-output.conf.common", "/usr/share/pulseaudio/alsa-mixer/paths/analog-output.conf.common.bak")?;
    Ok(())
}

