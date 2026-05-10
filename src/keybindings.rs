use crate::config::{Action, get_bindings};
use crate::compositor::DarkBox;

pub fn handle_key(compositor: &mut DarkBox, key: u32, modifiers: u32) {
    println!("Key pressed: {} with mods: {}", key, modifiers);
    
    for binding in get_bindings() {
        if binding.mods == modifiers && binding.key == key {
            match binding.action {
                Action::SpawnTerminal => {
                    let _ = std::process::Command::new(crate::config::TERMINAL)
                        .spawn();
                }
                Action::FocusNext => {
                    // TODO: implement
                }
                Action::FocusPrev => {
                    // TODO: implement
                }
                Action::ResizeHoriz(_delta) => {
                    // TODO: implement
                }
                Action::CloseWindow => {
                    // TODO: implement
                }
                Action::Quit => {
                    std::process::exit(0);
                }
            }
        }
    }
}