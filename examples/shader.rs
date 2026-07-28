use rl::prelude::*;

fn main() {
    let mut win = Window::builder().title("Shader").size(800, 600).init();

    let shader = shader! {
        fragment {
            { #version 330 }
            {
                in vec2 fragTexCoord;
                in vec4 fragColor;

                // Input uniform values
                uniform sampler2D texture0;
                uniform vec4 colDiffuse;

                // Output fragment color
                out vec4 finalColor;

                void main()
                {
                    vec4 texelColor = texture(texture0, fragTexCoord)*colDiffuse*fragColor;
                    finalColor = vec4(0.0, fragTexCoord.x, fragTexCoord.y, texelColor.a);
                }
            }
        }
    }
    .expect("Invalid shader");

    while let Some(mut frame) = win.next_frame() {
        frame.clear_background(Color::get_color(0x181818ff));
        let handle = shader.begin_mode();
        // frame.draw_text("hello world", (400., 300.), 32, Color::RED);
        frame.draw_rectangle(frame.bounds(), Color::WHITE);
        drop(handle);
    }
}
