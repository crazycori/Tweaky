# Tweaky
Tool that applies automated patches for some linux issues

Tool requires root

This tool was primarily designed to fix issues I had with my own device. But should work with devices with similar hardware

Currently this application only applies a patch for the Realtek ALC294
Some ASUS laptops don't have audio via onboard speakers, or volume is stuck at 100%

This tool currently does not backup your existing config. Please make a backup before hand of:
/usr/share/pulseaudio/alsa-mixer/paths/analog-output.conf.common
/etc/modprobe.d/alsa-base.conf (If you don't have this file, it will be created)

This applies a simple patch to PulseAudio config and Alsa-Conf

/usr/share/pulseaudio/alsa-mixer/paths/analog-output.conf.common
The file is overwritten by Patches/pulsePatch.txt

/etc/modprobe.d/alsa-base.conf
"options snd-hda-intel model=asus-zenbook" is added
