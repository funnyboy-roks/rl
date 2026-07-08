// https://github.com/raysan5/raylib/blob/master/examples/textures/textures_image_generation.c

use rl::prelude::*;

fn main() {
    let width = 800;
    let height = 450;
    let mut window = Window::init(
        width,
        height,
        "raylib [textures] example - image generation",
    );

    window.set_target_fps(60);

    let textures = [
        Image::gen_gradient_linear(width, height, 0, Color::RED, Color::BLUE),
        Image::gen_gradient_linear(width, height, 90, Color::RED, Color::BLUE),
        Image::gen_gradient_linear(width, height, 45, Color::RED, Color::BLUE),
        Image::gen_gradient_radial(width, height, 0., Color::WHITE, Color::BLACK),
        Image::gen_gradient_square(width, height, 0., Color::WHITE, Color::BLACK),
        Image::gen_checked(width, height, 32, 32, Color::RED, Color::BLUE),
        Image::gen_white_noise(width, height, 0.5),
        Image::gen_perlin_noise(width, height, 50, 50, 4.),
        Image::gen_cellular(width, height, 32),
    ]
    .map(|i| Texture2D::from_image(&i));

    let mut current_texture = 0;
    while let Some(mut frame) = window.next_frame() {
        frame.clear_background(Color::WHITE);

        if frame.is_key_pressed(KeyboardKey::KEY_SPACE) {
            current_texture += 1;
            current_texture %= textures.len();
        }

        frame.draw_texture(
            &textures[current_texture],
            Vector2::zero(),
            0.,
            1.,
            Color::WHITE,
        );

        frame.draw_rectangle(
            Rectangle::new(30., 400., 325., 30.),
            Color::SKYBLUE.alpha(0.5),
        );
        frame.draw_rectangle_lines(
            Rectangle::new(30., 400., 325., 30.),
            1.,
            Color::WHITE.alpha(0.5),
        );
        frame.draw_text(
            "SPACE to CYCLE PROCEDURAL TEXTURES",
            Vector2::new(40., 410.),
            10,
            Color::WHITE,
        );

        let (text, col) = match current_texture {
            0 => ("VERTICAL GRADIENT", Color::RAYWHITE),
            1 => ("HORIZONTAL GRADIENT", Color::RAYWHITE),
            2 => ("DIAGONAL GRADIENT", Color::RAYWHITE),
            3 => ("RADIAL GRADIENT", Color::LIGHTGRAY),
            4 => ("SQUARE GRADIENT", Color::LIGHTGRAY),
            5 => ("CHECKED", Color::RAYWHITE),
            6 => ("WHITE NOISE", Color::RED),
            7 => ("PERLIN NOISE", Color::RED),
            8 => ("CELLULAR", Color::RAYWHITE),
            _ => unreachable!(),
        };

        frame.draw_text(text, Vector2::new(10., 10.), 20, col);
    }
}
