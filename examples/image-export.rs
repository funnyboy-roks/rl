use std::ops::Deref;

// https://github.com/raysan5/raylib/blob/master/examples/core/core_basic_window.c
use rl::prelude::*;

fn main() {
    let mut img = Image::gen_gradient_linear(400, 400, 45, Color::RED, Color::BLUE);
    img.draw_triangle_ex(
        ((img.width() as f32 / 2., 0.), Color::RED),
        ((0., img.height() as f32), Color::GREEN),
        ((img.width() as f32, img.height() as f32), Color::BLUE),
    );
    let export = img.export_to_memory(FileType::Png);
    std::fs::write("out.png", export.deref()).unwrap();
}
