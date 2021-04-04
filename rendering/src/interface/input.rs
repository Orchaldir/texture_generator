/// Possible keyboard keys
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum KeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Comma,
    Period,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    Escape,
    Backspace,
    Enter,
    Space,
    Snapshot,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Plus,
    Minus,
    Multiply,
    Divide,

    Left,
    Up,
    Right,
    Down,
}

/// Possible buttons of a mouse
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub fn get_number(key: KeyCode) -> Option<usize> {
    match key {
        KeyCode::Key0 => Some(0),
        KeyCode::Key1 => Some(1),
        KeyCode::Key2 => Some(2),
        KeyCode::Key3 => Some(3),
        KeyCode::Key4 => Some(4),
        KeyCode::Key5 => Some(5),
        KeyCode::Key6 => Some(6),
        KeyCode::Key7 => Some(7),
        KeyCode::Key8 => Some(8),
        KeyCode::Key9 => Some(9),
        _ => None,
    }
}
