use image::{Rgb, ImageBuffer, RgbImage};
use std::f64;

const MAX_ITER: usize = 200;
const COLOR_PALETTE: [(u8, u8, u8); 17] = [
   (  0,   0,   0),
   ( 66,  30,  15),
   ( 25,   7,  26),
   (  9,   1,  47),
   (  4,   4,  73),
   (  0,   7, 100),
   ( 12,  44, 138),
   ( 24,  82, 177),
   ( 57, 125, 209),
   (134, 181, 229),
   (211, 236, 248),
   (241, 233, 191),
   (248, 201,  95),
   (255, 170,   0),
   (204, 128,   0),
   (153,  87,   0),
   (106,  52,   3)
];

fn is_mandel(cx: f64, cy: f64) -> Rgb<u8> {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut iter: usize = 0;

    // Optimisation
    // 1er cas
    let case_a: f64 = (cx + 1.0) * (cx + 1.0) + cy*cy;

    // 2eme cas
    let p: f64 = ((cx - 0.25) * (cx - 0.25) + cy*cy).sqrt();
    let case_b: f64 = p - 2.0*p*p + 0.25;

    if case_a < 1.0/16.0 {
        Rgb([0, 0, 0])
    } else if cx < case_b {
        Rgb([0, 0, 0])
    } else {
        while (x * x + y * y) <= 4.0 && iter < MAX_ITER {
            
            let xtemp: f64 = x * x - y * y + cx;
            y = 2.0 * x * y + cy;
            x = xtemp;

            iter += 1;
        }

        let color_value: (u8, u8, u8) = COLOR_PALETTE[(iter % COLOR_PALETTE.len()) as usize];

        Rgb([color_value.0, color_value.1, color_value.2])
        /*
        if iter == MAX_ITER {
            Rgb([0, 0, 0])
        } else {
            let color_i: u8 = (iter % 16) as u8;

            let color: (u8, u8, u8) = COLOR_PALETTE[color_i as usize];
            
            Rgb([color.0, color.1, color.2])
        }
        */
    }
}


pub fn mandelbrot(re_start: f64, re_end: f64, im_start: f64, im_end: f64, width: u32, height: u32) -> RgbImage {
    let mut image: RgbImage = ImageBuffer::new(width, height);

    // Utilisation de par_iter_mut() pour paralléliser la boucle
    image
        .enumerate_pixels_mut()
        .into_iter()
        .for_each(|(x, y, pixel)| {
            let cx: f64 = re_start + (x as f64 / width as f64) * (re_end - re_start);
            let cy: f64 = im_start + (y as f64 / height as f64) * (im_end - im_start);

            *pixel = is_mandel(cx, cy);
        });
    
    image
}