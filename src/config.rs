pub const MODKEY: u32 = 0x40; // Super key

pub const KEY_ENTER: u32 = 36;
pub const KEY_J: u32 = 44;
pub const KEY_K: u32 = 45;
pub const KEY_H: u32 = 43;
pub const KEY_L: u32 = 46;
pub const KEY_C: u32 = 54;
pub const KEY_Q: u32 = 24;

pub const SHIFT_MASK: u32 = 1;

pub enum Action {
    SpawnTerminal,
    FocusNext,
    FocusPrev,
    ResizeHoriz(i32),
    CloseWindow,
    Quit,
}

pub struct Binding {
    pub mods: u32,
    pub key: u32,
    pub action: Action,
}

pub fn get_bindings() -> Vec<Binding> {
    vec![
        Binding { mods: MODKEY, key: KEY_ENTER, action: Action::SpawnTerminal },
        Binding { mods: MODKEY, key: KEY_J, action: Action::FocusNext },
        Binding { mods: MODKEY, key: KEY_K, action: Action::FocusPrev },
        Binding { mods: MODKEY, key: KEY_H, action: Action::ResizeHoriz(-50) },
        Binding { mods: MODKEY, key: KEY_L, action: Action::ResizeHoriz(50) },
        Binding { mods: MODKEY | SHIFT_MASK, key: KEY_C, action: Action::CloseWindow },
        Binding { mods: MODKEY, key: KEY_Q, action: Action::Quit },
    ]
}

pub const TERMINAL: &str = "foot";