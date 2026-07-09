use std::collections::VecDeque;

use rl::prelude::*;

struct Particle {
    pos: Vector2,
    velocity: Vector2,
    size: f32,
    rotation: f32,
    age: f32,
    max_age: f32,
    color: Color,
}

impl Particle {
    fn draw(&mut self, frame: &mut rl::Frame<'_>) -> bool {
        if self.age > self.max_age {
            return false;
        }

        let rect = Rectangle {
            x: self.pos.x,
            y: self.pos.y,
            width: self.size,
            height: self.size,
        };

        frame
            .draw_rectangle_builder()
            .rectangle(rect)
            .rotation((self.size / 2., self.size / 2.), self.rotation)
            .color(self.color.alpha(1. - self.age / self.max_age))
            .draw();

        if !frame.bounds().check_collision_recs(rect) {
            return false;
        }

        self.pos += self.velocity * frame.get_time() * (1.5 - self.age / self.max_age);

        self.velocity += Vector2::new(0., 9.8) * 50. * frame.get_time();

        self.age += frame.get_time();
        self.rotation += 0.1;

        true
    }
}

fn generate_particles(particles: &mut VecDeque<Particle>, n: usize, pos: Vector2, colour: Color) {
    for _ in 0..n {
        let dir = Vector2::random();
        particles.insert(
            (f32::random() * particles.len() as f32) as usize,
            Particle {
                pos: pos + dir * 20. * f32::random(),
                velocity: dir * 100. * f32::random(),
                size: f32::random() * 10.,
                rotation: f32::random() * 90.,
                age: f32::random() * 3.0 + 1.0,
                max_age: 4.0,
                color: colour,
            },
        );
    }
}

fn main() {
    let mut window = Window::init(800, 600, "particles");
    window.set_target_fps(60);

    let mut particles: VecDeque<Particle> = VecDeque::with_capacity(1000);

    let mut prev_mouse = Vector2::zero();
    while let Some(mut frame) = window.next_frame() {
        frame.clear_background(Color::BLACK);

        particles.retain_mut(|p| p.draw(&mut frame));

        let mouse = frame.mouse().position();

        if frame
            .mouse()
            .is_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let colour = Color::color_from_hsv(f32::random() * 360., 1., 1.);
            generate_particles(&mut particles, 1000, mouse, colour);
        }

        if frame
            .mouse()
            .is_button_down(MouseButton::MOUSE_BUTTON_RIGHT)
        {
            let prev_to_curr = mouse - prev_mouse;
            for i in 0..=1000 {
                let colour = Color::color_from_hsv(f32::random() * 360., 1., 1.);
                let pos = prev_mouse.lerp(mouse, i as f32 / 1000.);

                let dir = Vector2::random() + prev_to_curr * 0.05;
                particles.insert(
                    (f32::random() * particles.len() as f32) as usize,
                    Particle {
                        pos: pos + dir * 20. * f32::random(),
                        velocity: dir * 100. * f32::random(),
                        size: f32::random() * 10.,
                        rotation: f32::random() * 90.,
                        age: f32::random() * 3.0 + 1.0,
                        max_age: 4.0,
                        color: colour,
                    },
                );
            }
        }

        frame.draw_fps(10, 10);
        frame.draw_text(
            format!("Particles: {}", particles.len()),
            (10., 10. + 20. + 5.),
            20,
            Color::RAYWHITE,
        );

        prev_mouse = frame.mouse().position();
    }
}
