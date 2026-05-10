use smithay::input::keyboard::Keysym;

pub const MODKEY: u32 = 0x40; // Super key (mod4)
pub const TERMINAL: &str = "foot"; // терминал по умолчанию

pub const KEYBINDINGS: &[(Modifiers, Keysym, Action)] = &[
    (MODKEY, Keysym::Return, Action::SpawnTerminal),
    (MODKEY, Keysym::J, Action::FocusNext),
    (MODKEY, Keysym::K, Action::FocusPrev),
    (MODKEY, Keysym::H, Action::ResizeHoriz(-50)),
    (MODKEY, Keysym::L, Action::ResizeHoriz(50)),
    (MODKEY | ShiftMask, Keysym::C, Action::CloseWindow),
    (MODKEY, Keysym::Q, Action::Quit),
];

pub type Modifiers = u32;
pub const ShiftMask: u32 = 0x01;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    SpawnTerminal,
    FocusNext,
    FocusPrev,
    ResizeHoriz(i32),  // положительное — увеличить, отрицательное — уменьшить
    CloseWindow,
    Quit,
}