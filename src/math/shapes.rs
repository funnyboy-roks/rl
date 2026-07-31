use crate::math::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    min: Vector3,
    max: Vector3,
}

impl BoundingBox {
    pub const fn new(a: Vector3, b: Vector3) -> Self {
        Self {
            min: a.min(b),
            max: a.max(b),
        }
    }

    pub fn collides_box(self, other: Self) -> bool {
        if (self.max.x >= other.min.x) && (self.min.x <= other.max.x) {
            if (self.max.y < other.min.y)
                || (self.min.y > other.max.y)
                || (self.max.z < other.min.z)
                || (self.min.z > other.max.z)
            {
                return false;
            };
            return true;
        }
        false
    }
}
