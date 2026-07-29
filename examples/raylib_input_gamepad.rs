use std::time::Duration;

// https://github.com/raysan5/raylib/blob/master/examples/core/core_2d_camera.c
use rl::prelude::*;

fn main() {
    let mut window = Window::builder()
        .size(800, 450)
        .title("raylib [core] example - input gamepad")
        .target_fps(60)
        .init();

    let tex_ps3 = Texture2D::load("examples/resources/ps3.png").unwrap();
    let tex_xbox = Texture2D::load("examples/resources/xbox.png").unwrap();

    const XBOX_ALIAS_1: &str = "xbox";
    const XBOX_ALIAS_2: &str = "x-box";
    const PS_ALIAS_1: &str = "playstation";
    const PS_ALIAS_2: &str = "sony";

    let left_stick_deadzone_x = 0.1;
    let left_stick_deadzone_y = 0.1;
    let right_stick_deadzone_x = 0.1;
    let right_stick_deadzone_y = 0.1;
    let left_trigger_deadzone = -0.9;
    let right_trigger_deadzone = -0.9;

    let mut gamepad_i: u32 = 0;

    while let Some(frame) = window.next_frame() {
        if frame.keyboard().is_key_pressed(KeyboardKey::Left) {
            gamepad_i = gamepad_i.saturating_sub(1);
        }

        if frame.keyboard().is_key_pressed(KeyboardKey::Right) {
            gamepad_i += 1;
        }

        let mut gamepad = frame.gamepad(gamepad_i);

        let vibrate_button = Rectangle::new(
            10.,
            70. + 20.
                * frame
                    .gamepad(gamepad_i)
                    .map(|g| g.axis_count())
                    .unwrap_or(0) as f32
                + 20.,
            75.,
            24.,
        );

        if frame.mouse().is_button_pressed(MouseButton::Left)
            && vibrate_button.check_collision_point_rec(frame.mouse().position())
            && let Some(ref mut gamepad) = gamepad
        {
            gamepad.set_vibration(1.0, 1.0, Duration::from_secs(1));
        }

        let mut canvas = frame.begin_drawing();
        canvas.clear_background(Color::RAYWHITE);

        if let Some(gamepad) = gamepad {
            canvas.draw_text(
                format!("GP{}: {}", gamepad_i, gamepad.name()),
                (10., 10.),
                10,
                Color::BLACK,
            );

            // Get axis values
            let mut left_stick_x = gamepad.get_axis_movement(GamepadAxis::LeftX);
            let mut left_stick_y = gamepad.get_axis_movement(GamepadAxis::LeftY);
            let mut right_stick_x = gamepad.get_axis_movement(GamepadAxis::RightX);
            let mut right_stick_y = gamepad.get_axis_movement(GamepadAxis::RightY);
            let mut left_trigger = gamepad.get_axis_movement(GamepadAxis::LeftTrigger);
            let mut right_trigger = gamepad.get_axis_movement(GamepadAxis::RightTrigger);

            // Calculate deadzones
            if left_stick_x.abs() < left_stick_deadzone_x {
                left_stick_x = 0.;
            }
            if left_stick_y.abs() < left_stick_deadzone_y {
                left_stick_y = 0.;
            }
            if right_stick_x.abs() < right_stick_deadzone_x {
                right_stick_x = 0.;
            }
            if right_stick_y.abs() < right_stick_deadzone_y {
                right_stick_y = 0.;
            }
            if left_trigger < left_trigger_deadzone {
                left_trigger = -1.;
            }
            if right_trigger < right_trigger_deadzone {
                right_trigger = -1.;
            }

            if gamepad.name().to_lowercase().contains(XBOX_ALIAS_1)
                || gamepad.name().to_lowercase().contains(XBOX_ALIAS_2)
            {
                canvas.draw_texture(&tex_xbox, (0., 0.), 0., 1., Color::DARKGRAY);

                // Draw buttons: xbox home
                if gamepad.is_button_down(GamepadButton::Middle) {
                    canvas.draw_circle((394., 89.), 19., Color::RED);
                }

                // Draw buttons: basic
                if gamepad.is_button_down(GamepadButton::MiddleRight) {
                    canvas.draw_circle((436., 150.), 9., Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::MiddleLeft) {
                    canvas.draw_circle((352., 150.), 9., Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceLeft) {
                    canvas.draw_circle((501., 151.), 15., Color::BLUE);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceDown) {
                    canvas.draw_circle((536., 187.), 15., Color::LIME);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceRight) {
                    canvas.draw_circle((572., 151.), 15., Color::MAROON);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceUp) {
                    canvas.draw_circle((536., 115.), 15., Color::GOLD);
                }

                // Draw buttons: d-pad
                canvas.draw_rectangle(Rectangle::new(317., 202., 19., 71.), Color::BLACK);
                canvas.draw_rectangle(Rectangle::new(293., 228., 69., 19.), Color::BLACK);
                if gamepad.is_button_down(GamepadButton::LeftFaceUp) {
                    canvas.draw_rectangle(Rectangle::new(317., 202., 19., 26.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceDown) {
                    canvas.draw_rectangle(Rectangle::new(317., 202. + 45., 19., 26.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceLeft) {
                    canvas.draw_rectangle(Rectangle::new(292., 228., 25., 19.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceRight) {
                    canvas.draw_rectangle(Rectangle::new(292. + 44., 228., 26., 19.), Color::RED);
                }

                // Draw buttons: left-right back
                if gamepad.is_button_down(GamepadButton::LeftTrigger1) {
                    canvas.draw_circle((259., 61.), 20., Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::RightTrigger1) {
                    canvas.draw_circle((536., 61.), 20., Color::RED);
                }

                // Draw axis: left joystick
                let left_gamepad_color = if gamepad.is_button_down(GamepadButton::LeftThumb) {
                    Color::RED
                } else {
                    Color::BLACK
                };
                canvas.draw_circle((259., 152.), 39., Color::BLACK);
                canvas.draw_circle((259., 152.), 34., Color::LIGHTGRAY);
                canvas.draw_circle(
                    (259. + (left_stick_x * 20.), 152. + (left_stick_y * 20.)),
                    25.,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let right_gamepad_color = if gamepad.is_button_down(GamepadButton::RightThumb) {
                    Color::RED
                } else {
                    Color::BLACK
                };
                canvas.draw_circle((461., 237.), 38., Color::BLACK);
                canvas.draw_circle((461., 237.), 33., Color::LIGHTGRAY);
                canvas.draw_circle(
                    (461. + (right_stick_x * 20.), 237. + (right_stick_y * 20.)),
                    25.,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                canvas.draw_rectangle(Rectangle::new(170., 30., 15., 70.), Color::GRAY);
                canvas.draw_rectangle(Rectangle::new(604., 30., 15., 70.), Color::GRAY);
                canvas.draw_rectangle(
                    Rectangle::new(170., 30., 15., ((1. + left_trigger) / 2.) * 70.),
                    Color::RED,
                );
                canvas.draw_rectangle(
                    Rectangle::new(604., 30., 15., ((1. + right_trigger) / 2.) * 70.),
                    Color::RED,
                );

                //DrawText(TextFormat("Xbox axis LT: %02.02", GetGamepadAxisMovement(gamepad, GamepadAxis::LEFT_TRIGGER)), 10, 40, 10, BLACK);
                //DrawText(TextFormat("Xbox axis RT: %02.02", GetGamepadAxisMovement(gamepad, GamepadAxis::RIGHT_TRIGGER)), 10, 60, 10, BLACK);
            } else if gamepad.name().to_lowercase().contains(PS_ALIAS_1)
                || gamepad.name().to_lowercase().contains(PS_ALIAS_2)
            {
                canvas.draw_texture(&tex_ps3, (0., 0.), 0., 1., Color::DARKGRAY);

                // Draw buttons: ps
                if gamepad.is_button_down(GamepadButton::Middle) {
                    canvas.draw_circle((396., 222.), 13., Color::RED)
                };

                // Draw buttons: basic
                if gamepad.is_button_down(GamepadButton::MiddleLeft) {
                    canvas.draw_rectangle(Rectangle::new(328., 170., 32., 13.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::MiddleRight) {
                    canvas.draw_triangle((436., 168.), (436., 185.), (464., 177.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceUp) {
                    canvas.draw_circle((557., 144.), 13., Color::LIME);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceRight) {
                    canvas.draw_circle((586., 173.), 13., Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceDown) {
                    canvas.draw_circle((557., 203.), 13., Color::VIOLET);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceLeft) {
                    canvas.draw_circle((527., 173.), 13., Color::PINK);
                }

                // Draw buttons: d-pad
                canvas.draw_rectangle(Rectangle::new(225., 132., 24., 84.), Color::BLACK);
                canvas.draw_rectangle(Rectangle::new(195., 161., 84., 25.), Color::BLACK);
                if gamepad.is_button_down(GamepadButton::LeftFaceUp) {
                    canvas.draw_rectangle(Rectangle::new(225., 132., 24., 29.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceDown) {
                    canvas.draw_rectangle(Rectangle::new(225., 132. + 54., 24., 30.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceLeft) {
                    canvas.draw_rectangle(Rectangle::new(195., 161., 30., 25.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceRight) {
                    canvas.draw_rectangle(Rectangle::new(195. + 54., 161., 30., 25.), Color::RED);
                }

                // Draw buttons: left-right back buttons
                if gamepad.is_button_down(GamepadButton::LeftTrigger1) {
                    canvas.draw_circle((239., 82.), 20., Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::RightTrigger1) {
                    canvas.draw_circle((557., 82.), 20., Color::RED);
                }

                // Draw axis: left joystick
                let left_gamepad_color = if gamepad.is_button_down(GamepadButton::LeftThumb) {
                    Color::RED
                } else {
                    Color::BLACK
                };
                canvas.draw_circle((319., 255.), 35., Color::BLACK);
                canvas.draw_circle((319., 255.), 31., Color::LIGHTGRAY);
                canvas.draw_circle(
                    (319. + (left_stick_x * 20.), 255. + (left_stick_y * 20.)),
                    25.,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let right_gamepad_color = if gamepad.is_button_down(GamepadButton::RightThumb) {
                    Color::RED
                } else {
                    Color::BLACK
                };
                canvas.draw_circle((475., 255.), 35., Color::BLACK);
                canvas.draw_circle((475., 255.), 31., Color::LIGHTGRAY);
                canvas.draw_circle(
                    (475. + (right_stick_x * 20.), 255. + (right_stick_y * 20.)),
                    25.,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                canvas.draw_rectangle(Rectangle::new(169., 48., 15., 70.), Color::GRAY);
                canvas.draw_rectangle(Rectangle::new(611., 48., 15., 70.), Color::GRAY);
                canvas.draw_rectangle(
                    Rectangle::new(169., 48., 15., ((1. + left_trigger) / 2.) * 70.),
                    Color::RED,
                );
                canvas.draw_rectangle(
                    Rectangle::new(611., 48., 15., ((1. + right_trigger) / 2.) * 70.),
                    Color::RED,
                );
            } else {
                // Draw background: generic
                canvas.draw_rectangle_rounded(
                    Rectangle::new(175., 110., 460., 220.),
                    0.3,
                    16,
                    Color::DARKGRAY,
                );

                // Draw buttons: basic
                canvas.draw_circle((365., 170.), 12., Color::RAYWHITE);
                canvas.draw_circle((405., 170.), 12., Color::RAYWHITE);
                canvas.draw_circle((445., 170.), 12., Color::RAYWHITE);
                canvas.draw_circle((516., 191.), 17., Color::RAYWHITE);
                canvas.draw_circle((551., 227.), 17., Color::RAYWHITE);
                canvas.draw_circle((587., 191.), 17., Color::RAYWHITE);
                canvas.draw_circle((551., 155.), 17., Color::RAYWHITE);
                if gamepad.is_button_down(GamepadButton::MiddleLeft) {
                    canvas.draw_circle((365., 170.), 10., Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::Middle) {
                    canvas.draw_circle((405., 170.), 10., Color::GREEN);
                }
                if gamepad.is_button_down(GamepadButton::MiddleRight) {
                    canvas.draw_circle((445., 170.), 10., Color::BLUE);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceLeft) {
                    canvas.draw_circle((516., 191.), 15., Color::GOLD);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceDown) {
                    canvas.draw_circle((551., 227.), 15., Color::BLUE);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceRight) {
                    canvas.draw_circle((587., 191.), 15., Color::GREEN);
                }
                if gamepad.is_button_down(GamepadButton::RightFaceUp) {
                    canvas.draw_circle((551., 155.), 15., Color::RED);
                }

                // Draw buttons: d-pad
                canvas.draw_rectangle(Rectangle::new(245., 145., 28., 88.), Color::RAYWHITE);
                canvas.draw_rectangle(Rectangle::new(215., 174., 88., 29.), Color::RAYWHITE);
                canvas.draw_rectangle(Rectangle::new(247., 147., 24., 84.), Color::BLACK);
                canvas.draw_rectangle(Rectangle::new(217., 176., 84., 25.), Color::BLACK);
                if gamepad.is_button_down(GamepadButton::LeftFaceUp) {
                    canvas.draw_rectangle(Rectangle::new(247., 147., 24., 29.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceDown) {
                    canvas.draw_rectangle(Rectangle::new(247., 147. + 54., 24., 30.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceLeft) {
                    canvas.draw_rectangle(Rectangle::new(217., 176., 30., 25.), Color::RED);
                }
                if gamepad.is_button_down(GamepadButton::LeftFaceRight) {
                    canvas.draw_rectangle(Rectangle::new(217. + 54., 176., 30., 25.), Color::RED);
                }

                // Draw buttons: left-right back
                canvas.draw_rectangle_rounded(
                    Rectangle::new(215., 98., 100., 10.),
                    0.5,
                    16,
                    Color::DARKGRAY,
                );
                canvas.draw_rectangle_rounded(
                    Rectangle::new(495., 98., 100., 10.),
                    0.5,
                    16,
                    Color::DARKGRAY,
                );
                if gamepad.is_button_down(GamepadButton::LeftTrigger1) {
                    canvas.draw_rectangle_rounded(
                        Rectangle::new(215., 98., 100., 10.),
                        0.5,
                        16,
                        Color::RED,
                    );
                }
                if gamepad.is_button_down(GamepadButton::RightTrigger1) {
                    canvas.draw_rectangle_rounded(
                        Rectangle::new(495., 98., 100., 10.),
                        0.5,
                        16,
                        Color::RED,
                    );
                }

                // Draw axis: left joystick
                let left_gamepad_color = if gamepad.is_button_down(GamepadButton::LeftThumb) {
                    Color::RED
                } else {
                    Color::BLACK
                };
                canvas.draw_circle((345., 260.), 40., Color::BLACK);
                canvas.draw_circle((345., 260.), 35., Color::LIGHTGRAY);
                canvas.draw_circle(
                    (345. + (left_stick_x * 20.), 260. + (left_stick_y * 20.)),
                    25.,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let right_gamepad_color = if gamepad.is_button_down(GamepadButton::RightThumb) {
                    Color::RED
                } else {
                    Color::BLACK
                };
                canvas.draw_circle((465., 260.), 40., Color::BLACK);
                canvas.draw_circle((465., 260.), 35., Color::LIGHTGRAY);
                canvas.draw_circle(
                    (465. + (right_stick_x * 20.), 260. + (right_stick_y * 20.)),
                    25.,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                canvas.draw_rectangle(Rectangle::new(151., 110., 15., 70.), Color::GRAY);
                canvas.draw_rectangle(Rectangle::new(644., 110., 15., 70.), Color::GRAY);
                canvas.draw_rectangle(
                    Rectangle::new(151., 110., 15., ((1. + left_trigger) / 2.) * 70.),
                    Color::RED,
                );
                canvas.draw_rectangle(
                    Rectangle::new(644., 110., 15., ((1. + right_trigger) / 2.) * 70.),
                    Color::RED,
                );
            }

            canvas.draw_text(
                format!("DETECTED AXIS [{}]:", gamepad.axis_count()),
                (10., 50.),
                10,
                Color::MAROON,
            );

            for (i, axis) in GamepadAxis::VARIANTS.iter().enumerate() {
                canvas.draw_text(
                    format!("AXIS {:?}: {:.2}", axis, gamepad.get_axis_movement(*axis)),
                    (20., 70. + 20. * i as f32),
                    10,
                    Color::DARKGRAY,
                );
            }

            // Draw vibrate button
            canvas.draw_rectangle(vibrate_button, Color::SKYBLUE);
            canvas.draw_text(
                "VIBRATE",
                (vibrate_button.x + 14., vibrate_button.y + 1.),
                10,
                Color::DARKGRAY,
            );

            if let Some(button) = Gamepad::get_button_pressed() {
                canvas.draw_text(
                    format!("DETECTED BUTTON: {:?}", button),
                    (10., 430.),
                    10,
                    Color::RED,
                );
            } else {
                canvas.draw_text("DETECTED BUTTON: NONE", (10., 430.), 10, Color::GRAY);
            }
        }
    }
}
