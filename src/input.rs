use std::{ffi::CStr, sync::atomic::Ordering, time::Duration};

use raylib_sys as sys;

use crate::{globals::WINDOW_INITIALISED, math::Vector2};

/// Assert that it's okay to access input devices
// NOTE: this is kind of a hack because there's no clean way to specify the lifetime of the input
// device in such a way that it's okay to also call mutable functions on the frame.
// NOTE: This is a macro instead of a function to ensure error message location is accurate
macro_rules! assert_access_okay {
    () => {
        assert!(WINDOW_INITIALISED.load(Ordering::Acquire));
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MouseButton {
    /// Mouse button left
    Left = 0,
    /// Mouse button right
    Right = 1,
    /// Mouse button middle (pressed wheel)
    Middle = 2,
    /// Mouse button side (advanced mouse device)
    Side = 3,
    /// Mouse button extra (advanced mouse device)
    Extra = 4,
    /// Mouse button forward (advanced mouse device)
    Forward = 5,
    /// Mouse button back (advanced mouse device)
    Back = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u32)]
pub enum MouseCursor {
    /// Default pointer shape
    #[default]
    Default = 0,
    /// Arrow shape
    Arrow = 1,
    /// Text writing cursor shape
    Ibeam = 2,
    /// Cross shape
    Crosshair = 3,
    /// Pointing hand cursor
    PointingHand = 4,
    /// Horizontal resize/move arrow shape
    ResizeEw = 5,
    /// Vertical resize/move arrow shape
    ResizeNs = 6,
    /// Top-left to bottom-right diagonal resize/move arrow shape
    ResizeNwse = 7,
    /// The top-right to bottom-left diagonal resize/move arrow shape
    ResizeNesw = 8,
    /// The omnidirectional resize/move cursor shape
    ResizeAll = 9,
    /// The operation-not-allowed shape
    NotAllowed = 10,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Mouse;

impl Mouse {
    /// Get mouse position XY
    pub fn position(&self) -> Vector2 {
        assert_access_okay!();
        unsafe { sys::GetMousePosition() }.into()
    }

    /// Set mouse position XY
    pub fn set_position(&mut self, position: Vector2) {
        assert_access_okay!();
        unsafe { sys::SetMousePosition(position.x as _, position.y as _) }
    }

    /// Set mouse offset
    pub fn set_offset(&mut self, offset: impl Into<Vector2>) {
        assert_access_okay!();
        let offset = offset.into();
        unsafe { sys::SetMouseOffset(offset.x as _, offset.y as _) }
    }

    /// Set mouse scaling
    pub fn set_scale(&mut self, scale: impl Into<Vector2>) {
        assert_access_okay!();
        let scale = scale.into();
        unsafe { sys::SetMouseScale(scale.x, scale.y) }
    }

    /// Get mouse delta between frames
    pub fn delta(&self) -> Vector2 {
        assert_access_okay!();
        unsafe { sys::GetMouseDelta() }.into()
    }

    /// Get mouse wheel movement for X or Y, whichever is larger
    pub fn wheel_move(&self) -> f32 {
        assert_access_okay!();
        unsafe { sys::GetMouseWheelMove() }
    }

    /// Get mouse wheel movement for both X and Y
    pub fn wheel_move_v(&self) -> Vector2 {
        assert_access_okay!();
        unsafe { sys::GetMouseWheelMoveV() }.into()
    }

    pub fn show_cursor(&mut self) {
        assert_access_okay!();
        unsafe { sys::ShowCursor() }
    }

    pub fn hide_cursor(&mut self) {
        assert_access_okay!();
        unsafe { sys::HideCursor() }
    }

    pub fn is_cursor_hidden(&self) -> bool {
        assert_access_okay!();
        unsafe { sys::IsCursorHidden() }
    }

    pub fn enable_cursor(&mut self) {
        assert_access_okay!();
        unsafe { sys::EnableCursor() }
    }

    pub fn disable_cursor(&mut self) {
        assert_access_okay!();
        unsafe { sys::DisableCursor() }
    }

    pub fn set_cursor(&mut self, cursor: MouseCursor) {
        assert_access_okay!();
        unsafe { sys::SetMouseCursor(cursor as _) }
    }

    pub fn is_cursor_on_screen(&self) -> bool {
        assert_access_okay!();
        unsafe { sys::IsCursorOnScreen() }
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        assert_access_okay!();
        unsafe { sys::IsMouseButtonPressed(button as _) }
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        assert_access_okay!();
        unsafe { sys::IsMouseButtonDown(button as _) }
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        assert_access_okay!();
        unsafe { sys::IsMouseButtonReleased(button as _) }
    }

    pub fn is_button_up(&self, button: MouseButton) -> bool {
        assert_access_okay!();
        unsafe { sys::IsMouseButtonUp(button as _) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[rustfmt::skip]
// NOTE: using the #[doc] syntax here, just so I can inline them
pub enum Key {
    // Alphanumeric keys
    #[doc = r"Key: '"                          ] Apostrophe        = 39,
    #[doc = r"Key: ,"                          ] Comma             = 44,
    #[doc = r"Key: -"                          ] Minus             = 45,
    #[doc = r"Key: ."                          ] Period            = 46,
    #[doc = r"Key: /"                          ] Slash             = 47,
    #[doc = r"Key: 0"                          ] Zero              = 48,
    #[doc = r"Key: 1"                          ] One               = 49,
    #[doc = r"Key: 2"                          ] Two               = 50,
    #[doc = r"Key: 3"                          ] Three             = 51,
    #[doc = r"Key: 4"                          ] Four              = 52,
    #[doc = r"Key: 5"                          ] Five              = 53,
    #[doc = r"Key: 6"                          ] Six               = 54,
    #[doc = r"Key: 7"                          ] Seven             = 55,
    #[doc = r"Key: 8"                          ] Eight             = 56,
    #[doc = r"Key: 9"                          ] Nine              = 57,
    #[doc = r"Key: ;"                          ] Semicolon         = 59,
    #[doc = r"Key: ="                          ] Equal             = 61,
    #[doc = r"Key: A | a"                      ] A                 = 65,
    #[doc = r"Key: B | b"                      ] B                 = 66,
    #[doc = r"Key: C | c"                      ] C                 = 67,
    #[doc = r"Key: D | d"                      ] D                 = 68,
    #[doc = r"Key: E | e"                      ] E                 = 69,
    #[doc = r"Key: F | f"                      ] F                 = 70,
    #[doc = r"Key: G | g"                      ] G                 = 71,
    #[doc = r"Key: H | h"                      ] H                 = 72,
    #[doc = r"Key: I | i"                      ] I                 = 73,
    #[doc = r"Key: J | j"                      ] J                 = 74,
    #[doc = r"Key: K | k"                      ] K                 = 75,
    #[doc = r"Key: L | l"                      ] L                 = 76,
    #[doc = r"Key: M | m"                      ] M                 = 77,
    #[doc = r"Key: N | n"                      ] N                 = 78,
    #[doc = r"Key: O | o"                      ] O                 = 79,
    #[doc = r"Key: P | p"                      ] P                 = 80,
    #[doc = r"Key: Q | q"                      ] Q                 = 81,
    #[doc = r"Key: R | r"                      ] R                 = 82,
    #[doc = r"Key: S | s"                      ] S                 = 83,
    #[doc = r"Key: T | t"                      ] T                 = 84,
    #[doc = r"Key: U | u"                      ] U                 = 85,
    #[doc = r"Key: V | v"                      ] V                 = 86,
    #[doc = r"Key: W | w"                      ] W                 = 87,
    #[doc = r"Key: X | x"                      ] X                 = 88,
    #[doc = r"Key: Y | y"                      ] Y                 = 89,
    #[doc = r"Key: Z | z"                      ] Z                 = 90,
    #[doc = r"Key: ["                          ] LeftBracket       = 91,
    #[doc = r"Key: '\'"                        ] Backslash         = 92,
    #[doc = r"Key: ]"                          ] RightBracket      = 93,
    #[doc = r"Key: `"                          ] Grave             = 96,
    // Function keys
    #[doc = r"Key: Space"                      ] Space             = 32,
    #[doc = r"Key: Esc"                        ] Escape            = 256,
    #[doc = r"Key: Enter"                      ] Enter             = 257,
    #[doc = r"Key: Tab"                        ] Tab               = 258,
    #[doc = r"Key: Backspace"                  ] Backspace         = 259,
    #[doc = r"Key: Ins"                        ] Insert            = 260,
    #[doc = r"Key: Del"                        ] Delete            = 261,
    #[doc = r"Key: Cursor right"               ] Right             = 262,
    #[doc = r"Key: Cursor left"                ] Left              = 263,
    #[doc = r"Key: Cursor down"                ] Down              = 264,
    #[doc = r"Key: Cursor up"                  ] Up                = 265,
    #[doc = r"Key: Page up"                    ] PageUp            = 266,
    #[doc = r"Key: Page down"                  ] PageDown          = 267,
    #[doc = r"Key: Home"                       ] Home              = 268,
    #[doc = r"Key: End"                        ] End               = 269,
    #[doc = r"Key: Caps lock"                  ] CapsLock          = 280,
    #[doc = r"Key: Scroll down"                ] ScrollLock        = 281,
    #[doc = r"Key: Num lock"                   ] NumLock           = 282,
    #[doc = r"Key: Print screen"               ] PrintScreen       = 283,
    #[doc = r"Key: Pause"                      ] Pause             = 284,
    #[doc = r"Key: F1"                         ] F1                = 290,
    #[doc = r"Key: F2"                         ] F2                = 291,
    #[doc = r"Key: F3"                         ] F3                = 292,
    #[doc = r"Key: F4"                         ] F4                = 293,
    #[doc = r"Key: F5"                         ] F5                = 294,
    #[doc = r"Key: F6"                         ] F6                = 295,
    #[doc = r"Key: F7"                         ] F7                = 296,
    #[doc = r"Key: F8"                         ] F8                = 297,
    #[doc = r"Key: F9"                         ] F9                = 298,
    #[doc = r"Key: F10"                        ] F10               = 299,
    #[doc = r"Key: F11"                        ] F11               = 300,
    #[doc = r"Key: F12"                        ] F12               = 301,
    #[doc = r"Key: Shift left"                 ] LeftShift         = 340,
    #[doc = r"Key: Control left"               ] LeftControl       = 341,
    #[doc = r"Key: Alt left"                   ] LeftAlt           = 342,
    #[doc = r"Key: Super left"                 ] LeftSuper         = 343,
    #[doc = r"Key: Shift right"                ] RightShift        = 344,
    #[doc = r"Key: Control right"              ] RightControl      = 345,
    #[doc = r"Key: Alt right"                  ] RightAlt          = 346,
    #[doc = r"Key: Super right"                ] RightSuper        = 347,
    #[doc = r"Key: KB menu"                    ] KbMenu            = 348,
    // Keypad keys
    #[doc = r"Key: Keypad 0"                   ] Kp0               = 320,
    #[doc = r"Key: Keypad 1"                   ] Kp1               = 321,
    #[doc = r"Key: Keypad 2"                   ] Kp2               = 322,
    #[doc = r"Key: Keypad 3"                   ] Kp3               = 323,
    #[doc = r"Key: Keypad 4"                   ] Kp4               = 324,
    #[doc = r"Key: Keypad 5"                   ] Kp5               = 325,
    #[doc = r"Key: Keypad 6"                   ] Kp6               = 326,
    #[doc = r"Key: Keypad 7"                   ] Kp7               = 327,
    #[doc = r"Key: Keypad 8"                   ] Kp8               = 328,
    #[doc = r"Key: Keypad 9"                   ] Kp9               = 329,
    #[doc = r"Key: Keypad ."                   ] KpDecimal         = 330,
    #[doc = r"Key: Keypad /"                   ] KpDivide          = 331,
    #[doc = r"Key: Keypad *"                   ] KpMultiply        = 332,
    #[doc = r"Key: Keypad -"                   ] KpSubtract        = 333,
    #[doc = r"Key: Keypad +"                   ] KpAdd             = 334,
    #[doc = r"Key: Keypad Enter"               ] KpEnter           = 335,
    #[doc = r"Key: Keypad ="                   ] KpEqual           = 336,
    // Android key buttons
    #[doc = r"Key: Android back button"        ] AndroidBack       = 4,
    #[doc = r"Key: Android menu button"        ] AndroidMenu       = 5,
    #[doc = r"Key: Android volume up button"   ] AndroidVolumeUp   = 24,
    #[doc = r"Key: Android volume down button" ] AndroidVolumeDown = 25,
}

impl Key {
    /// # SAFETY
    ///
    /// Caller must guarantee that `key` can be represented by [`KeyboardKey`]
    pub(crate) unsafe fn from_u32_unchecked(key: u32) -> Self {
        unsafe { std::mem::transmute(key) }
    }

    pub(crate) fn from_u32(value: u32) -> Option<Self> {
        // SAFETY: We don't use this value until we match and guarantee it is safe.
        let this = unsafe { Self::from_u32_unchecked(value) };

        // XXX: referencing `this` outside of the branch in this match is UNDEFINED BEHAVIOUR, do
        // not do it.
        match this {
            Key::Apostrophe
            | Key::Comma
            | Key::Minus
            | Key::Period
            | Key::Slash
            | Key::Zero
            | Key::One
            | Key::Two
            | Key::Three
            | Key::Four
            | Key::Five
            | Key::Six
            | Key::Seven
            | Key::Eight
            | Key::Nine
            | Key::Semicolon
            | Key::Equal
            | Key::A
            | Key::B
            | Key::C
            | Key::D
            | Key::E
            | Key::F
            | Key::G
            | Key::H
            | Key::I
            | Key::J
            | Key::K
            | Key::L
            | Key::M
            | Key::N
            | Key::O
            | Key::P
            | Key::Q
            | Key::R
            | Key::S
            | Key::T
            | Key::U
            | Key::V
            | Key::W
            | Key::X
            | Key::Y
            | Key::Z
            | Key::LeftBracket
            | Key::Backslash
            | Key::RightBracket
            | Key::Grave
            | Key::Space
            | Key::Escape
            | Key::Enter
            | Key::Tab
            | Key::Backspace
            | Key::Insert
            | Key::Delete
            | Key::Right
            | Key::Left
            | Key::Down
            | Key::Up
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End
            | Key::CapsLock
            | Key::ScrollLock
            | Key::NumLock
            | Key::PrintScreen
            | Key::Pause
            | Key::F1
            | Key::F2
            | Key::F3
            | Key::F4
            | Key::F5
            | Key::F6
            | Key::F7
            | Key::F8
            | Key::F9
            | Key::F10
            | Key::F11
            | Key::F12
            | Key::LeftShift
            | Key::LeftControl
            | Key::LeftAlt
            | Key::LeftSuper
            | Key::RightShift
            | Key::RightControl
            | Key::RightAlt
            | Key::RightSuper
            | Key::KbMenu
            | Key::Kp0
            | Key::Kp1
            | Key::Kp2
            | Key::Kp3
            | Key::Kp4
            | Key::Kp5
            | Key::Kp6
            | Key::Kp7
            | Key::Kp8
            | Key::Kp9
            | Key::KpDecimal
            | Key::KpDivide
            | Key::KpMultiply
            | Key::KpSubtract
            | Key::KpAdd
            | Key::KpEnter
            | Key::KpEqual
            | Key::AndroidBack
            | Key::AndroidMenu
            | Key::AndroidVolumeUp
            | Key::AndroidVolumeDown => return Some(this),
        }

        #[expect(
            unreachable_code,
            reason = "The transmute makes this 'unreachable', but if key is not a valid KeyboardKey, it will be hit."
        )]
        None
    }

    /// Get name of a QWERTY key on the current keyboard layout (eg returns string 'q' for KEY_A on
    /// an AZERTY keyboard)
    pub fn name(self) -> &'static str {
        assert_access_okay!();
        let ptr = unsafe { sys::GetKeyName(self as _) };
        // SAFETY: raylib promised
        unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .expect("Key name was invalid utf-8")
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Keyboard;

impl Keyboard {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        assert_access_okay!();
        unsafe { sys::IsKeyPressed(key as _) }
    }

    pub fn is_key_pressed_repeat(&self, key: Key) -> bool {
        assert_access_okay!();
        unsafe { sys::IsKeyPressedRepeat(key as _) }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        assert_access_okay!();
        unsafe { sys::IsKeyDown(key as _) }
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        assert_access_okay!();
        unsafe { sys::IsKeyReleased(key as _) }
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        assert_access_okay!();
        unsafe { sys::IsKeyUp(key as _) }
    }

    /// Get an iterator over the keys pressed.
    ///
    /// If only a few items of the iterator are consumed, this function may be called again to get
    /// the remaining keys.
    pub fn get_keys_pressed<'f>(&'f mut self) -> impl Iterator<Item = Key> + use<'f> {
        assert_access_okay!();
        std::iter::from_fn(|| {
            assert_access_okay!();
            let key = unsafe { sys::GetKeyPressed() };
            if key == 0 {
                None
            } else {
                #[cfg(debug_assertions)]
                {
                    // just assert when we're in debug mode
                    Key::from_u32(key as u32).expect("Invalid KeybordKey");
                }
                // SAFETY: key should be a valid KeyboardKey according to Raylib
                Some(unsafe { Key::from_u32_unchecked(key as u32) })
            }
        })
    }

    pub fn get_chars_pressed<'f>(&'f mut self) -> impl Iterator<Item = char> + use<'f> {
        assert_access_okay!();
        std::iter::from_fn(|| {
            assert_access_okay!();
            let c = unsafe { sys::GetCharPressed() };
            if c == 0 {
                None
            } else {
                Some(char::from_u32(c as u32).expect("Raylib says this is a valid char"))
            }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GamepadButton {
    /// Unknown button, for error checking
    Unknown = 0,
    /// Gamepad left DPAD up button
    LeftFaceUp = 1,
    /// Gamepad left DPAD right button
    LeftFaceRight = 2,
    /// Gamepad left DPAD down button
    LeftFaceDown = 3,
    /// Gamepad left DPAD left button
    LeftFaceLeft = 4,
    /// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceUp = 5,
    /// Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceRight = 6,
    /// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceDown = 7,
    /// Gamepad right button left (i.e. PS3: Square, Xbox: X)
    RightFaceLeft = 8,
    /// Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger1 = 9,
    /// Gamepad top/back trigger left (second), it could be a trailing button
    LeftTrigger2 = 10,
    /// Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger1 = 11,
    /// Gamepad top/back trigger right (second), it could be a trailing button
    RightTrigger2 = 12,
    /// Gamepad center buttons, left one (i.e. PS3: Select)
    MiddleLeft = 13,
    /// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    Middle = 14,
    /// Gamepad center buttons, right one (i.e. PS3: Start)
    MiddleRight = 15,
    /// Gamepad joystick pressed button left
    LeftThumb = 16,
    /// Gamepad joystick pressed button right
    RightThumb = 17, // If this is no longer the last item, update from_u32
}

impl GamepadButton {
    pub const fn from_u32(n: u32) -> Option<Self> {
        if n > Self::RightThumb as _ {
            None
        } else {
            // SAFETY: Checked above
            Some(unsafe { std::mem::transmute::<u32, Self>(n) })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum GamepadAxis {
    /// Gamepad left stick X axis
    LeftX = 0,
    /// Gamepad left stick Y axis
    LeftY = 1,
    /// Gamepad right stick X axis
    RightX = 2,
    /// Gamepad right stick Y axis
    RightY = 3,
    /// Gamepad back trigger left, pressure level: [1..-1]
    LeftTrigger = 4,
    /// Gamepad back trigger right, pressure level: [1..-1]
    RightTrigger = 5,
}

impl GamepadAxis {
    pub const VARIANTS: [Self; 6] = [
        Self::LeftX,
        Self::LeftY,
        Self::RightX,
        Self::RightY,
        Self::LeftTrigger,
        Self::RightTrigger,
    ];
}

#[derive(Debug)]
pub struct Gamepad {
    id: u32,
}

impl Gamepad {
    pub(crate) fn new(id: u32) -> Option<Self> {
        assert_access_okay!();
        if unsafe { sys::IsGamepadAvailable(id as _) } {
            Some(Self { id })
        } else {
            None
        }
    }

    /// Get the last gamepad button pressed
    pub fn get_button_pressed() -> Option<GamepadButton> {
        assert_access_okay!();
        let n = unsafe { sys::GetGamepadButtonPressed() };
        GamepadButton::from_u32(n as _)
    }

    /// Get gamepad internal name id
    pub fn name(&self) -> &str {
        assert_access_okay!();
        // SAFETY: this is only constructed if gamepad is available
        let name = unsafe { sys::GetGamepadName(self.id as _) };
        // SAFETY: raylib promised
        unsafe { CStr::from_ptr(name) }
            .to_str()
            .expect("Gamepad name was invalid utf-8")
    }

    /// Check if a gamepad button has been pressed once
    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        assert_access_okay!();
        unsafe { sys::IsGamepadButtonPressed(self.id as _, button as _) }
    }

    /// Check if a gamepad button is being pressed
    pub fn is_button_down(&self, button: GamepadButton) -> bool {
        unsafe { sys::IsGamepadButtonDown(self.id as _, button as _) }
    }

    /// Check if a gamepad button has been released once
    pub fn is_button_released(&self, button: GamepadButton) -> bool {
        assert_access_okay!();
        unsafe { sys::IsGamepadButtonReleased(self.id as _, button as _) }
    }

    /// Check if a gamepad button is NOT being pressed
    pub fn is_button_up(&self, button: GamepadButton) -> bool {
        unsafe { sys::IsGamepadButtonUp(self.id as _, button as _) }
    }

    /// Get axis count for a gamepad
    pub fn axis_count(&self) -> u32 {
        assert_access_okay!();
        (unsafe { sys::GetGamepadAxisCount(self.id as _) }) as u32
    }

    /// Get axis count for a gamepad
    pub fn get_axis_movement(&self, axis: GamepadAxis) -> f32 {
        unsafe { sys::GetGamepadAxisMovement(self.id as _, axis as _) }
    }

    /// Set gamepad vibration for both motors
    pub fn set_vibration(&mut self, left_motor: f32, right_motor: f32, duration: Duration) {
        assert_access_okay!();
        unsafe {
            sys::SetGamepadVibration(
                self.id as _,
                left_motor,
                right_motor,
                duration.as_secs_f32(),
            )
        }
    }
}
