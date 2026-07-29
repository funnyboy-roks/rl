use rl::prelude::*;

fn main() {
    let mut window = Window::init(800, 600, "Texture");
    window.set_target_fps(60);

    let image = Image::load("examples/smile.png").unwrap();
    // let texture = Texture2D::from_image(&image);
    let mut texture = RenderTexture2D::new(64, 64);
    let img_tex = Texture2D::from_image(&image);

    texture.draw_with(|t| {
        let bounds = t.bounds();
        t.draw_texture_builder()
            .texture(&img_tex)
            .destination(bounds)
            .draw();
    });

    let mut rotation = 0.;
    while let Some(frame) = window.next_frame() {
        let mut canvas = frame.begin_drawing();
        canvas.clear_background(Color::GRAY);
        let center = canvas.size() / 2.;
        let texture_size = texture.size();

        canvas
            .draw_texture_builder()
            .texture(&texture.texture())
            .position(center, 4.0)
            .rotation(texture_size * 2., rotation)
            .draw();

        rotation += 1.;
    }
}
