use crate::vec::Vector;
use crate::ray::{Ray, RGBColor};


#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center : Vector,
    radius : f32
}

impl Sphere {
    pub fn new(center: Vector, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn hit(self, ray: Ray) -> f32 {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let dis = b * b - 4.0 * a * c;
        if dis < 0.0 {
            return -1.0;
        }
        return  (-b - dis.sqrt()) / (2.0 * a);
    }

    pub fn color(self, r: Ray) -> RGBColor {

        let t = self.hit(r);

        if  t > 0.0 {
            let n = (r.at(t) - Vector::new(0.0, 0.0, -1.0)).unit();
            return RGBColor::new(Vector::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0 ) * 0.5);
        }

        return  r.color();
    }
}
