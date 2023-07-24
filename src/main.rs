mod mandelbrot;

use mandelbrot::mandelbrot;
use minifb::{Window, WindowOptions, Key, KeyRepeat};
use image::Rgb;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;

fn zoom(x: f32, y: f32, x_min: &mut f64, x_max: &mut f64, y_min: &mut f64, y_max: &mut f64, zoom_factor: f64) {
    let width: f64 = *x_max - *x_min;
    let height: f64 = *y_max - *y_min;
    let x_percent: f64 = x as f64 / WIDTH as f64;
    let y_percent: f64 = y as f64 / HEIGHT as f64;

    *x_min += width * x_percent * zoom_factor;
    *x_max -= width * (1.0 - x_percent) * zoom_factor;
    *y_min += height * y_percent * zoom_factor;
    *y_max -= height * (1.0 - y_percent) * zoom_factor;
}

fn main() {
    let mut x_min: f64 = -2.0;
    let mut x_max: f64 = 1.0;
    let mut y_min: f64 = -1.0;
    let mut y_max: f64 = 1.0;

    let image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = mandelbrot(x_min, x_max, y_min, y_max, WIDTH as u32, HEIGHT as u32);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel_value: &Rgb<u8> = image.get_pixel(x as u32, y as u32);
            let color: u32 = ((pixel_value[0] as u32) << 16) | ((pixel_value[1] as u32) << 8) | pixel_value[2] as u32;

            buffer[y * WIDTH + x] = color;
        }
    }

    let mut window = Window::new(
        "Test",
        WIDTH, 
        HEIGHT, 
        WindowOptions::default()
    )
    .unwrap_or_else(|e|{
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) { 
                zoom(x, y, &mut x_min, &mut x_max, &mut y_min, &mut y_max, 0.5);
                buffer = update_buffer(x_min, x_max, y_min, y_max, WIDTH, HEIGHT)
            }
        };
        if window.is_key_pressed(Key::Backspace, KeyRepeat::No) {
            if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) { 
                zoom(x, y, &mut x_min, &mut x_max, &mut y_min, &mut y_max, -0.5);
                buffer = update_buffer(x_min, x_max, y_min, y_max, WIDTH, HEIGHT)
            }
        };

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn update_buffer(x_min: f64, x_max: f64, y_min: f64, y_max: f64, width: usize, height: usize) -> Vec<u32> {
    let image: image::ImageBuffer<Rgb<u8>, Vec<u8>> = mandelbrot(x_min, x_max, y_min, y_max, width as u32, height as u32);

    let mut buffer: Vec<u32> = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let pixel_value: &Rgb<u8> = image.get_pixel(x as u32, y as u32);
            let color: u32 = ((pixel_value[0] as u32) << 16) | ((pixel_value[1] as u32) << 8) | pixel_value[2] as u32;

            buffer[y * width + x] = color;
        }
    }

    buffer
}