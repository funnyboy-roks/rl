use std::f32::consts::PI;

use rl::{math::Angle, prelude::*};

fn reset(ball_pos: &mut Vector2, ball_vel: &mut Vector2) {
    *ball_pos = Vector2::new(400., 300.);
    // random vector, biased towards a paddle
    *ball_vel = Vector2::new(400., 0.).rotate(
        Angle::degrees(rl::rand::random_value(-45, 45) as f32)
            + if bool::random() {
                Angle::radians(PI)
            } else {
                Angle::ZERO
            },
    );
}

fn main() {
    let mut window = Window::builder().title("hello world").size(800, 600).init();

    let ball_radius = 10.;
    let mut ball_pos = Vector2::ZERO;
    let mut ball_vel = Vector2::ZERO;
    reset(&mut ball_pos, &mut ball_vel);

    let player_speed = 500.;

    let player_height = 80.;
    let mut player1 = Rectangle::new(50., 300. - player_height / 2., 10., player_height);
    let mut player2 = Rectangle::new(800. - 60., 300. - player_height / 2., 10., player_height);

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut death_flash = 0.;

    while let Some(frame) = window.next_frame() {
        // update ball
        if ball_pos.y + ball_radius + ball_vel.y * frame.get_time() > frame.height() as _
            || ball_pos.y - ball_radius + ball_vel.y * frame.get_time() < 0.
        {
            ball_vel.y *= -0.99;
        }

        if ball_pos.x + ball_radius + ball_vel.x * frame.get_time() > frame.width() as _
            || ball_pos.x - ball_radius + ball_vel.x * frame.get_time() < 0.
        {
            death_flash = 0.5;

            if ball_pos.x < frame.width() as f32 / 2. {
                p2_score += 1;
            } else {
                p1_score += 1;
            }

            reset(&mut ball_pos, &mut ball_vel);
        }

        ball_pos += ball_vel * frame.get_time();

        // update player
        let player_speed = player_speed * frame.get_time();
        let mut player1_speed = 0.;
        if frame.keyboard().is_key_down(KeyboardKey::S)
            && player1.y + player1.height + player_speed < frame.height() as _
        {
            player1_speed += player_speed;
        }
        if frame.keyboard().is_key_down(KeyboardKey::W) && player1.y - player_speed > 0. {
            player1_speed -= player_speed;
        }

        let mut player2_speed = 0.;
        if frame.keyboard().is_key_down(KeyboardKey::J)
            && player2.y + player2.height + player_speed < frame.height() as _
        {
            player2_speed += player_speed;
        }
        if frame.keyboard().is_key_down(KeyboardKey::K) && player2.y - player_speed > 0. {
            player2_speed -= player_speed;
        }

        player1.y += player1_speed;
        player2.y += player2_speed;

        if player1.check_collision_circle_rec(ball_pos, ball_radius) {
            ball_pos.x = player1.x + player1.width + ball_radius;
            ball_vel.x *= -1.;
            ball_vel.y += player1_speed * 50.;
        }

        if player2.check_collision_circle_rec(ball_pos, ball_radius) {
            ball_pos.x = player2.x - ball_radius;
            ball_vel.x *= -1.;
            ball_vel.y += player2_speed * 50.;
        }

        if death_flash > f32::EPSILON {
            death_flash -= frame.get_time();
        } else {
            death_flash = 0.;
        }

        let mut canvas = frame.begin_drawing();

        canvas.clear_background(Color::new(0x18, 0x18, 0x18, 0xff));

        if death_flash > f32::EPSILON {
            canvas.draw_rectangle(canvas.bounds(), Color::MAROON.alpha(death_flash / 0.5));
        }

        canvas.draw_text(
            format!("{}", p1_score),
            Vector2::new(50. + 10. + 10., 10.),
            64,
            Color::RED,
        );

        let p2_score_text = format!("{}", p2_score);
        let p2_score_width = rl::text::measure(&p2_score_text, 64);
        canvas.draw_text(
            p2_score_text,
            Vector2::new(
                canvas.width() as f32 - 50. - 10. - 10. - p2_score_width as f32,
                10.,
            ),
            64,
            Color::GREEN,
        );

        canvas.draw_circle(ball_pos, 10., Color::RAYWHITE);

        canvas.draw_rectangle(player1, Color::RED);
        canvas.draw_rectangle(player2, Color::GREEN);
    }
}
