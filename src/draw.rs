use std::rc::Rc;

use crate::{Bounded, Color, Rectangle, Texture2D, Vector2};

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

#[derive(Clone, Copy)]
pub struct DrawRectangleBuilder<T> {
    target: T,
    rectangle: Option<Rectangle>,
    rotation: (Vector2, f32),
    color: Option<Color>,
}

impl<T> DrawRectangleBuilder<T> {
    fn new(target: T) -> Self {
        Self {
            target,
            rectangle: None,
            rotation: (Vector2::zero(), 0.),
            color: None,
        }
    }

    pub fn rectangle(&mut self, rectangle: Rectangle) -> &mut Self {
        self.rectangle = Some(rectangle);
        self
    }

    pub fn rotation(&mut self, origin: impl Into<Vector2>, rotation: f32) -> &mut Self {
        self.rotation = (origin.into(), rotation);
        self
    }

    pub fn color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.color = Some(color.into());
        self
    }

    /// # Panics
    ///
    /// If `rectangle` or `color` have not been set
    pub fn draw(&mut self)
    where
        T: DrawTargetFull,
    {
        let rectangle = self
            .rectangle
            .expect("rectangle must be set on DrawRectangleBuilder");
        let color = self
            .color
            .expect("color must be set on DrawRectangleBuilder");

        self.target
            .draw_rectangle_pro(rectangle, self.rotation.0, self.rotation.1, color);
    }
}

macro_rules! expect {
    ($ty: ty => $self: ident [$($e:ident),* ]) => {
        $(
            let $e = $self.$e.expect(concat!(stringify!($e), " must be set on ", stringify!($ty)));
        )*
    }
}

#[derive(Clone, Copy)]
enum Destination {
    Rect(Rectangle),
    Scale(Vector2, f32),
}

pub struct DrawTextureBuilder<'target, T> {
    target: &'target mut T,
    texture: Option<Texture2D>,
    source: Option<Rectangle>,
    destination: Option<Destination>,
    rotation: (Vector2, f32),
    tint: Color,
}

impl<'target, T> DrawTextureBuilder<'target, T> {
    fn new(target: &'target mut T) -> Self {
        Self {
            target,
            texture: None,
            source: None,
            destination: None,
            rotation: Default::default(),
            tint: Color::WHITE,
        }
    }

    pub fn texture(&mut self, texture: &Texture2D) -> &mut Self {
        self.texture = Some(texture.clone());
        self
    }

    pub fn source(&mut self, rect: impl Into<Rectangle>) -> &mut Self {
        self.source = Some(rect.into());
        self
    }

    /// NOTE: Overwrites the value specified by [`Self::position`]
    pub fn destination(&mut self, rect: impl Into<Rectangle>) -> &mut Self {
        self.destination = Some(Destination::Rect(rect.into()));
        self
    }

    /// NOTE: Overwrites the value specified by [`Self::destination`]
    pub fn position(&mut self, position: impl Into<Vector2>, scale: f32) -> &mut Self {
        self.destination = Some(Destination::Scale(position.into(), scale));
        self
    }

    pub fn rotation(&mut self, origin: impl Into<Vector2>, rotation: f32) -> &mut Self {
        self.rotation = (origin.into(), rotation);
        self
    }

    pub fn tint(&mut self, tint: impl Into<Color>) -> &mut Self {
        self.tint = tint.into();
        self
    }

    pub fn draw(&mut self)
    where
        T: DrawTargetFull,
        Self: 'target,
    {
        expect!(DrawTextureBuilder => self [destination]);
        let texture = self.texture.as_ref().unwrap().clone();

        let source = self.source.unwrap_or(texture.bounds());

        self.target.draw_texture_pro(
            &texture,
            source,
            match destination {
                Destination::Rect(r) => r,
                Destination::Scale(p, s) => Rectangle {
                    x: p.x,
                    y: p.y,
                    width: texture.width() as f32 * s,
                    height: texture.height() as f32 * s,
                },
            },
            self.rotation.0,
            self.rotation.1,
            self.tint,
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
    fn draw_rectangle_builder(&mut self) -> DrawRectangleBuilder<&mut Self> {
        DrawRectangleBuilder::new(self)
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
        DrawTextureBuilder::new(self)
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
