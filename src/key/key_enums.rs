use serde::{Serialize, Deserialize};
use num_enum::TryFromPrimitive;

/// define virtual keys and related code in win
///
#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum VirtualKey {
    // function keys
    Backspace = 0x08,
    Tab = 0x09,
    Enter = 0x0D,
    Shift = 0x10,
    Ctrl = 0x11,
    Alt = 0x12,
    Pause = 0x13,
    CapsLock = 0x14,
    Esc = 0x1B,
    Space = 0x20,
    PageUp = 0x21,
    PageDown = 0x22,
    End = 0x23,
    Home = 0x24,
    LeftArrow = 0x25,
    UpArrow = 0x26,
    RightArrow = 0x27,
    DownArrow = 0x28,

    // numbers in main area
    Key0 = 0x30,
    Key1 = 0x31,
    Key2 = 0x32,
    Key3 = 0x33,
    Key4 = 0x34,
    Key5 = 0x35,
    Key6 = 0x36,
    Key7 = 0x37,
    Key8 = 0x38,
    Key9 = 0x39,

    // characters
    KeyA = 0x41,
    KeyB = 0x42,
    KeyC = 0x43,
    KeyD = 0x44,
    KeyE = 0x45,
    KeyF = 0x46,
    KeyG = 0x47,
    KeyH = 0x48,
    KeyI = 0x49,
    KeyJ = 0x4A,
    KeyK = 0x4B,
    KeyL = 0x4C,
    KeyM = 0x4D,
    KeyN = 0x4E,
    KeyO = 0x4F,
    KeyP = 0x50,
    KeyQ = 0x51,
    KeyR = 0x52,
    KeyS = 0x53,
    KeyT = 0x54,
    KeyU = 0x55,
    KeyV = 0x56,
    KeyW = 0x57,
    KeyX = 0x58,
    KeyY = 0x59,
    KeyZ = 0x5A,

    // control keys
    LeftShift = 0xA0,
    RightShift = 0xA1,
    LeftControl = 0xA2,
    RightControl = 0xA3,

    #[num_enum(default)]
    Unknown = 0x0,
}

/// define actions of key in win event
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum KeyAction {
    Press = 0x100,
    Release = 0x101,

    #[num_enum(default)]
    Unknown = 0x0,
}

/// define key status
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum KeyStatus {
    Pressed,
    Released,
    #[num_enum(default)]
    Unknown,
}

/// a key trait
pub trait Key {

    /// check if this key is pressed
    fn is_pressed(&self) -> bool;

    /// check if this key is released. a reverse function from is_pressed
    fn is_released(&self) -> bool;

    /// send a press event of this key to system
    /// to simulated a real key press
    fn press(&self);

    /// bind a function to this key, if the key was pressed then the function will be called.
    fn on_pressed(&self, func: &dyn Fn() -> ());
}


