use crate::{
    color::Color,
    math::{Angle, Ray, Vector2, Vector3},
};

pub trait DrawTarget3D {
    fn draw_line(&mut self, start: impl Into<Vector3>, end: impl Into<Vector3>, color: Color);
    fn draw_point(&mut self, point: impl Into<Vector3>, color: Color);
    fn draw_circle(
        &mut self,
        center: impl Into<Vector3>,
        radius: f32,
        rotation_axis: impl Into<Vector3>,
        rotation_angle: Angle,
        color: Color,
    );
    fn draw_triangle(
        &mut self,
        p1: impl Into<Vector3>,
        p2: impl Into<Vector3>,
        p3: impl Into<Vector3>,
        color: Color,
    );
    fn draw_triangle_strip(&mut self, points: &[Vector3], color: Color);
    fn draw_cube(&mut self, position: impl Into<Vector3>, size: impl Into<Vector3>, color: Color);
    fn draw_cube_wires(
        &mut self,
        position: impl Into<Vector3>,
        size: impl Into<Vector3>,
        color: Color,
    );
    fn draw_sphere(&mut self, center: impl Into<Vector3>, radius: f32, color: Color) {
        // from rmodels.c:435
        self.draw_sphere_ex(center, radius, 16, 16, color);
    }
    fn draw_sphere_ex(
        &mut self,
        center: impl Into<Vector3>,
        radius: f32,
        rings: u32,
        slices: u32,
        color: Color,
    );
    fn draw_sphere_wires(&mut self, center: impl Into<Vector3>, radius: f32, color: Color) {
        self.draw_sphere_wires_ex(center, radius, 16, 16, color);
    }
    // Maps to DrawSphereWires
    fn draw_sphere_wires_ex(
        &mut self,
        center: impl Into<Vector3>,
        radius: f32,
        rings: u32,
        slices: u32,
        color: Color,
    );
    fn draw_cylinder(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius_start: f32,
        radius_end: f32,
        sides: u32,
        color: Color,
    );
    fn draw_cylinder_wires(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius_start: f32,
        radius_end: f32,
        sides: u32,
        color: Color,
    );
    fn draw_capsule(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius: f32,
        slices: u32,
        rings: u32,
        color: Color,
    );
    fn draw_capsule_wires(
        &mut self,
        start: impl Into<Vector3>,
        end: impl Into<Vector3>,
        radius: f32,
        slices: u32,
        rings: u32,
        color: Color,
    );
    fn draw_plane(&mut self, center: impl Into<Vector3>, size: impl Into<Vector2>, color: Color);
    fn draw_ray(&mut self, ray: Ray, color: Color);
    fn draw_grid(&mut self, slices: u32, spacing: f32);
}
