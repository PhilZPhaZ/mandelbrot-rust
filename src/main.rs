use image::{Rgb, ImageBuffer, RgbImage};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
const MAX_ITER: usize = 200;

const RE_START: f64 = -2.0;
const RE_END: f64 = 1.0;
const IM_START: f64 = 1.0;
const IM_END: f64 = -1.0;

const COLOR_PALETTE: [(u8, u8, u8); 16] = [
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

    while x * x + y * y <= 4.0 && iter < MAX_ITER {
        let xtemp: f64 = x*x - y*y + cx;
        y = 2.0*x*y + cy;
        x = xtemp;

        iter += 1;
    }

    // let color_value: u8 = 255 - (iter as f64 * 255.0 / MAX_ITER as f64) as u8;
    // Rgb([color_value, color_value, color_value])
    if iter == MAX_ITER {
        Rgb([0, 0, 0])
    } else {
        let color_i: u8 = (iter % 16) as u8;

        let color: (u8, u8, u8) = COLOR_PALETTE[color_i as usize];
        
        Rgb([color.0, color.1, color.2])
    }
}

fn main() {
    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let cx: f64 = RE_START + (x as f64 / WIDTH as f64) * (RE_END - RE_START);
        let cy: f64 = IM_START + (y as f64 / HEIGHT as f64) * (IM_END - IM_START);

        *pixel = is_mandel(cx, cy);
    }

    
    // Sauvegardez l'image dans un fichier (exemple : "mandelbrot.png")
    match image.save("mandelbrot.png") {
        Ok(_) => println!("L'image a été sauvegardée avec succès."),
        Err(e) => eprintln!("Erreur lors de la sauvegarde de l'image : {:?}", e),
    }
}
