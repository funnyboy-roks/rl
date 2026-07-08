// https://github.com/raysan5/raylib/blob/master/examples/core/core_delta_time.c
use rl::prelude::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let mut window = Window::init(
        screen_width,
        screen_height,
        "raylib [core] example - delta time",
    );

    let mut current_fps = 60;

    let mut delta_circle = Vector2::new(0., screen_height as f32 / 3.);
    let mut frame_circle = Vector2::new(0., screen_height as f32 * (2. / 3.));

    // The speed applied to both circles
    let speed = 10.;
    let circle_radius = 32.;

    window.set_target_fps(current_fps);

    while let Some(mut frame) = window.next_frame() {
        let mouse_wheel = frame.mouse().wheel_move();

        if mouse_wheel != 0. {
            current_fps = current_fps.saturating_add_signed(mouse_wheel as i32);
            frame.window_mut().set_target_fps(current_fps);
        }

        delta_circle.x += frame.get_time() * 6. * speed;
        frame_circle.x += 0.1 * speed;
        if delta_circle.x > screen_width as _ {
            delta_circle.x = 0.;
        }
        if frame_circle.x > screen_width as _ {
            frame_circle.x = 0.;
        }

        if frame.is_key_pressed(KeyboardKey::KEY_R) {
            delta_circle.x = 0.;
            frame_circle.x = 0.;
        }

        frame.clear_background(Color::RAYWHITE);

        frame.draw_circle(delta_circle, circle_radius, Color::RED);
        frame.draw_circle(frame_circle, circle_radius, Color::BLUE);

        let fps_text = if current_fps == 0 {
            format!("FPS: unlimited ({})", frame.window().get_fps())
        } else {
            format!(
                "FPS: {} (target: {})",
                frame.window().get_fps(),
                current_fps
            )
        };
        frame.draw_text(fps_text, Vector2::new(10., 10.), 20, Color::DARKGRAY);
        frame.draw_text(
            format!("Frame time: {:02} ms", frame.get_time()),
            Vector2::new(10., 30.),
            20,
            Color::DARKGRAY,
        );
        frame.draw_text(
            "Use the scroll wheel to change the fps limit, r to reset",
            Vector2::new(10., 50.),
            20,
            Color::DARKGRAY,
        );

        frame.draw_text(
            "FUNC: x += frame.get_time() * speed",
            Vector2::new(10., 90.),
            20,
            Color::RED,
        );
        frame.draw_text("FUNC: x += speed", Vector2::new(10., 240.), 20, Color::BLUE);
    }
}
