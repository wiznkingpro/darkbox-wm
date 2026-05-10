// src/keybindings.rs
use crate::config::{Action, get_bindings};
use crate::compositor::DarkBox;

pub fn handle_key(compositor: &mut DarkBox, key: u32, modifiers: u32) {
    for binding in get_bindings() {
        if binding.mods == modifiers && binding.key == key {
            match binding.action {
                Action::SpawnTerminal => {
                    let _ = std::process::Command::new(crate::config::TERMINAL)
                        .spawn();
                }
                Action::FocusNext => {
                    if compositor.focused + 1 < compositor.windows.len() {
                        compositor.focused += 1;
                    }
                }
                Action::FocusPrev => {
                    if compositor.focused > 0 {
                        compositor.focused -= 1;
                    }
                }
                Action::ResizeHoriz(_delta) => {
                    // TODO: implement resize
                }
                Action::CloseWindow => {
                    if let Some(window) = compositor.windows.get(compositor.focused) {
                        window.close();
                    }
                }
                Action::Quit => {
                    std::process::exit(0);
                }
            }
        }
    }
}