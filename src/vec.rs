use std::ops;


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn length(self) -> f32 {
        (&self.x * self.x + &self.y * self.y + &self.z * self.z).sqrt()
    }

    pub fn unit(self) -> Vector {
        self / self.length()
    }

    pub fn ones() -> Vector {
        Vector::new(1.0, 1.0, 1.0)
    }

    pub fn zeros() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn x(self) -> f32 { self.x }

    pub fn y(self) -> f32 { self.y }

    pub fn z(self) -> f32 { self.z }

    pub fn dot(self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            (self.x * other.z - self.z * other.x) * -1.0,
            self.x * other.y - self.y * other.x
        )
    }
}


impl ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Vector  {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs * -1.0
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, a: f32) -> Self::Output {
        Vector::new( self.x * &a, self.y * &a,  self.z * a )
    }
}

impl ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / &rhs, self.y / &rhs, self.z / rhs)
    }
}



#[cfg(test)]
mod test {
    use crate::vec::Vector;

    #[test]
    fn test_add() {
        let a = Vector::new(2.0, 4.0, 6.0);
        let b = Vector::new(3.0, 5.0, 7.0);
        let c = Vector::new(5.0, 9.0, 13.0);
        assert_eq!(c, a + b)
    }

    #[test]
    fn test_mul() {
        let a = Vector::new(2.0, 4.0, 6.0);
        let c = Vector::new(4.0, 8.0, 12.0);
        assert_eq!(c, a * 2.0)
    }

    #[test]
    fn test_div() {
        let a = Vector::new(2.0, 4.0, 6.0);
        let c = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(c, a / 2.0)
    }

    #[test]
    fn test_length() {
        let a = Vector::new(0.0, 4.0, 3.0);
        assert_eq!(5.0, a.length())
    }

    #[test]
    fn test_unit() {
        let a = Vector::new(1.0, 4.0, 3.0);
        let c = Vector::new(0.19611613, 0.78446454, 0.5883484);
        assert_eq!(c, a.unit())
    }

    #[test]
    fn test_dot() {
        let a = Vector::new(1.0, 1.0, 1.0);
        let b = Vector::new(2.0, 2.0, 2.0);
        assert_eq!(6.0, a.dot(b));
    }

    #[test]
    fn test_cross() {
        let a = Vector::new(3.0, -3.0, 1.0);
        let b = Vector::new(4.0, 9.0, 2.0);
        let c = Vector::new(-15.0, -2.0, 39.0);

        assert_eq!(c, a.cross(b))

    }


}




