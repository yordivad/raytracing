
use crate::vec::Vector;
use sdl2::pixels::Color;



#[derive(Debug, Copy, Clone)]
pub struct RGBColor {
    color: Vector
}

impl RGBColor {
    pub fn new(color: Vector) -> Self {
        RGBColor { color }
    }

    pub fn R(self) -> u8 {
        (self.color.x() * 255.99) as u8
    }

    pub fn G(self) -> u8 {
        (self.color.y() * 255.99) as u8
    }

    pub fn B(self) -> u8 {
        (self.color.z() * 255.99) as u8
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    a: Vector,
    b: Vector
}

impl Ray {
    pub fn new(a: Vector, b: Vector) -> Self {
        Ray{ a, b}
    }

    pub fn origin(self) -> Vector {
        self.a
    }

    pub fn  direction(self) -> Vector {
        self.b
    }

    pub fn point(self, t: f32) -> Vector {
        self.a + self.b * t
    }

    pub fn color(self) -> RGBColor {
        let unit_direction = self.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return  RGBColor::new(  Vector::ones() * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t);
    }

    pub fn at(self, t: f32) -> Vector {
        self.a + self.b * t
    }

    pub fn position(self, t:f32) -> Vector {
        return self.origin() + self.direction() * t;
    }

}

#[cfg(test)]
mod test {
    use crate::ray::Ray;
    use crate::vec::Vector;

    #[test]
    fn test_position() {
        let ray = Ray::new(Vector::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(Vector::new(2.0, 3.0, 4.0), ray.position(0.0));
        assert_eq!(Vector::new(3.0, 3.0, 4.0), ray.position(1.0));
    }
}


