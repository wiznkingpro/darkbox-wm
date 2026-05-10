use smithay::input::keyboard::Keysym;
use super::config::*;
use super::compositor::DarkBox;

pub fn handle_key(compositor: &mut DarkBox, keysym: Keysym, modifiers: u32) {
    for (mods, key, action) in KEYBINDINGS {
        if *mods == modifiers && *key == keysym {
            match action {
                Action::SpawnTerminal => {
                    let _ = std::process::Command::new(TERMINAL)
                        .spawn();
                }
                Action::FocusNext => {
                    if compositor.focused + 1 < compositor.windows.len() {
                        compositor.focused += 1;
                        // обновить фокус ввода
                    }
                }
                Action::FocusPrev => {
                    if compositor.focused > 0 {
                        compositor.focused -= 1;
                    }
                }
                Action::ResizeHoriz(delta) => {
                    if let Some(window) = compositor.windows.get_mut(compositor.focused) {
                        let current = window.geometry();
                        // Здесь нужно правильно изменить размер через Wayland
                        // Для простоты оставляем заглушку
                    }
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