use rl::{camera::Camera2D, prelude::*};

fn main() {
    let mut window = Window::builder()
        .size(800, 600)
        .title("camera")
        .target_fps(60)
        .init();

    let mut camera = Camera2D::builder().build();

    let mut player = Vector2::ZERO;

    while let Some(mut frame) = window.next_frame() {
        camera.offset = frame.size() / 2.;

        let mut vel = Vector2::ZERO;
        if frame.keyboard().is_key_down(KeyboardKey::A) {
            vel.x -= 1.;
        }
        if frame.keyboard().is_key_down(KeyboardKey::D) {
            vel.x += 1.;
        }
        if frame.keyboard().is_key_down(KeyboardKey::S) {
            vel.y += 1.;
        }
        if frame.keyboard().is_key_down(KeyboardKey::W) {
            vel.y -= 1.;
        }
        player += vel.normalize() * 10.;

        let mut vel = Vector2::ZERO;
        if frame.keyboard().is_key_down(KeyboardKey::Left) {
            vel.x -= 1.;
        }
        if frame.keyboard().is_key_down(KeyboardKey::Right) {
            vel.x += 1.;
        }
        if frame.keyboard().is_key_down(KeyboardKey::Down) {
            vel.y += 1.;
        }
        if frame.keyboard().is_key_down(KeyboardKey::Up) {
            vel.y -= 1.;
        }
        camera.target += vel.normalize() * 10.;

        if frame.keyboard().is_key_pressed(KeyboardKey::R) {
            camera.target = Vector2::ZERO;
            player = Vector2::ZERO;
        }

        frame.with_canvas(|canvas| {
            canvas.clear_background(Color::BLUE);

            canvas.with_camera_mode_2d(camera, |cam| {
                // draw some circles as a reference point for the camera
                for i in 0..200 {
                    let r = i as f32 * 50.;
                    cam.draw_ring(Vector2::ZERO, r, r + 2., 0., 360., 60, Color::GREEN);
                }

                cam.draw_rectangle(
                    Rectangle::new(player.x - 50., player.y - 50., 100., 100.),
                    Color::RED,
                );
            });

            canvas.draw_text(
                "Use WASD to move player",
                (10., canvas.height() as f32 - 30. - 30. - 10.),
                30,
                Color::BLACK,
            );
            canvas.draw_text(
                "and arrow keys to move the camera",
                (10., canvas.height() as f32 - 30. - 10.),
                30,
                Color::BLACK,
            );
            canvas.draw_fps(10, 10);
        });
    }
}
