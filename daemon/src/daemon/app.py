# Copyright (c) 2026 Simon-Weij
# 
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

import evdev
import json
import os
import pwd
import subprocess

pressed_keys = set()

def load_settings():
    user = None
    try:
        result = subprocess.run(['who'], capture_output=True, text=True, check=True)
        user = result.stdout.strip().split('\n')[0].split()[0]
    except:
        pass
    user = user or os.environ.get('SUDO_USER') or os.environ.get('USER')
    
    try:
        home = pwd.getpwnam(user).pw_dir if user else os.path.expanduser("~")
    except KeyError:
        home = os.path.expanduser("~")
    
    config_dir = os.path.join(home, ".config", "flux")
    settings_path = os.path.join(config_dir, "settings.json")
        
    if not os.path.exists(settings_path):
        return {
            "backend": "gpu-screen-recorder",
            "clip_length": 30,
            "clip_hotkey": ["KEY_LEFTALT", "KEY_Z"],
            "window": "screen",
            "framerate": 60,
            "replay_time": 30,
            "container": "mp4",
            "output": "~/Videos/clip",
            "codec": "h264",
            "quality": 20,
            "framerate_mode": "vfr",
            "bitrate_mode": "cqp"
        }
    
    with open(settings_path, 'r') as f:
        data = json.load(f)
        return data

def find_keyboard():
    devices = [evdev.InputDevice(path) for path in evdev.list_devices()]
    keyboards = []
    for dev in devices:
        caps = dev.capabilities()
        if evdev.ecodes.EV_KEY in caps:
            if "keyboard" in dev.name.lower() or "at trans" in dev.name.lower():
                keyboards.append(dev)
    if not keyboards:
        raise RuntimeError("No keyboard found")
    return keyboards[0]

def keypress(event):
    global pressed_keys
    keycode = evdev.ecodes.KEY[event.code]
    
    if event.value == 1:
        pressed_keys.add(keycode)
    elif event.value == 0:
        pressed_keys.discard(keycode)
    
    settings = load_settings()
    clip_hotkey = set(settings.get("clip_hotkey", []))
    
    
    if pressed_keys == clip_hotkey and event.value >= 1:
        print("\033[91mHello, World!\033[0m")

def main():
    keyboard = find_keyboard()
    print(f"Using keyboard: {keyboard.name} at {keyboard.path}")

    settings = load_settings()
    backend = settings.get("backend", "gpu-screen-recorder")
    
    if backend == "gpu-screen-recorder":
        window = settings.get("window", "screen")
        framerate = settings.get("framerate", 60)
        replay_time = settings.get("replay_time", 30)
        container = settings.get("container", "mp4")
        output = os.path.expanduser(settings.get("output", "~/Videos/clip"))
        codec = settings.get("codec", "h264")
        quality = settings.get("quality", 20)
        framerate_mode = settings.get("framerate_mode", "vfr")
        bitrate_mode = settings.get("bitrate_mode", "cqp")
        
        cmd = [
            'gpu-screen-recorder',
            '-w', window,
            '-f', str(framerate),
            '-r', str(replay_time),
            '-c', container,
            '-o', output,
            '-a', "default_output",
            '-k', codec,
            '-q', str(quality),
            '-fm', framerate_mode,
            '-bm', bitrate_mode
        ]
        print(f"Starting recorder with command: {cmd}")
        subprocess.run(cmd)
    else:
        print(f"Backend {backend} not supported")

if __name__ == "__main__":
    main()