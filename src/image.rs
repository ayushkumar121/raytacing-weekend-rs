use crate::{algebra::Vec3, clamp};

pub type Color = Vec3;

pub struct Image {
    width: usize,
    height: usize,
    pixel_data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Image {
            width,
            height,
            pixel_data: Vec::new(),
        }
    }

    pub fn set_data(&mut self, pixel_data: Vec<Color>) {
        self.pixel_data = pixel_data;
    }

    // pub fn get_data(&self) -> Vec<Color> {
    //     self.pixel_data
    // }

    pub fn write_ppm(&self, samples_per_pixel: i32) {
        println!("P3\n{} {}\n255", self.width, self.height);

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let color = &self.pixel_data[self.width * y + x];
                let scale = 1.0 / (samples_per_pixel as f64);

                let r = f64::sqrt(color.0 * scale);
                let g = f64::sqrt(color.1 * scale);
                let b = f64::sqrt(color.2 * scale);

                let ir = (256.0 * clamp(r, 0.0, 0.999)).floor();
                let ig = (256.0 * clamp(g, 0.0, 0.999)).floor();
                let ib = (256.0 * clamp(b, 0.0, 0.999)).floor();

                println!("{ir} {ig} {ib}");
            }
        }
    }
}
