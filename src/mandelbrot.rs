use image::{ImageBuffer, Rgb, RgbImage};
use std::thread;
use std::sync::{Arc, Mutex};

const MAX_ITER: usize = 500;

const COLOR_PALETTE: [(u8, u8, u8); 16] = [
    (66, 30, 15),
    (25, 7, 26),
    (9, 1, 47),
    (4, 4, 73),
    (0, 7, 100),
    (12, 44, 138),
    (24, 82, 177),
    (57, 125, 209),
    (134, 181, 229),
    (211, 236, 248),
    (241, 233, 191),
    (248, 201, 95),
    (255, 170, 0),
    (204, 128, 0),
    (153, 87, 0),
    (106, 52, 3),
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

        if iter >= MAX_ITER {
            Rgb([0, 0, 0])
        } else {
            let log_zn: f64 = (x * x + y * y).ln();
            let nu: f64 = (log_zn / 2.0_f64.ln()).ln() / (2.0_f64.ln());

            // Effectuer l'interpolation pour obtenir des valeurs réelles pour les coordonnées non entières.
            let smooth_iter: f64 = (iter as f64) + 1.0 - nu;

            // Obtenir les parties entières et fractionnaires de la valeur lissée.
            let smooth_iter_floor: usize = smooth_iter.floor() as usize;
            let smooth_iter_frac: f64 = smooth_iter - (smooth_iter_floor as f64);

            // Interpolation linéaire entre les couleurs de la palette.
            let color1: (u8, u8, u8) = COLOR_PALETTE[smooth_iter_floor % COLOR_PALETTE.len()];
            let color2: (u8, u8, u8) = COLOR_PALETTE[(smooth_iter_floor + 1) % COLOR_PALETTE.len()];

            let r: u8 = ((color1.0 as f64) * (1.0 - smooth_iter_frac)
                + (color2.0 as f64) * smooth_iter_frac) as u8;
            let g: u8 = ((color1.1 as f64) * (1.0 - smooth_iter_frac)
                + (color2.1 as f64) * smooth_iter_frac) as u8;
            let b: u8 = ((color1.2 as f64) * (1.0 - smooth_iter_frac)
                + (color2.2 as f64) * smooth_iter_frac) as u8;

            Rgb([r, g, b])
        }
    }
}

pub fn mandelbrot(
    re_start: f64,
    re_end: f64,
    im_start: f64,
    im_end: f64,
    width: u32,
    height: u32,
) -> RgbImage {
    //let mut image: RgbImage = ImageBuffer::new(width, height);
    let image: Arc<Mutex<RgbImage>> = Arc::new(Mutex::new(ImageBuffer::new(width, height)));

    let num_threads = 1000;
    let rows_per_thread = height / num_threads as u32;

    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let start_y = thread_id * rows_per_thread;
        let end_y = if thread_id == num_threads - 1 {
            height
        } else {
            start_y + rows_per_thread
        };

        // let mut local_image = image.clone();
        let shared_image = Arc::clone(&image);
        thread::spawn(move || {
            let mut local_image = shared_image.lock().unwrap();
            for y in start_y..end_y {
                for x in 0..width {
                    let cx: f64 = re_start + (x as f64 / width as f64) * (re_end - re_start);
                    let cy: f64 = im_start + (y as f64 / height as f64) * (im_end - im_start);
                    let color = is_mandel(cx, cy);
                    local_image.put_pixel(x, y, color);
                }
            }
        })
    }).collect();

    for handle in handles {
        handle.join().expect("Thread panicked!");
    }

    /*
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let cx: f64 = re_start + (x as f64 / width as f64) * (re_end - re_start);
        let cy: f64 = im_start + (y as f64 / height as f64) * (im_end - im_start);

        *pixel = is_mandel(cx, cy);
    }
    */
    Arc::try_unwrap(image).unwrap().into_inner().unwrap()
}
