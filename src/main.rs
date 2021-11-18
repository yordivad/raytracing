extern crate sdl2;
mod vec;
mod ray;
mod camera;
mod object;

use std::process;
use sdl2::rect::{Rect, Point};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::ray::Ray;
use crate::vec::Vector;
use crate::camera::{Camera, Image};
use crate::camera::Viewport;
use crate::object::{Sphere};




fn main() {
    let ctx = sdl2::init().unwrap();
    let video = ctx.video().unwrap();
    let viewport = Viewport::aspect(4, 16.0 / 9.0);
    let image = Image::new(1200, 800);
    let camara = Camera::new(viewport, 1.0);


    let window = video.window("demo", image.width() as u32, image.height() as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .target_texture()
        .build()
        .unwrap();

    let mut events = ctx.event_pump().unwrap();

    let sphere = Sphere::new(Vector::new(0.0,0.0,-2.0), 0.5);

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(1)
                },

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {},

                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {},

                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {},

                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {},
                _ => {}
            }
        }

        canvas.clear();

        let mut y = 0;
        for i in (0..image.height()).rev() {
            for x in 0..image.width() {
                let u = image.u(x);
                let v = image.v(i);

                let ray = camara.ray(u, v);
                //let color = ray.color();

                let color  = sphere.color(ray);

                canvas.set_draw_color(Color::RGB(color.R(), color.G(), color.B()));
                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }
            y= y + 1;
        }


        canvas.present();
    };

    loop { main_loop(); }
}

