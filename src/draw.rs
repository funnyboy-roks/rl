use bauer::Builder;

use crate::{Bounded, Color, Rectangle, Texture2D, math::Vector2};

// basic item that image, frame, and target can use
pub trait DrawTarget {
    fn clear_background(&mut self, color: Color);

    fn draw_pixel(&mut self, position: impl Into<Vector2>, color: Color);
    fn draw_line(
        &mut self,
        from: impl Into<Vector2>,
        to: impl Into<Vector2>,
        thick: f32,
        color: Color,
    );

    fn draw_circle(&mut self, center: impl Into<Vector2>, radius: f32, color: Color);
    fn draw_circle_lines(&mut self, center: impl Into<Vector2>, radius: f32, color: Color);

    fn draw_rectangle(&mut self, rect: Rectangle, color: Color);
    fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color);

    fn draw_triangle(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    );
    fn draw_triangle_lines(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        color: Color,
    );
    /// Draw a triangle fan defined by points (first vertex is the center)
    fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color);
    /// Draw a triangle strip defined by points
    fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color);

    fn draw_text(
        &mut self,
        text: impl AsRef<str>,
        pos: impl Into<Vector2>,
        font_size: u32,
        color: Color,
    );
    // TODO
    // fn draw_text_ex(
    //     &mut self,
    //     text: impl AsRef<str>,
    //     pos: impl Into<Vector2>,
    //     font_size: u32,
    //     color: Color,
    // );
}

// body of impl Type for &mut T.  Should be basically the same syntax as the trait definition
macro_rules! deref {
    ($(fn $name: ident(&mut self, $($f_name: ident: $f_ty: ty),*$(,)?);)*) => {
        $(
            fn $name(&mut self, $($f_name: $f_ty),*) {
                (*self).$name($($f_name),*);
            }
        )*
    };
}

impl<T> DrawTarget for &mut T
where
    T: DrawTarget,
{
    deref![
        fn clear_background(&mut self, color: Color);

        fn draw_pixel(&mut self, position: impl Into<Vector2>, color: Color);
        fn draw_line(
            &mut self,
            from: impl Into<Vector2>,
            to: impl Into<Vector2>,
            thick: f32,
            color: Color,
        );

        fn draw_circle(&mut self, center: impl Into<Vector2>, radius: f32, color: Color);
        fn draw_circle_lines(&mut self, center: impl Into<Vector2>, radius: f32, color: Color);

        fn draw_rectangle(&mut self, rect: Rectangle, color: Color);
        fn draw_rectangle_lines(&mut self, rect: Rectangle, line_thick: f32, color: Color);

        fn draw_triangle(
            &mut self,
            p1: impl Into<Vector2>,
            p2: impl Into<Vector2>,
            p3: impl Into<Vector2>,
            color: Color,
        );
        fn draw_triangle_lines(
            &mut self,
            p1: impl Into<Vector2>,
            p2: impl Into<Vector2>,
            p3: impl Into<Vector2>,
            color: Color,
        );
        fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color);
        fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color);

        fn draw_text(
            &mut self,
            text: impl AsRef<str>,
            pos: impl Into<Vector2>,
            font_size: u32,
            color: Color,
        );
    ];
}

pub enum GradientDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Builder)]
#[builder(kind = "type-state", builder_fn(visibility = pub(self)))]
pub struct DrawRectangle<T> {
    #[builder(associated)]
    target: T,
    rectangle: Rectangle,
    #[builder(adapter = |origin: impl Into<Vector2>, rotation: f32| (origin.into(), rotation))]
    rotation: (Vector2, f32),
    #[builder(into)]
    color: Color,
}

type EmptyDrawRectangleBuilder<T> = DrawRectangleBuilder<
    T,
    DrawRectangle_Rectangle_Set<false>,
    DrawRectangle_Rotation_Set<false>,
    DrawRectangle_Color_Set<false>,
>;
type FilledDrawRectangleBuilder<T> = DrawRectangleBuilder<
    T,
    DrawRectangle_Rectangle_Set<true>,
    DrawRectangle_Rotation_Set<true>,
    DrawRectangle_Color_Set<true>,
