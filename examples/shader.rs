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

    while let Some(frame) = win.next_frame() {
        let mut canvas = frame.begin_drawing();
        canvas.clear_background(Color::from_int(0x181818ff));
        shader.with(|| {
            // frame.draw_text("hello world", (400., 300.), 32, Color::RED);
            canvas.draw_rectangle(canvas.bounds(), Color::WHITE);
        });
    }
}
