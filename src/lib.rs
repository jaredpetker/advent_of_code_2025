#[macro_export]
macro_rules! include_file {
    ( $file:literal ) => {{
        println!("Loading file: {}", $file);
        include_str!($file)
    }};
}

#[derive(PartialEq, Copy, Clone, Debug, PartialOrd)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl From<Vec2i> for Vec2f {
    fn from(value: Vec2i) -> Self {
        Vec2f {
            x: value.x as f64,
            y: value.y as f64,
        }
    }
}

impl Vec2f {
    pub fn cross(&self, other: &Vec2f) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn dot(&self, other: &Vec2f) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64,
}

impl Vec2i {
    pub fn new(x: i64, y: i64) -> Self {
        Vec2i { x, y }
    }

    pub fn up() -> Self {
        Vec2i { x: 0, y: -1 }
    }

    pub fn down() -> Self {
        Vec2i { x: 0, y: 1 }
    }

    pub fn left() -> Self {
        Vec2i { x: -1, y: 0 }
    }

    pub fn right() -> Self {
        Vec2i { x: 1, y: 0 }
    }

    pub fn rotate_left(&self) -> Self {
        Vec2i {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn rotate_right(&self) -> Self {
        Vec2i {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn add(&self, other: &Vec2i) -> Vec2i {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn prod_vec(&self, other: &Vec2i) -> Vec2i {
        Vec2i {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl From<(&str, &str)> for Vec2i {
    fn from((x, y): (&str, &str)) -> Self {
        Self {
            x: x.parse::<i64>().expect("expected a number"),
            y: y.parse::<i64>().expect("expected a number"),
        }
    }
}

impl std::ops::Add<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn add(self, other: Vec2i) -> Vec2i {
        Vec2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign<Vec2i> for Vec2i {
    fn add_assign(&mut self, other: Vec2i) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn sub(self, other: Vec2i) -> Vec2i {
        Vec2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign<Vec2i> for Vec2i {
    fn sub_assign(&mut self, other: Vec2i) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul<i64> for Vec2i {
    type Output = Vec2i;

    fn mul(self, rhs: i64) -> Vec2i {
        Vec2i {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<i64> for Vec2i {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Div<i64> for Vec2i {
    type Output = Vec2i;

    fn div(self, rhs: i64) -> Vec2i {
        Vec2i {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::DivAssign<i64> for Vec2i {
    fn div_assign(&mut self, rhs: i64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::Neg for Vec2i {
    type Output = Vec2i;

    fn neg(self) -> Vec2i {
        Vec2i {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub struct Vec3i {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3i {
    pub fn distance(&self, other: &Self) -> i64 {
        ((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)).isqrt()
    }
}

impl From<&str> for Vec3i {
    fn from(s: &str) -> Self {
        let mut split = s.splitn(3, ",");

        Vec3i {
            x: split
                .next()
                .expect("Expected token")
                .parse::<i64>()
                .expect("Expected number"),
            y: split
                .next()
                .expect("Expected token")
                .parse::<i64>()
                .expect("Expected number"),
            z: split
                .next()
                .expect("Expected token")
                .parse::<i64>()
                .expect("Expected number"),
        }
    }
}