>;

impl<T: DrawTargetFull> FilledDrawRectangleBuilder<T> {
    pub fn draw(self) {
        let mut draw_rect = self.build();
        draw_rect.target.draw_rectangle_pro(
            draw_rect.rectangle,
            draw_rect.rotation.0,
            draw_rect.rotation.1,
            draw_rect.color,
        );
    }
}

#[derive(Clone, Copy)]
enum Destination {
    Rect(Rectangle),
    Scale(Vector2, f32),
}

#[derive(Builder)]
#[builder(builder_fn(visibility = pub(self)))]
pub struct DrawTexture<'target, T> {
    #[builder(associated)]
    target: &'target mut T,
    #[builder(adapter = |texture: &Texture2D| texture.clone())]
    texture: Texture2D,
    #[builder(into)]
    source: Option<Rectangle>,
    #[builder(adapter = |rect: impl Into<Rectangle>| Destination::Rect(rect.into()))]
    destination: Destination,
    #[builder(
        default = "(Vector2::ZERO, 0.)",
        adapter = |origin: impl Into<Vector2>, rotation: f32| (origin.into(), rotation)
    )]
    rotation: (Vector2, f32),
    #[builder(into, default = "Color::WHITE")]
    tint: Color,
}

impl<T> DrawTextureBuilder<'_, T> {
    pub fn position(self, position: impl Into<Vector2>, scale: f32) -> Self {
        let mut this = self;
        #[expect(
            deprecated,
            unused_unsafe, // just to be obvious
            reason = "This is the only way to do this.  Maybe that should change?"
        )]
        let inner = unsafe { &mut this.__unsafe_builder_content };
        inner.3 = Some(Destination::Scale(position.into(), scale));
        this
    }
}

impl<'target, T> DrawTextureBuilder<'target, T> {
    pub fn draw(self)
    where
        T: DrawTargetFull,
        Self: 'target,
    {
        let this = self.build().unwrap();

        let source = this.source.unwrap_or(this.texture.bounds());

        this.target.draw_texture_pro(
            &this.texture,
            source,
            match this.destination {
                Destination::Rect(r) => r,
                Destination::Scale(p, s) => Rectangle {
                    x: p.x,
                    y: p.y,
                    width: this.texture.width() as f32 * s,
                    height: this.texture.height() as f32 * s,
                },
            },
            this.rotation.0,
            this.rotation.1,
            this.tint,
        );
    }
}

// the full set of global draw functions from raylib
pub trait DrawTargetFull: DrawTarget + Sized {
    fn draw_line_strip(&mut self, points: &[Vector2], color: Color);
    fn draw_line_bezier(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        thick: f32,
        color: Color,
    );
    fn draw_line_dashed(
        &mut self,
        start: impl Into<Vector2>,
        end: impl Into<Vector2>,
        dash_size: u32,
        space_size: u32,
        color: Color,
    );

    fn draw_circle_gradient(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        inner: Color,
        outer: Color,
    );
    fn draw_circle_sector(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: f32, // TODO: Range?
        end_angle: f32,
        segments: u32,
        color: Color,
    );
    fn draw_circle_sector_lines(
        &mut self,
        center: impl Into<Vector2>,
        radius: f32,
        start_angle: f32, // TODO: Range?
        end_angle: f32,
        segments: u32,
        color: Color,
    );

    fn draw_ellipse(
        &mut self,
        center: impl Into<Vector2>,
        radius: impl Into<Vector2>,
        color: Color,
    );
    fn draw_ellipse_lines(
        &mut self,
        center: impl Into<Vector2>,
        radius: impl Into<Vector2>,
        color: Color,
    );

    #[expect(clippy::too_many_arguments, reason = "matching with raylib api")]
    fn draw_ring(
        &mut self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32, // TODO: Range?
        end_angle: f32,
        segments: u32,
        color: Color,
    );

