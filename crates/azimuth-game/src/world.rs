#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldPosition {
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn translated(self, offset: WorldVector) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WorldVector {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn scaled(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn added(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
