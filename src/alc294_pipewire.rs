use std::fs;


pub fn alc294_patch() -> std::io::Result<()> {

    fs::write("/etc/modprobe.d/alsa-base.conf",
              b"options snd-hda-intel model=alc294-lenovo-mic")?;
    Ok(())
}