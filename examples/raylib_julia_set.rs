//! https://github.com/raysan5/raylib/blob/master/examples/shaders/shaders_julia_set.c

use rl::prelude::*;

// A few good julia sets
const POINTS_OF_INTEREST: [Vector2; 6] = [
    Vector2::new(-0.348827, 0.607167),
    Vector2::new(-0.786268, 0.169728),
    Vector2::new(-0.8, 0.156),
    Vector2::new(0.285, 0.0),
    Vector2::new(-0.835, -0.2321),
    Vector2::new(-0.70176, -0.3842),
];

const ZOOM_SPEED: f32 = 1.01;
const OFFSET_SPEED_MUL: f32 = 2.0;

const STARTING_ZOOM: f32 = 0.75;

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
fn main() {
    let mut window = Window::builder()
        .title("raylib [shaders] example - julia set")
        .size(800, 600)
        .flags(ConfigFlags::WINDOW_RESIZABLE)
        .init();

    let shader = shader! {
        fragment {
            { #version 330 }
            {
                // Input vertex attributes (from vertex shader)
                in vec2 fragTexCoord;
                in vec4 fragColor;

                // Output fragment color
                out vec4 finalColor;

                uniform vec2 c;                 // c.x = real, c.y = imaginary component. Equation done is z^2 + c
                uniform vec2 offset;            // Offset of the scale
                uniform float zoom;             // Zoom of the scale

                const int maxIterations = 255;  // Max iterations to do
                const float colorCycles = 2.0;  // Number of times the color palette repeats. Can show higher detail for higher iteration numbers

                // Square a complex number
                vec2 ComplexSquare(vec2 z) {
                    return vec2(z.x*z.x - z.y*z.y, z.x*z.y*2.0);
                }

                // Convert Hue Saturation Value (HSV) color into RGB
                vec3 Hsv2rgb(vec3 c) {
                    vec4 K = vec4(1.0, 2.0/3.0, 1.0/3.0, 3.0);
                    vec3 p = abs(fract(c.xxx + K.xyz)*6.0 - K.www);
                    return c.z*mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
                }

                void main() {
                    /**********************************************************************************************
                      Julia sets use a function z^2 + c, where c is a constant
                      This function is iterated until the nature of the point is determined

                      If the magnitude of the number becomes greater than 2, then from that point onward
                      the number will get bigger and bigger, and will never get smaller (tends towards infinity)
                      2^2 = 4, 4^2 = 8 and so on
                      So at 2 we stop iterating

                      If the number is below 2, we keep iterating
                      But when do we stop iterating if the number is always below 2 (it converges)?
                      That is what maxIterations is for
                      Then we can divide the iterations by the maxIterations value to get a normalized value
                      that we can then map to a color

                      We use dot product (z.x*z.x + z.y*z.y) to determine the magnitude (length) squared
                      And once the magnitude squared is > 4, then magnitude > 2 is also true (saves computational power)
                    *************************************************************************************************/

                    // The pixel coordinates are scaled so they are on the mandelbrot scale
                    // NOTE: fragTexCoord already comes as normalized screen coordinates but offset must be normalized before scaling and zoom
                    vec2 z = vec2((fragTexCoord.x - 0.5f)*2.5, (fragTexCoord.y - 0.5)*1.5)/zoom;
                    z.x += offset.x;
                    z.y += offset.y;

                    int iterations = 0;
                    for (iterations = 0; iterations < maxIterations; iterations++)
                    {
                        z = ComplexSquare(z) + c;  // Iterate function

                        if (dot(z, z) > 4.0) break;
                    }

                    // Another few iterations decreases errors in the smoothing calculation
                    // See http://linas.org/art-gallery/escape/escape.html for more information
                    z = ComplexSquare(z) + c;
                    z = ComplexSquare(z) + c;

                    // This last part smooths the color (again see link above)
                    float smoothVal = float(iterations) + 1.0 - (log(log(length(z)))/log(2.0));

                    // Normalize the value so it is between 0 and 1
                    float norm = smoothVal/float(maxIterations);

                    // If in set, color black. 0.999 allows for some float accuracy error
                    if (norm > 0.999) finalColor = vec4(0.0, 0.0, 0.0, 1.0);
                    else finalColor = vec4(Hsv2rgb(vec3(norm*colorCycles, 1.0, 1.0)), 1.0);
                }
            }
        }
    }.expect("Invalid shader");

    // Create a RenderTexture2D to be used for render to texture
    let mut target = RenderTexture2D::new(window.width(), window.height());

    // c constant to use in z^2 + c
    let mut c = POINTS_OF_INTEREST[0];

    // Offset and zoom to draw the julia set at. (centered on screen and default size)
    let mut offset = Vector2::ZERO;
    let mut zoom = STARTING_ZOOM;

    // Get variable (uniform) locations on the shader to connect with the program
    // NOTE: If uniform variable could not be found in the shader, function returns -1
    let mut c_loc = shader.get_location::<Vector2>("c").expect("valid");
    let mut zoom_loc = shader.get_location::<f32>("zoom").expect("valid");
    let mut offset_loc = shader.get_location::<Vector2>("offset").expect("valid");

    // Upload the shader uniform values!
    c_loc.set(c);
    zoom_loc.set(zoom);
    offset_loc.set(offset);

    let mut increment_speed = 0.; // Multiplier of speed to change c value
    let mut show_controls = true; // Show controls

    window.set_target_fps(60); // Set our game to run at 60 frames-per-second

    //--------------------------------------------------------------------------------------

    // Main game loop
    while let Some(frame) = window.next_frame() {
        if frame.size() != target.size() {
            drop(target);
            target = RenderTexture2D::new(frame.width(), frame.height());
        }

        target.draw_with(|t| {
            t.clear_background(Color::BLACK);
            // Draw a rectangle in shader mode to be used as shader canvas
            // NOTE: Rectangle uses font white character texture coordinates,
            // so shader can not be applied here directly because input vertexTexCoord
            // do not represent full screen coordinates (space where want to apply shader)
            t.draw_rectangle(t.bounds(), Color::BLACK);
        });

        if frame.keyboard().is_key_pressed(Key::One)
            || frame.keyboard().is_key_pressed(Key::Two)
            || frame.keyboard().is_key_pressed(Key::Three)
            || frame.keyboard().is_key_pressed(Key::Four)
            || frame.keyboard().is_key_pressed(Key::Five)
            || frame.keyboard().is_key_pressed(Key::Six)
        {
            c = match () {
                _ if frame.keyboard().is_key_pressed(Key::One) => POINTS_OF_INTEREST[0],
                _ if frame.keyboard().is_key_pressed(Key::Two) => POINTS_OF_INTEREST[1],
                _ if frame.keyboard().is_key_pressed(Key::Three) => POINTS_OF_INTEREST[2],
                _ if frame.keyboard().is_key_pressed(Key::Four) => POINTS_OF_INTEREST[3],
                _ if frame.keyboard().is_key_pressed(Key::Five) => POINTS_OF_INTEREST[4],
                _ if frame.keyboard().is_key_pressed(Key::Six) => POINTS_OF_INTEREST[5],
                _ => unreachable!(),
            };

            c_loc.set(c);
        }

        // If "R" is pressed, reset zoom and offset
        if frame.keyboard().is_key_pressed(Key::R) {
            zoom = STARTING_ZOOM;
            offset = Vector2::ZERO;

            zoom_loc.set(zoom);
            offset_loc.set(offset);
        }

        if frame.keyboard().is_key_pressed(Key::Space) {
            increment_speed = 0.
        } // Pause animation (c change)
        if frame.keyboard().is_key_pressed(Key::F1) {
            show_controls = !show_controls
        } // Toggle whether or not to show controls

        if frame.keyboard().is_key_pressed(Key::Right) {
            increment_speed += 1.
        } else if frame.keyboard().is_key_pressed(Key::Left) {
            increment_speed -= 1.
        }

        // If either left or right button is pressed, zoom in/out
        if frame.mouse().is_button_down(MouseButton::Left)
            || frame.mouse().is_button_down(MouseButton::Right)
        {
            // Change zoom. If Mouse left -> zoom in. Mouse right -> zoom out
            if frame.mouse().is_button_down(MouseButton::Left) {
                zoom *= ZOOM_SPEED;
            } else {
                zoom /= ZOOM_SPEED;
            }

            let mouse_pos = frame.mouse().position();
            // Find the velocity at which to change the camera. Take the distance of the mouse
            // from the center of the screen as the direction, and adjust magnitude based on the current zoom
            let offset_velocity = Vector2 {
                x: (mouse_pos.x / frame.width() as f32 - 0.5) * OFFSET_SPEED_MUL / zoom,
                y: (mouse_pos.y / frame.height() as f32 - 0.5) * OFFSET_SPEED_MUL / zoom,
            };

            // Apply move velocity to camera
            offset += offset_velocity * frame.get_time();

            // Update the shader uniform values!
            zoom_loc.set(zoom);
            offset_loc.set(offset);
        }

        // Increment c value with time
        let dc = frame.get_time() * increment_speed * 0.0005;
        c += Vector2::new(dc, dc);
        c_loc.set(c);
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        // Using a render texture to draw Julia set

        let mut canvas = frame.begin_drawing();

        canvas.clear_background(Color::BLACK); // Clear screen background

        // Draw the saved texture and rendered julia set with shader
        // NOTE: We do not invert texture on Y, already considered inside shader
        shader.with(|| {
            // WARNING: If FLAG_WINDOW_HIGHDPI is enabled, HighDPI monitor scaling should be considered
            // when rendering the RenderTexture2D to fit in the HighDPI scaled Window
            canvas.draw_texture(&target.texture(), Vector2::ZERO, 0., 1., Color::WHITE);
        });

        if show_controls {
            canvas.draw_text(
                "Press Mouse buttons right/left to zoom in/out and move",
                (10., 15.),
                10,
                Color::RAYWHITE,
            );
            canvas.draw_text(
                "Press F1 to toggle these controls",
                (10., 30.),
                10,
                Color::RAYWHITE,
            );
            canvas.draw_text(
                "Press KEYS [1 - 6] to change point of interest",
                (10., 45.),
                10,
                Color::RAYWHITE,
            );
            canvas.draw_text(
                "Press LEFT | RIGHT to change speed",
                (10., 60.),
                10,
                Color::RAYWHITE,
            );
            canvas.draw_text(
                "Press SPACE to stop movement animation",
                (10., 75.),
                10,
                Color::RAYWHITE,
            );
            canvas.draw_text(
                "Press R to recenter the camera",
                (10., 90.),
                10,
                Color::RAYWHITE,
            );
            canvas.draw_fps(0, 0);
        }
        // EndDrawing();
    }
}
