# rl (name tbd)

A level of abstraction on top of [RayLib] that makes it work well in
Rust and easier to use in general.

[RayLib]: https://raylib.com

## Usage

```rust
use rl::{Window, Vector2, Color};

fn main() {
    let mut window = Window::init(800, 600, "My Game");

    while let Some(mut frame) = window.next_frame() {
        frame.draw_circle(Vector2::new(100., 100.), 50., Color::RED);
    }
}
```
