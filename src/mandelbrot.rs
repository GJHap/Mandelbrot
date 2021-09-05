use image::GrayImage;
use rayon::prelude::*;

fn escape_time(x_0: f32, y_0: f32) -> u32 {
    let mut x = 0.0;
    let mut x_2 = 0.0;

    let mut y_2 = 0.0;
    let mut y = 0.0;

    let mut iter = 0;
    let max_iter = 1000;
    while x_2 + y_2 <= 4.0 && iter < max_iter {
        y = 2.0 * x * y + y_0;
        x = x_2 - y_2 + x_0;
        x_2 = x * x;
        y_2 = y * y;

        iter += 1;
    }

    iter
}

fn get_color_palette() -> Vec<u8> {
    (0..=255).rev().collect()
}

fn get_index_value(index: f32, min: f32, step: f32) -> f32 {
    min + (index + 1.0) * step
}

fn get_pixels(width: u32, height: u32) -> Vec<u8> {
    let x_min = -2.0;
    let x_max = 0.8;
    let step_x = (x_max - x_min) / width as f32;

    let y_min = -1.2;
    let y_max = 1.2;
    let step_y = (y_max - y_min) / height as f32;

    let color_palette = get_color_palette();

    (0..height)
        .into_par_iter()
        .flat_map(|y| {
            let y_val = get_index_value(y as f32, y_min, step_y);

            (0..width)
                .into_par_iter()
                .map(|x| {
                    let x_val = get_index_value(x as f32, x_min, step_x);
                    color_palette[(escape_time(x_val, y_val) % 256) as usize]
                })
                .collect::<Vec<u8>>()
        })
        .collect()
}

pub fn generate(width: u32, height: u32) -> GrayImage {
    GrayImage::from_vec(width, height, get_pixels(width, height)).unwrap()
}