    #[expect(clippy::too_many_arguments, reason = "matching with raylib api")]
    fn draw_ring_lines(
        &mut self,
        center: impl Into<Vector2>,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32, // TODO: Range?
        end_angle: f32,
        segments: u32,
        color: Color,
    );

    fn draw_rectangle_gradient(
        &mut self,
        rect: Rectangle,
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
    );
    fn draw_rectangle_pro(
        &mut self,
        rect: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        color: Color,
    );
    fn draw_rectangle_builder(&mut self) -> EmptyDrawRectangleBuilder<&mut Self> {
        DrawRectangle::builder(self)
    }
    fn draw_rectangle_rounded(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        color: Color,
    );
    fn draw_rectangle_rounded_lines(
        &mut self,
        rect: Rectangle,
        roundness: f32,
        segments: u32,
        thick: f32,
        color: Color,
    );

    fn draw_poly(
        &mut self,
        center: impl Into<Vector2>,
        sides: u32,
        radius: f32,
        rotation: f32,
        color: Color,
    );
    fn draw_poly_lines(
        &mut self,
        center: impl Into<Vector2>,
        sides: u32,
        radius: f32,
        rotation: f32,
        thick: f32,
        color: Color,
    );

    fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: Color);
    fn draw_spline_basis(&mut self, points: &[Vector2], thick: f32, color: Color);
    fn draw_spline_catmull_rom(&mut self, points: &[Vector2], thick: f32, color: Color);
    fn draw_spline_bezier_quadratic(&mut self, points: &[Vector2], thick: f32, color: Color);
    fn draw_spline_bezier_cubic(&mut self, points: &[Vector2], thick: f32, color: Color);
    fn draw_spline_segment_linear(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        thick: f32,
        color: Color,
    );
    fn draw_spline_segment_basis(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        p4: impl Into<Vector2>,
        thick: f32,
        color: Color,
    );
    fn draw_spline_segment_catmull_rom(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        p4: impl Into<Vector2>,
        thick: f32,
        color: Color,
    );
    fn draw_spline_segment_bezier_quadratic(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        thick: f32,
        color: Color,
    );
    fn draw_spline_segment_bezier_cubic(
        &mut self,
        p1: impl Into<Vector2>,
        p2: impl Into<Vector2>,
        p3: impl Into<Vector2>,
        p4: impl Into<Vector2>,
        thick: f32,
        color: Color,
    );

    fn draw_texture(
        &mut self,
        texture: &Texture2D,
        position: impl Into<Vector2>,
        rotation: f32,
        scale: f32,
        tint: Color,
    );

    fn draw_texture_pro(
        &mut self,
        texture: &Texture2D,
        src: Rectangle,
        dst: Rectangle,
        origin: impl Into<Vector2>,
        rotation: f32,
        tint: Color,
    );
    fn draw_texture_builder<'dt>(&'dt mut self) -> DrawTextureBuilder<'dt, Self> {
        DrawTexture::builder(self)
    }
}

