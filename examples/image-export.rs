use std::ops::Deref;

// https://github.com/raysan5/raylib/blob/master/examples/core/core_basic_window.c
use rl::prelude::*;

fn main() {
    let mut img = Image::gen_gradient_linear(400, 400, 45, Color::RED, Color::BLUE);
    img.draw_circle(Vector2::new(100., 100.), 75., Color::WHITE.alpha(0.25));
    let export = img.export_to_memory(FileType::Png);
    std::fs::write("out.png", export.deref()).unwrap();
}
