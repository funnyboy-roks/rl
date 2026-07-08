// https://github.com/raysan5/raylib/blob/master/examples/core/core_basic_window.c
use rl::prelude::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let mut window = Window::init(
        screen_width,
        screen_height,
        "raylib [core] example - basic window",
    );

    window.set_target_fps(60);

    while let Some(mut frame) = window.next_frame() {
        frame.clear_background(Color::RAYWHITE);
        frame.draw_text(
            "Congrats! You created your first window!",
            Vector2::new(190., 200.),
            20,
            Color::LIGHTGRAY,
        );
    }
}