impl<T> DrawTargetFull for &mut T
where
    T: DrawTargetFull,
{
    deref![
        fn draw_line_strip(&mut self, points: &[Vector2], color: Color);
        fn draw_line_bezier(
            &mut self,
            start: impl Into<Vector2>,
            end: impl Into<Vector2>,
            thick: f32,
            color: Color,
        );
        fn draw_line_dashed(
            &mut self,
            start: impl Into<Vector2>,
            end: impl Into<Vector2>,
            dash_size: u32,
            space_size: u32,
            color: Color,
        );

        fn draw_circle_gradient(
            &mut self,
            center: impl Into<Vector2>,
            radius: f32,
            inner: Color,
            outer: Color,
        );
        fn draw_circle_sector(
            &mut self,
            center: impl Into<Vector2>,
            radius: f32,
            start_angle: f32, // TODO: Range?
            end_angle: f32,
            segments: u32,
            color: Color,
        );
        fn draw_circle_sector_lines(
            &mut self,
            center: impl Into<Vector2>,
            radius: f32,
            start_angle: f32, // TODO: Range?
            end_angle: f32,
            segments: u32,
            color: Color,
        );

        fn draw_ellipse(
            &mut self,
            center: impl Into<Vector2>,
            radius: impl Into<Vector2>,
            color: Color,
        );
        fn draw_ellipse_lines(
            &mut self,
            center: impl Into<Vector2>,
            radius: impl Into<Vector2>,
            color: Color,
        );

        fn draw_ring(
            &mut self,
            center: impl Into<Vector2>,
            inner_radius: f32,
            outer_radius: f32,
            start_angle: f32, // TODO: Range?
            end_angle: f32,
            segments: u32,
            color: Color,
        );

        fn draw_ring_lines(
            &mut self,
            center: impl Into<Vector2>,
            inner_radius: f32,
            outer_radius: f32,
            start_angle: f32, // TODO: Range?
            end_angle: f32,
            segments: u32,
            color: Color,
        );

        fn draw_rectangle_gradient(
            &mut self,
            rect: Rectangle,
            top_left: Color,
            top_right: Color,
            bottom_left: Color,
            bottom_right: Color,
        );
        fn draw_rectangle_pro(
            &mut self,
            rect: Rectangle,
            origin: impl Into<Vector2>,
            rotation: f32,
            color: Color,
        );
        fn draw_rectangle_rounded(
            &mut self,
            rect: Rectangle,
            roundess: f32,
            segments: u32,
            color: Color,
        );
        fn draw_rectangle_rounded_lines(
            &mut self,
            rect: Rectangle,
            roundess: f32,
            segments: u32,
            thick: f32,
            color: Color,
        );

        fn draw_poly(
            &mut self,
            center: impl Into<Vector2>,
            sides: u32,
            radius: f32,
            rotation: f32,
            color: Color,
        );
        fn draw_poly_lines(
            &mut self,
            center: impl Into<Vector2>,
            sides: u32,
            radius: f32,
            rotation: f32,
            thick: f32,
            color: Color,
        );

        fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: Color);
        fn draw_spline_basis(&mut self, points: &[Vector2], thick: f32, color: Color);
        fn draw_spline_catmull_rom(&mut self, points: &[Vector2], thick: f32, color: Color);
        fn draw_spline_bezier_quadratic(&mut self, points: &[Vector2], thick: f32, color: Color);
        fn draw_spline_bezier_cubic(&mut self, points: &[Vector2], thick: f32, color: Color);
        fn draw_spline_segment_linear(
            &mut self,
            p1: impl Into<Vector2>,
            p2: impl Into<Vector2>,
            thick: f32,
            color: Color,
        );
        fn draw_spline_segment_basis(
            &mut self,
            p1: impl Into<Vector2>,
            p2: impl Into<Vector2>,
            p3: impl Into<Vector2>,
            p4: impl Into<Vector2>,
            thick: f32,
            color: Color,
        );
        fn draw_spline_segment_catmull_rom(
            &mut self,
            p1: impl Into<Vector2>,
            p2: impl Into<Vector2>,
            p3: impl Into<Vector2>,
            p4: impl Into<Vector2>,
            thick: f32,
            color: Color,
        );
        fn draw_spline_segment_bezier_quadratic(
            &mut self,
            p1: impl Into<Vector2>,
            p2: impl Into<Vector2>,
            p3: impl Into<Vector2>,
            thick: f32,
            color: Color,
        );
        fn draw_spline_segment_bezier_cubic(
            &mut self,
            p1: impl Into<Vector2>,
            p2: impl Into<Vector2>,
            p3: impl Into<Vector2>,
            p4: impl Into<Vector2>,
            thick: f32,
            color: Color,
        );

        fn draw_texture(
            &mut self,
            texture: &Texture2D,
            position: impl Into<Vector2>,
            rotation: f32,
            scale: f32,
            tint: Color,
        );

        fn draw_texture_pro(
            &mut self,
            texture: &Texture2D,
            src: Rectangle,
            dst: Rectangle,
            origin: impl Into<Vector2>,
            rotation: f32,
            tint: Color,
        );
    ];
}
