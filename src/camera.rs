use crate::vec::Vector;
use sdl2::rect::Point;
use crate::ray::Ray;


#[derive(Debug, Copy, Clone)]
pub struct Image {
    width: i32,
    height: i32,
}

impl Image {

    pub fn new(width: i32, height: i32 ) -> Self {
        Image { width, height }
    }

    pub fn height(self) -> i32 {
        self.height
    }

    pub fn width(self) -> i32  {
        self.width
    }
    pub fn u(self, x: i32) -> f32 {
        x as f32 / self.width as f32
    }

    pub fn v(self, y: i32) -> f32 {
        y as f32 / self.height as f32
    }
}





#[derive(Debug, Copy, Clone)]
pub struct Viewport {
    width: i32,
    height: i32,
}

impl Viewport {

    pub fn aspect(width: i32, aspect_ratio: f32) -> Viewport {
        Viewport { width: width, height: ((width as f32) / aspect_ratio) as i32 }
    }
    pub fn normal(height: i32, width: i32) -> Viewport {
        Viewport {height, width }
    }

    pub fn height(self) -> i32 {
        self.height
    }

    pub fn width(self) -> i32  {
        self.width
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Vector,
    horizontal: Vector,
    vertical: Vector,
    corner_lower_left: Vector,
}


impl Camera {

    pub fn new(viewport: Viewport, focal_length: f32) -> Camera {
        let o = Vector::new(0.0, 0.0, 0.0);
        let h = Vector::new(viewport.width as f32, 0.0,0.0);
        let v = Vector::new(0.0, viewport.height as f32,0.0);
        let l =    o - (h / 2.0) - (v / 2.0) - Vector::new(0.0,0.0, focal_length);

        Camera {
            origin: o,
            horizontal: h,
            vertical: v,
            corner_lower_left: l
        }


    }

    pub fn ray(self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.corner_lower_left + self.horizontal * u + self.vertical * v - self.origin)
    }



}