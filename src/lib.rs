mod utils;

use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};

use std::f64;
use wasm_bindgen_test::console_log;
use web_sys::{CanvasRenderingContext2d, ImageData};
#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    mouse_x: u32,
    mouse_y: u32,
    scale_factor: f64
) {
    console_log!("Starting drawing!");
    let mb_set = get_pixels(width, height, mouse_x, mouse_y, scale_factor);
    let data= ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mb_set), width, height).unwrap();
    ctx.put_image_data(&data, 0.0, 0.0).expect("TODO: panic message");
}
#[derive(Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn square(&self) -> Complex {
        Complex {
            re: self.re * self.re - self.im * self.im,
            im: 2. * (self.re * self.im)
        }
    }

    fn norm(&self) -> f64 {
        self.im*self.im+self.re*self.re
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

fn julia_set(coordinate: Complex, julia_coordinate: Complex, max_iterations: u32) -> u32 {
    let mut i: u32 = 0;
    let mut z = Complex {re: julia_coordinate.re, im: julia_coordinate.im};
    while i < max_iterations && z.norm() < 4. {
        z = z.square() + coordinate;
        i += 1;
    }
    i
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn black() -> Color {
        Color {r: 0, g: 0, b: 0}
    }

    fn from_iterations(max_iterations: u32, iterations: u32) -> Color {
        let color_steps = [
            Color{r: 0, g: 0, b: 255},  // Blue
            Color{r: 255, g: 0, b: 255},// Pink
            Color{r: 255, g: 0, b: 0},  // Red
            Color{r: 255, g: 255, b: 0},// Yellow
            Color{r: 0, g: 255, b: 0},  // Green
            Color{r: 255, g: 255, b: 255},// White
            Color{r: 0, g: 0, b: 0}     // Black
        ];
        let segments: u32 = (color_steps.len() -1) as u32;
        let steps_per_segment: u32 = max_iterations/segments;
        let segment = (iterations/steps_per_segment).min(segments-1);
        let t: f32 = (iterations % steps_per_segment) as f32 / steps_per_segment as f32;
        let start = &color_steps[segment as usize];
        let end = &color_steps[segment as usize+1];

        let r: u8 = (start.r as f32 + t * (end.r as f32 - start.r as f32)) as u8;
        let g: u8 = (start.g as f32 + t * (end.g as f32 - start.g as f32)) as u8;
        let b: u8 = (start.b as f32 + t * (end.b as f32 - start.b as f32)) as u8;
        Color {r, g, b}
    }
}

#[wasm_bindgen]
pub struct Screen {
    width: u32,
    height: u32,
    pixels: Vec<u8>
}

fn get_pixels(
    width: u32,
    height: u32,
    mouse_x: u32,
    mouse_y: u32,
    scale_factor: f64
) -> Vec<u8> {
    let x_start: f64 = -2.;
    let x_end: f64 = 2.5;
    let y_start: f64 = -1.;
    let y_end: f64 = 1.;
    let max_iterations: u32 = 200;
    let mut pixels: Vec<u8> = Vec::new();

    // Calculate the center position based on mouse coordinates
    let mouse_x_pos = x_start + mouse_x as f64 * (x_end - x_start) / width as f64;
    let mouse_y_pos = y_start + mouse_y as f64 * (y_end - y_start) / height as f64;

    // Adjust the width and height of the viewing window based on the scale factor
    let new_width = (x_end - x_start) / scale_factor;
    let new_height = (y_end - y_start) / scale_factor;

    // Calculate the new start and end positions based on the zoom level and mouse position
    let x_center = mouse_x_pos;
    let y_center = mouse_y_pos;

    let new_x_start = x_center - new_width / 2.0;
    let new_x_end = x_center + new_width / 2.0;
    let new_y_start = y_center - new_height / 2.0;
    let new_y_end = y_center + new_height / 2.0;

    for i in 0..height {
        for j in 0..width {
            let x: f64 = new_x_start + j as f64 * (new_x_end - new_x_start) / width as f64;
            let y: f64 = new_y_start + i as f64 * (new_y_end - new_y_start) / height as f64;
            let coordinate = Complex { re: x, im: y };
            let iterations = julia_set(coordinate, Complex { re: 0., im: 0. }, max_iterations);
            let pixel_color = if iterations == max_iterations {
                Color::black()
            } else {
                Color::from_iterations(max_iterations, iterations)
            };
            pixels.push(pixel_color.r);
            pixels.push(pixel_color.g);
            pixels.push(pixel_color.b);
            pixels.push(255);
        }
    }
    pixels
}


#[wasm_bindgen]
impl Screen {
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}