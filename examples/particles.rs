// https://github.com/raysan5/raylib/blob/master/examples/core/core_basic_window.c
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

        self.age += frame.get_time();
        self.rotation += 0.1;

        true
    }
}

fn main() {
    let mut window = Window::init(800, 600, "particles");
    window.set_target_fps(60);

    let mut particles: Vec<Particle> = Vec::with_capacity(1000);

    while let Some(mut frame) = window.next_frame() {
        frame.clear_background(Color::BLACK);

        particles.retain_mut(|p| p.draw(&mut frame));

        if frame
            .mouse()
            .is_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            || frame
                .mouse()
                .is_button_down(MouseButton::MOUSE_BUTTON_RIGHT)
        {
            let pos = frame.mouse().position();
            let colour = Color::color_from_hsv(f32::random() * 360., 1., 1.);
            for _ in 0..1000 {
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

        frame.draw_fps(10, 10);
        frame.draw_text(
            format!("Particles: {}", particles.len()),
            (10., 10. + 20. + 5.),
            20,
            Color::RAYWHITE,
        );
    }
}
