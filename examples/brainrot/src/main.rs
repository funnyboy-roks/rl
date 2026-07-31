use rl::{math::shapes::BoundingBox, prelude::*, shader::ShaderLocation};

struct Light {
    position: Vector3,
    target: Vector3,
    color: Color,

    // Shader locations
    position_loc: ShaderLocation<Vector3>,
    target_loc: ShaderLocation<Vector3>,
    color_loc: ShaderLocation<Vector4>,
}

impl Light {
    fn new(position: Vector3, target: Vector3, color: Color, shader: &Shader) -> Self {
        let pos_loc = shader.get_location("sun.position").unwrap();
        let target_loc = shader.get_location("sun.target").unwrap();
        let color_loc = shader.get_location("sun.color").unwrap();

        Self {
            position,
            target,
            color,
            position_loc: pos_loc,
            target_loc,
            color_loc,
        }
    }

    fn update(&mut self) {
        self.position_loc.set(self.position);
        self.target_loc.set(self.target);
        let [r, g, b, a] = self.color.to_normalized();
        self.color_loc.set(Vector4::new(r, g, b, a));
    }

    fn shader() -> Shader {
        shader! {
            vertex {
                { #version 330 }
                {
                    // Input vertex attributes
                    in vec3 vertexPosition;
                    in vec2 vertexTexCoord;
                    in vec3 vertexNormal;
                    in vec4 vertexColor;

                    // Input uniform values
                    uniform mat4 mvp;
                    uniform mat4 matModel;
                    uniform mat4 matNormal;

                    // Output vertex attributes (to fragment shader)
                    out vec3 fragPosition;
                    out vec2 fragTexCoord;
                    out vec4 fragColor;
                    out vec3 fragNormal;

                    // NOTE: Add your custom variables here

                    void main()
                    {
                        // Send vertex attributes to fragment shader
                        fragPosition = vec3(matModel*vec4(vertexPosition, 1.0));
                        fragTexCoord = vertexTexCoord;
                        fragColor = vertexColor;
                        fragNormal = normalize(vec3(matNormal*vec4(vertexNormal, 1.0)));

                        // Calculate final vertex position
                        gl_Position = mvp*vec4(vertexPosition, 1.0);
                    }
                }
            }
            fragment {
                { #version 330 }
                {
                    // Input vertex attributes (from vertex shader)
                    in vec3 fragPosition;
                    in vec2 fragTexCoord;
                    in vec4 fragColor;
                    in vec3 fragNormal;

                    // Input uniform values
                    uniform sampler2D texture0;
                    uniform vec4 colDiffuse;

                    // Output fragment color
                    out vec4 finalColor;

                    struct Light {
                        vec3 position;
                        vec3 target;
                        vec4 color;
                    };

                    // Input lighting values
                    uniform Light sun;
                    uniform vec4 ambient;
                    uniform vec3 viewPos;

                    void main()
                    {
                        // Texel color fetching from texture sampler
                        vec4 texelColor = texture(texture0, fragTexCoord);
                        vec3 lightDot = vec3(0.0);
                        vec3 normal = normalize(fragNormal);
                        vec3 viewD = normalize(viewPos - fragPosition);
                        vec3 specular = vec3(0.0);

                        vec4 tint = colDiffuse*fragColor;

                        // NOTE: Implement here your fragment shader code

                        vec3 light = -normalize(sun.target - sun.position);

                        float NdotL = max(dot(normal, light), 0.0);
                        lightDot += sun.color.rgb*NdotL;

                        float specCo = 0.0;
                        if (NdotL > 0.0) specCo = pow(max(0.0, dot(viewD, reflect(-(light), normal))), 16.0); // 16 refers to shine
                        specular += specCo;

                        finalColor = (texelColor*((tint + vec4(specular, 1.0))*vec4(lightDot, 1.0)));
                        finalColor += texelColor*(ambient/10.0)*tint;

                        // Gamma correction
                        finalColor = pow(finalColor, vec4(1.0/2.2));
                    }
                }
            }
        }
        .unwrap()
    }
}

struct Side {
    effect: i32,
    collided: bool,
}

impl Side {
    fn random_value() -> i32 {
        if rl::rand::random_value(1, 6) == 1 {
            return 0;
        }

        if rl::rand::random_value(0, 1) == 1 {
            rl::rand::random_value(-100, -1)
        } else {
            rl::rand::random_value(1, 50)
        }
    }

    fn new() -> Self {
        Side {
            effect: Self::random_value(),
            collided: false,
        }
    }

    fn color(&self) -> Color {
        if self.collided {
            Color::RAYWHITE
        } else if self.effect < 0 {
            Color::RED
        } else {
            Color::GREEN
        }
    }

    fn collide(&mut self, score: &mut i32) -> bool {
        if self.collided {
            return false;
        }
        *score += self.effect;
        self.collided = true;
        true
    }

    fn draw(
        &mut self,
        cam: &mut impl DrawTarget3D,
        center: Vector3,
        player_box: BoundingBox,
        score: &mut i32,
        // good_sound: Sound,
        // bad_sound: Sound,
    ) {
        if self.effect == 0 || self.collided {
            return;
        }

        cam.draw_cube_wires(center, TARGET_SIZE, self.color());
        cam.draw_cube(center, TARGET_SIZE, self.color().brightness(0.5));
        // TODO:
        let s = format!("{:+}", self.effect);
        draw_score_text(center, &s);

        let bounds = BoundingBox::new(center - TARGET_SIZE / 2., center + TARGET_SIZE / 2.);

        if player_box.collides_box(bounds) && self.collide(score) {
            // TODO:
            // if self.effect > 0 {
            //     rl::playSound(good);
            // } else {
            //     rl::playSound(bad);
            // }
        }
    }
}

const TARGET_SIZE: Vector3 = Vector3::new(0.25, 3., 10.);

struct Target {
    pos: Vector3,
    left: Side,
    right: Side,
}

const LETTER_BOUNDRY_SIZE: f32 = 0.25;
const LETTER_BOUNDRY_COLOR: Color = Color::VIOLET;
const SHOW_LETTER_BOUNDRY: bool = false;

fn draw_text_codepoint_3d(
    font: &Font,
    c: char,
    mut position: Vector3,
    font_size: f32,
    backface: bool,
    tint: Color,
) {
    // Character index position in sprite font
    // NOTE: In case a codepoint is not available in the font, index returned points to '?'
    let index = font.get_glyph_index(c);
    let scale = font_size / font.base_size() as f32;

    // Character destination rectangle on screen
    // NOTE: We consider charsPadding on drawing
    position.x += (font.glyphs()[index].offset_x() - font.glyph_padding()) as f32 * scale;
    position.z += (font.glyphs()[index].offset_y() - font.glyph_padding()) as f32 * scale;

    // Character source rectangle from font texture atlas
    // NOTE: We consider chars padding when drawing, it could be required for outline/glow shader effects
    let rec = font.recs()[index];
    let pad = font.glyph_padding() as f32;
    let src_rec = Rectangle {
        x: rec.x - pad,
        y: rec.y - pad,
        width: rec.width + 2. * pad,
        height: rec.height + 2. * pad,
    };

    let width = (rec.width + 2. * pad) * scale;
    let height = (rec.height + 2. * pad) * scale;

    let x = 0.;
    let y = 0.;
    let z = 0.;

    let texture = font.texture();
    // normalized texture coordinates of the glyph inside the font texture (0.0f -> 1.0f)
    let tx = src_rec.x / texture.width() as f32;
    let ty = src_rec.y / texture.height() as f32;
    let tw = (src_rec.x + src_rec.width) / texture.width() as f32;
    let th = (src_rec.y + src_rec.height) / texture.height() as f32;

    // if (SHOW_LETTER_BOUNDRY) {
    //     rl::drawCubeWiresV(
    //         { position.x + width/2, position.y, position.z + height/2},
    //         { width, LETTER_BOUNDRY_SIZE, height },
    //         LETTER_BOUNDRY_COLOR
    //     );
    // }

    rl::rlgl::check_render_batch_limit(4 + if backface { 4 } else { 0 });
    rl::rlgl::set_texture(texture);

    rl::rlgl::with_matrix(|mat| {
        mat.translate(position);

        rl::rlgl::drawing_mode(rl::rlgl::DrawingMode::Quads, |ctx| {
            ctx.color(tint);

            // Front Face
            ctx.normal(Vector3::UNIT_Y); // Normal Pointing Up
            ctx.tex_coord((tx, ty)).vertex((x, y, z)); // Top Left Of The Texture and Quad
            ctx.tex_coord((tx, th)).vertex((x, y, z + height)); // Bottom Left Of The Texture and Quad
            ctx.tex_coord((tw, th)).vertex((x + width, y, z + height)); // Bottom Right Of The Texture and Quad
            ctx.tex_coord((tw, ty)).vertex((x + width, y, z)); // Top Right Of The Texture and Quad

            if backface {
                // Back Face
                ctx.normal(-Vector3::UNIT_Y); // Normal Pointing Down
                ctx.tex_coord((tx, ty)).vertex((x, y, z)); // Top Right Of The Texture and Quad
                ctx.tex_coord((tw, ty)).vertex((x + width, y, z)); // Top Left Of The Texture and Quad
                ctx.tex_coord((tw, th)).vertex((x + width, y, z + height)); // Bottom Left Of The Texture and Quad
                ctx.tex_coord((tx, th)).vertex((x, y, z + height)); // Bottom Right Of The Texture and Quad
            }
        });
    });

    rl::rlgl::unset_texture();
}

fn draw_text_3d(
    font: &Font,
    text: &str,
    position: Vector3,
    font_size: f32,
    font_spacing: f32,
    line_spacing: f32,
    backface: bool,
    tint: Color,
) {
    let mut text_offset_y = 0.; // Offset between lines (on line break '\n')
    let mut text_offset_x = 0.; // Offset X to next character to draw

    let scale = font_size / font.base_size() as f32;

    for c in text.chars() {
        let index = font.get_glyph_index(c);

        // NOTE: Normally we exit the decoding sequence as soon as a bad byte is found (and return 0x3f)
        // but we need to draw all of the bad bytes using the '?' symbol moving one byte

        if c == '\n' {
            // NOTE: Fixed line spacing of 1.5 line-height
            // TODO: Support custom line spacing defined by user
            text_offset_y += font_size + line_spacing;
            text_offset_x = 0.;
        } else {
            if (c != ' ') && (c != '\t') {
                draw_text_codepoint_3d(
                    font,
                    c,
                    position + Vector3::new(text_offset_x, 0., text_offset_y),
                    font_size,
                    backface,
                    tint,
                );
            }

            if font.glyphs()[index].advance_x() == 0 {
                text_offset_x += font.recs()[index].width * scale + font_spacing;
            } else {
                text_offset_x += font.glyphs()[index].advance_x() as f32 * scale + font_spacing;
            }
        }
    }
}

fn draw_score_text(position: Vector3, text: &str) {
    let font = Font::default();

    let font_size = font.measure_text(text, 1.6, 0.1);

    rl::rlgl::with_matrix(|matrix| {
        matrix.translate(position + Vector3::new(0.15, font_size.y / 2., font_size.x / 2.));
        matrix.rotate(Angle::degrees(90.), Vector3::new(1., 0., 0.));
        matrix.rotate(Angle::degrees(90.), Vector3::new(0., 0., -1.));
        draw_text_3d(
            &font,
            text,
            Vector3::ZERO,
            1.6,
            0.1,
            0.0,
            false,
            Color::BLACK,
        );
    });
}

fn draw_score(score: i32, canvas: &mut (impl DrawTarget2DFull + Bounded)) {
    let score_size = canvas.height() / 10;
    let score_text = score.to_string();
    let score_width = rl::text::measure(&score_text, score_size);
    canvas.draw_text(
        score_text,
        ((canvas.width() / 2 - score_width / 2) as f32, 10.),
        score_size,
        Color::BLACK,
    );
}

fn main() {
    let mut window = Window::builder()
        .size(800, 600)
        .title("Brainrot")
        .flags(ConfigFlags::MSAA_4X_HINT)
        .target_fps(60)
        .init();

    let mut position = Vector3::new(15., 0.25, 0.);
    let player_height = 2.;
    let mut paused = false;

    let mut targets = Vec::new();

    let start = 10.;
    let gap = 30.;
    for i in 0..10 {
        targets.push(Target {
            pos: Vector3::new(-(start + i as f32 * gap), 0.25, 0.),
            left: Side::new(),
            right: Side::new(),
        });
    }

    let camera = Camera3D::builder()
        .position((30., 10., 0.))
        .target(Vector3::ZERO)
        .up(Vector3::UNIT_Y)
        .fovy(Angle::degrees(45.))
        .projection(CameraProjection::Perspective)
        .build();

    let shader = Light::shader();
    // shader.locs[rl::SHADER_LOC_VECTOR_VIEW] = rl::getShaderLocation(shader, "viewPos");
    let mut view_loc = shader.get_location::<Vector3>("viewPos").unwrap();

    let mut ambient_loc = shader.get_location::<Vector4>("ambient").unwrap();
    ambient_loc.set(Vector4::ONE);

    let mut sun = Light::new(
        Vector3::new(30., 10., -10.),
        Vector3::ZERO,
        Color::WHITE,
        &shader,
    );

    window.disable_cursor();

    let mut score = 0i32;

    while let Some(frame) = window.next_frame() {
        view_loc.set(camera.position);

        sun.update();

        let speed = Vector3::new(0., 0., 12.);
        let max = 10. - player_height / 2.;
        let min = -10. + player_height / 2.;

        if frame.keyboard().is_key_down(Key::Left) {
            position += speed * frame.get_time();
            position.z = position.z.clamp(min, max)
        }

        if frame.keyboard().is_key_down(Key::Right) {
            position -= speed * frame.get_time();
            position.z = position.z.clamp(min, max)
        }

        if frame.keyboard().is_key_down(Key::Space) {
            paused = !paused;
        }

        // TODO:
        // if frame.keyboard().is_key_down(Key::S) {
        //     Image screen = rl::loadImageFromScreen();
        //     rl::exportImage(screen, "img/screenshot.png");
        //     rl::unloadImage(screen);
        // }

        if frame.keyboard().is_key_down(Key::Zero) {
            position.z = 0.;
        }

        let mut canvas = frame.begin_drawing();

        canvas.clear_background(Color::SKYBLUE);

        let player_box =
            BoundingBox::new(position - player_height / 2., position + player_height / 2.);

        canvas.with_camera_3d(camera, |cam| {
            shader.with(|| {
                cam.draw_cube(
                    Vector3::new(-125., 0., 0.),
                    Vector3::new(300., 0.5, 20.),
                    Color::RAYWHITE,
                );

                cam.draw_cube(
                    position + Vector3::new(0., player_height / 2., 0.),
                    Vector3::value(player_height),
                    Color::RED,
                );

                for target in targets.iter_mut() {
                    let left_center = target.pos + TARGET_SIZE / 2.;
                    let right_center =
                        target.pos + TARGET_SIZE.div_components(Vector3::new(2., 2., -2.));

                    target.left.draw(
                        cam,
                        left_center,
                        player_box,
                        &mut score,
                        // good,
                        // bad,
                    );
                    target.right.draw(
                        cam,
                        right_center,
                        player_box,
                        &mut score,
                        // good,
                        // bad,
                    );

                    if !paused {
                        target.pos.x += 45. * cam.frame().get_time();
                    };

                    if target.pos.x > gap {
                        target.pos.x = -(start + 9. * gap);
                        target.left = Side::new();
                        target.right = Side::new();
                    }
                }
            });
        });

        draw_score(score, &mut canvas);
    }
}
