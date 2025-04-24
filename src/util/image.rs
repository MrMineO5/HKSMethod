use image::{ImageBuffer, Rgba};
use std::path::Path;

#[derive(Copy, Clone)]
pub struct Layer<T, const NX: usize, const NY: usize> {
    data: [[T; NY]; NX],
    range_x: (f64, f64),
    range_y: (f64, f64),
    merge_behaviour: fn(&T, T) -> T,
}
impl<T : Copy, const NX: usize, const NY: usize> Layer<T, NX, NY> {
    pub fn write(&mut self, x: f64, y: f64, data: T) {
        let x_index =
            ((x - self.range_x.0) / (self.range_x.1 - self.range_x.0) * NX as f64) as usize;
        let y_index =
            ((y - self.range_y.0) / (self.range_y.1 - self.range_y.0) * NY as f64) as usize;
        self.data[x_index][y_index] = (self.merge_behaviour)(&self.data[x_index][y_index], data);
    }
    
    pub fn merge(&mut self, other: &Layer<T, NX, NY>) {
        for i in 0..NX {
            for j in 0..NY {
                self.data[i][j] = (self.merge_behaviour)(&self.data[i][j], other.data[i][j]);
            }
        }
    }
}

pub fn boolean_layer<const NX: usize, const NY: usize>(
    range_x: (f64, f64),
    range_y: (f64, f64),
) -> Layer<bool, NX, NY> {
    Layer {
        data: [[false; NY]; NX],
        range_x,
        range_y,
        merge_behaviour: |a, b| *a || b,
    }
}

pub fn count_layer<const NX: usize, const NY: usize>(
    range_x: (f64, f64),
    range_y: (f64, f64),
) -> Layer<i64, NX, NY> {
    Layer {
        data: [[0; NY]; NX],
        range_x,
        range_y,
        merge_behaviour: |a, b| *a + b,
    }
}

pub fn average_layer<const NX: usize, const NY: usize>(
    range_x: (f64, f64),
    range_y: (f64, f64),
) -> Layer<(f64, u64), NX, NY> {
    Layer {
        data: [[(0.0, 0); NY]; NX],
        range_x,
        range_y,
        merge_behaviour: |(aa, na), (ab, nb)| (aa + ab, na + nb)
    }
}

#[derive(Copy, Clone)]
pub struct Image<const NX: usize, const NY: usize> {
    data: [[u32; NY]; NX],
}
impl<const NX: usize, const NY: usize> Image<NX, NY> {
    pub fn new() -> Self {
        Image {
            data: [[0xFFFFFF; NY]; NX],
        }
    }

    pub fn draw_layer<T>(&mut self, layer: &Layer<T, NX, NY>, transformer: fn(&T) -> i32) {
        for i in 0..NX {
            for j in 0..NY {
                let transformed = transformer(&layer.data[i][j]);
                if transformed < 0 {
                    continue;
                }
                self.data[i][j] = transformed as u32;
            }
        }
    }
    
