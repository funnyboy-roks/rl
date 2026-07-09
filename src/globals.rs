use std::sync::atomic::AtomicBool;

/// Whether `InitWindow()` has been called.  When this is `true`, no other windows may be created.
pub(crate) static WINDOW_INITIALISED: AtomicBool = AtomicBool::new(false);

/// Whether `BeginTextureMode()` has been called and all draw calls are being applied to a render
/// texture.  If this is true, then draw calls on `frame` must fail.
pub(crate) static DRAWING_TO_TEXTURE: AtomicBool = AtomicBool::new(false);
