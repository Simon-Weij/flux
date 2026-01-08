// Copyright (c) 2026 Simon-Weij
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

import { register, unregisterAll } from "@tauri-apps/plugin-global-shortcut";
import { invoke } from "@tauri-apps/api/core";

interface ShortcutEvent {
  state: "Pressed" | "Released";
  shortcut: string;
}

export async function setupGlobalShortcut() {
  try {
    await unregisterAll();

    const clipHotkey = localStorage.getItem("clipHotkey") || "Alt+Z";
    await register(clipHotkey, async (event: ShortcutEvent) => {
      if (event.state === "Pressed") {
        console.log("Shortcut triggered");

        try {
          await invoke("run_terminal_command", {
            command: "notify-send hi",
          });
        } catch (error) {
          console.error("Failed to send notification:", error);
        }
      }
    });
    console.log("Shortcut registered:", clipHotkey);
  } catch (error) {
    console.error("Failed to register shortcut:", error);
  }
}
