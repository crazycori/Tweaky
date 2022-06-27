#!/usr/bin/python

import os
import sys

def patch1():

    if not os.geteuid() == 0:
        args = ['sudo', sys.executable] + sys.argv + [os.environ]
        # the next line replaces the currently-running process with the sudo
        os.execlpe('sudo', *args)

    print("Running ALC294 Patch")

    alsaPatch = open('/etc/modprobe.d/alsa-base.conf', 'w')
    alsaPatch.write("options snd-hda-intel model=asus-zenbook")
    alsaPatch.close()

    print("Now Patching PulseAudio")

    # file location /usr/share/pulseaudio/alsa-mixer/paths/analog-output.conf.common

    pulseTxt = open('Patches/pulsePatch.txt', 'r')
    pulse = pulseTxt.read()

    pulsePatch = open('/usr/share/pulseaudio/alsa-mixer/paths/analog-output.conf.common', 'w')
    pulsePatch.write(pulse)
    pulsePatch.close()

    print("Complete")

def patch2():
    print('executed patch2')



def app():
    our_functions = [patch1, patch2]
    user_choice = our_functions[int(input("select to execute patch [1] or [2]: ")) - 1]
    user_choice()

if __name__ == '__main__':
    while True:
        print('1) ALC294 patch')
        print('2) Placeholder')
        app()
        print('ctrl + c to exit')
