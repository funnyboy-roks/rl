// https://github.com/raysan5/raylib/blob/master/examples/core/core_2d_camera.c
use rl::{camera::Camera2D, prelude::*};

fn main() {
    let mut window = Window::builder()
        .size(800, 450)
        .title("raylib [core] example - 2d camera")
        .flags(ConfigFlags::MSAA_4X_HINT)
        .target_fps(60)
        .init();

    let mut player = Rectangle::new(400., 280., 40., 40.);
    let mut buildings = Vec::new();

    let mut spacing = 0.;

    for _ in 0..100 {
        let width = rl::rand::random_value(50, 200) as f32;
        let height = rl::rand::random_value(100, 800) as f32;
        let y = window.height() as f32 - 130. - height;
        let x = -6000. + spacing;

        spacing += width;

        buildings.push((Rectangle::new(x, y, width, height), Color::random()))
    }

    let mut camera = Camera2D::builder()
        .target(Vector2::new(player.x, player.y))
        .offset(window.size() / 2.)
        .build();

    while let Some(frame) = window.next_frame() {
        if frame.keyboard().is_key_down(KeyboardKey::Right) {
            player.x += 2.;
        } else if frame.keyboard().is_key_down(KeyboardKey::Left) {
            player.x -= 2.;
        }

        camera.target = Vector2::new(player.x + 20., player.y + 20.);

        if frame.keyboard().is_key_down(KeyboardKey::A) {
            camera.rotation -= 1.;
        } else if frame.keyboard().is_key_down(KeyboardKey::S) {
            camera.rotation += 1.;
        }
        camera.rotation = camera.rotation.clamp(-40., 40.);

        // Camera zoom controls
        // Uses log scaling to provide consistent zoom speed
        camera.zoom = (camera.zoom.ln() + frame.mouse().wheel_move() * 0.1).exp();
        camera.zoom = camera.zoom.clamp(0.1, 3.);

        if frame.keyboard().is_key_pressed(KeyboardKey::R) {
            camera.zoom = 1.0;
            camera.rotation = 0.0;
        }

        let mut canvas = frame.begin_drawing();
        canvas.clear_background(Color::RAYWHITE);

        let size = canvas.size();

        canvas.with_camera_mode_2d(camera, |cam| {
            cam.draw_rectangle(Rectangle::new(-6000., 320., 13000., 8000.), Color::DARKGRAY);

            for &(rect, col) in &buildings {
                cam.draw_rectangle(rect, col);
            }

            cam.draw_rectangle(player, Color::RED);

            cam.draw_line(
                (camera.target.x, -size.y * 10.),
                (camera.target.x, size.y * 10.),
                3.,
                Color::GREEN,
            );
            cam.draw_line(
                (-size.x * 10., camera.target.y),
                (size.x * 10., camera.target.y),
                3.,
                Color::GREEN,
            );
        });

        canvas.draw_rectangle(
            Rectangle::new(10., 10., 250., 113.),
            Color::SKYBLUE.alpha(0.5),
        );
        canvas.draw_rectangle_lines(Rectangle::new(10., 10., 250., 113.), 3., Color::BLUE);

        canvas.draw_text("Free 2D camera controls:", (20., 20.), 10, Color::BLACK);
        canvas.draw_text(
            "- Right/Left to move player",
            (40., 40.),
            10,
            Color::DARKGRAY,
        );
        canvas.draw_text(
            "- Mouse Wheel to Zoom in-out",
            (40., 60.),
            10,
            Color::DARKGRAY,
        );
        canvas.draw_text("- A / S to Rotate", (40., 80.), 10, Color::DARKGRAY);
        canvas.draw_text(
            "- R to reset Zoom and Rotation",
            (40., 100.),
            10,
            Color::DARKGRAY,
        );
    }
}