    pub fn draw_count_layers(&mut self, layer1: &Layer<i64, NX, NY>, layer2: &Layer<i64, NX, NY>, color1: u32, color_mid: u32, color2: u32) {
        // Draw layers as a gradient from color1 to color2 depending on the ratio
        for i in 0..NX {
            for j in 0..NY {
                let count1 = layer1.data[i][j];
                let count2 = layer2.data[i][j];
                if count1 == 0 && count2 == 0 {
                    continue;
                }
                let ratio = count1 as f64 / (count1 + count2) as f64;
                let blended_color = if ratio > 0.5 {
                    let ratio = (ratio - 0.5) * 2.0;
                    let r1 = (color1 >> 16) & 0xFF;
                    let g1 = (color1 >> 8) & 0xFF;
                    let b1 = color1 & 0xFF;
                    let r2 = (color_mid >> 16) & 0xFF;
                    let g2 = (color_mid >> 8) & 0xFF;
                    let b2 = color_mid & 0xFF;

                    let r = ((r1 as f64 * ratio) + (r2 as f64 * (1.0 - ratio))) as u32;
                    let g = ((g1 as f64 * ratio) + (g2 as f64 * (1.0 - ratio))) as u32;
                    let b = ((b1 as f64 * ratio) + (b2 as f64 * (1.0 - ratio))) as u32;

                    (r << 16) | (g << 8) | b
                } else {
                    let ratio = ratio * 2.0;
                    let r1 = (color_mid >> 16) & 0xFF;
                    let g1 = (color_mid >> 8) & 0xFF;
                    let b1 = color_mid & 0xFF;
                    let r2 = (color2 >> 16) & 0xFF;
                    let g2 = (color2 >> 8) & 0xFF;
                    let b2 = color2 & 0xFF;

                    let r = ((r1 as f64 * ratio) + (r2 as f64 * (1.0 - ratio))) as u32;
                    let g = ((g1 as f64 * ratio) + (g2 as f64 * (1.0 - ratio))) as u32;
                    let b = ((b1 as f64 * ratio) + (b2 as f64 * (1.0 - ratio))) as u32;

                    (r << 16) | (g << 8) | b
                };
                
                self.data[i][j] = blended_color;
            }
        }
    }

    pub fn draw_boolean_layer(&mut self, layer: &Layer<bool, NX, NY>, color: u32) {
        for i in 0..NX {
            for j in 0..NY {
                if layer.data[i][j] {
                    self.data[i][j] = color;
                }
            }
        }
    }
    
    pub fn draw_gradient_layer(&mut self, layer: &Layer<(f64, u64), NX, NY>, min_color: u32, max_color: u32) {
        let mut values: [[f64; NX]; NY] = [[0.0; NX]; NY];
        let mut max: f64 = f64::NEG_INFINITY;
        let mut min: f64 = f64::INFINITY;
        for x in 0..NX {
            for y in 0..NY {
                let (sum, count) = layer.data[x][y];
                if count == 0 {
                    values[x][y] = f64::NAN;
                    continue;
                }
                let avg = sum / count as f64;
                values[x][y] = avg;
                
                if avg < min {
                    min = avg;
                }
                if avg > max {
                    max = avg;
                }
            }
        }
        
        // Extract RGB components of min and max colors
        let min_r = ((min_color >> 16) & 0xFF) as f64;
        let min_g = ((min_color >> 8) & 0xFF) as f64;
        let min_b = (min_color & 0xFF) as f64;
        
        let max_r = ((max_color >> 16) & 0xFF) as f64;
        let max_g = ((max_color >> 8) & 0xFF) as f64;
        let max_b = (max_color & 0xFF) as f64;
        
        for x in 0..NX {
            for y in 0..NY {
                if values[x][y].is_nan() {
                    continue;
                }
                let partial = (values[x][y] - min) / (max - min);
                if partial > 1.0 || partial < 0.0 {
                    panic!("Partial is out of range: {}", partial);
                }
                
                // Interpolate each color channel separately
                let r = ((max_r * partial) + (min_r * (1.0 - partial))) as u32;
                let g = ((max_g * partial) + (min_g * (1.0 - partial))) as u32;
                let b = ((max_b * partial) + (min_b * (1.0 - partial))) as u32;
                
                self.data[x][y] = (r << 16) | (g << 8) | b;
            }
        }
    }

    pub fn save_to_png(&self, filename: &str) -> Result<(), image::ImageError> {
        // Create an ImageBuffer for RGBA pixels
        let mut img = ImageBuffer::new(NX as u32, NY as u32);

        // Copy pixel data, assuming u32 is in 0xRRGGBBAA format
        for x in 0..NX {
            for y in 0..NY {
                let pixel = self.data[x][y];
                // Extract RGBA components from u32
                let r = ((pixel >> 16) & 0xFF) as u8;
                let g = ((pixel >> 8) & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;
                img.put_pixel(x as u32, y as u32, Rgba([r, g, b, 255]));
            }
        }

        img.save_with_format(Path::new(filename), image::ImageFormat::Png)?;

        Ok(())
    }
}
