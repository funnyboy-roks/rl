use std::collections::{HashSet, VecDeque};

use rl::{prelude::*, rand::random_value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn update_player(self, mut player: (u32, u32), dimension: (u32, u32)) -> (u32, u32) {
        match self {
            Dir::Up => player.1 = player.1.checked_sub(1).unwrap_or(dimension.1 - 1),
            Dir::Down => player.1 = (player.1 + 1) % dimension.1,
            Dir::Left => player.0 = player.0.checked_sub(1).unwrap_or(dimension.0 - 1),
            Dir::Right => player.0 = (player.0 + 1) % dimension.0,
        }
        (player.0 % dimension.0, player.1 % dimension.1)
    }
}

fn main() {
    let mut window = Window::init_with_flags(800, 600, "Snake", ConfigFlags::WINDOW_RESIZABLE);

    window.set_target_fps(60);

    let mut snek = VecDeque::new();

    let mut fruit = HashSet::new();

    let cell_size = 50;
    let width = window.width() / cell_size;
    let height = window.height() / cell_size;

    snek.push_front((width / 2, height / 2));

    for _ in 0..10 {
        fruit.insert((
            random_value(0, width as i32 - 1) as u32,
            random_value(0, width as i32 - 1) as u32,
        ));
    }

    let mut dir = Dir::Right;

    let mut dead = false;

    while let Some(mut frame) = window.next_frame() {
        let width = frame.width() / cell_size;
        let height = frame.height() / cell_size;
        let pad_x = frame.width() % cell_size;
        let pad_y = frame.height() % cell_size;

        if frame.is_key_pressed(KeyboardKey::KEY_DOWN) && dir != Dir::Up {
            dir = Dir::Down
        }

        if frame.is_key_pressed(KeyboardKey::KEY_UP) && dir != Dir::Down {
            dir = Dir::Up
        }

        if frame.is_key_pressed(KeyboardKey::KEY_RIGHT) && dir != Dir::Left {
            dir = Dir::Right;
        }

        if frame.is_key_pressed(KeyboardKey::KEY_LEFT) && dir != Dir::Right {
            dir = Dir::Left
        }

        if dead && frame.is_key_pressed(KeyboardKey::KEY_SPACE) {
            dir = Dir::Right;
            dead = false;
            fruit.clear();
            for _ in 0..10 {
                fruit.insert((
                    random_value(0, width as i32 - 1) as u32,
                    random_value(0, width as i32 - 1) as u32,
                ));
            }
            snek.clear();
            snek.push_front((width / 2, height / 2));
        }

        if !dead && frame.count() % 5 == 4 {
            let next = dir.update_player(snek[0], (width, height));
            if fruit.remove(&snek[0]) {
                fruit.insert((
                    random_value(0, width as i32 - 1) as u32,
                    random_value(0, width as i32 - 1) as u32,
                ));
            } else {
                snek.pop_back();
            }

            if snek.contains(&next) {
                dead = true;
            } else {
                snek.push_front(next);
            }
        }

        frame.clear_background(Color::new(0x25, 0x25, 0x25, 0x1));
        for y in 0..height {
            for x in 0..width {
                if fruit.contains(&(x, y)) {
                    frame.draw_rectangle(
                        Rectangle {
                            x: (pad_x / 2 + x * cell_size) as f32 + 15.,
                            y: (pad_y / 2 + y * cell_size) as f32 + 15.,
                            width: cell_size as f32 - 30.,
                            height: cell_size as f32 - 30.,
                        },
                        Color::DARKRED,
                    );
                }

                for (i, &cell) in snek.iter().enumerate() {
                    if (x, y) == cell {
                        frame.draw_rectangle(
                            Rectangle {
                                x: (pad_x / 2 + x * cell_size) as f32,
                                y: (pad_y / 2 + y * cell_size) as f32,
                                width: cell_size as _,
                                height: cell_size as _,
                            },
                            if i == 0 {
                                Color::GREEN
                            } else if i % 2 == 0 {
                                Color::DARKGREEN
                            } else {
                                Color::DARKGREEN.alpha(0.5)
                            },
                        );
                    }
                }

                frame.draw_rectangle_lines(
                    Rectangle {
                        x: (pad_x / 2 + x * cell_size) as f32,
                        y: (pad_y / 2 + y * cell_size) as f32,
                        width: cell_size as _,
                        height: cell_size as _,
                    },
                    1.,
                    Color::new(0x40, 0x40, 0x40, 0xff),
                );
            }
        }

        if dead {
            frame.draw_rectangle(frame.bounds(), Color::BLACK.alpha(0.25));

            let text = "You Died";
            let sz = 125;
            let width = rl::text::measure(text, sz);
            frame.draw_text(
                text,
                Vector2::new(
                    frame.width() as f32 / 2. - width as f32 / 2.,
                    frame.height() as f32 / 2. - sz as f32 - 40.,
                ),
                sz,
                Color::RAYWHITE,
            );

            let score = format!("Score: {}", snek.len());
            let sz = 64;
            let width = rl::text::measure(&score, sz);
            frame.draw_text(
                score,
                Vector2::new(
                    frame.width() as f32 / 2. - width as f32 / 2.,
                    frame.height() as f32 / 2. - sz as f32 / 2.,
                ),
                sz,
                Color::RAYWHITE,
            );

            let text = "Press [SPACE] to play again";
            let sz = 32;
            let width = rl::text::measure(text, sz);
            frame.draw_text(
                text,
                Vector2::new(
                    frame.width() as f32 / 2. - width as f32 / 2.,
                    frame.height() as f32 / 2. + 40.,
                ),
                sz,
                Color::RAYWHITE,
            );
        }
    }
}
