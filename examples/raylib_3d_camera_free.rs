// https://github.com/raysan5/raylib/blob/master/examples/core/core_3d_camera_free.c
use rl::prelude::*;

fn main() {
    let mut window = Window::builder()
        .size(800, 450)
        .title("raylib [core] example - 2d camera")
        .flags(ConfigFlags::MSAA_4X_HINT)
        .target_fps(60)
        .init();

    let mut camera = Camera3D::builder()
        .position((10., 10., 10.))
        .target(Vector3::ZERO)
        .fovy(Angle::degrees(45.))
        .projection(CameraProjection::Perspective)
        .build();

    let cube_pos = Vector3::ZERO;

    window.disable_cursor();

    while let Some(frame) = window.next_frame() {
        camera.update(CameraMode::Free);
        if frame.keyboard().is_key_pressed(KeyboardKey::Z) {
            camera.target = Vector3::ZERO;
        }

        let mut canvas = frame.begin_drawing();

        canvas.clear_background(Color::RAYWHITE);

        canvas.with_camera_3d(camera, |cam| {
            cam.draw_cube(cube_pos, Vector3::value(2.), Color::RED);
            cam.draw_cube_wires(cube_pos, Vector3::value(2.), Color::RED);

            cam.draw_grid(10, 1.);
        });

        canvas.draw_rectangle(
            Rectangle::new(10., 10., 320., 93.),
            Color::SKYBLUE.alpha(0.5),
        );
        canvas.draw_rectangle_lines(Rectangle::new(10., 10., 320., 93.), 3., Color::BLUE);

        canvas.draw_text(
            "Free camera default controls:",
            (20., 20.),
            10,
            Color::BLACK,
        );
        canvas.draw_text(
            "- Mouse Wheel to Zoom in-out",
            (40., 40.),
            10,
            Color::DARKGRAY,
        );
        canvas.draw_text(
            "- Mouse Wheel Pressed to Pan",
            (40., 60.),
            10,
            Color::DARKGRAY,
        );
        canvas.draw_text("- Z to zoom to (0, 0, 0)", (40., 80.), 10, Color::DARKGRAY);
    }
}
