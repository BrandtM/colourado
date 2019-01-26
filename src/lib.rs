//! A small and minimalistic library to generate a random color palette.
//! The user-facing `Color` struct contains RGB colors ranging from 0 to 1.
//! All colors are of type f32 (no exceptions)
//! 
//! # Usage
//! 
//! ```rust
//! use colourado::{Color, ColorPalette, PaletteType};
//! 
//! let palette = ColorPalette::new(4, PaletteType::Random, false);
//! let random_color = palette.colors[0].red;
//! let color_array: [f32; 3] = palette.colors[1].to_array();
//! let hue = 315.0;
//! let saturation = 0.5;
//! let value = 0.3;
//! let rgb_color: Color = Color::hsv_to_rgb(hue, saturation, value);
//! ```
//! 
//! The second param to ColorPalette::new() determines the color scheme.  
//! Currently 3 different schemes are supported:  
//! `PaletteType::Random` generates random colors 
//! `PaletteType::Pastel` generates pastel colors 
//! `PaletteType::Dark` generates dark colors  
//! 
//! The third param determines whether colors are generated close to each other
//! or are spread apart. `true` generates adjacent colors while `false` will generate
//! a very spread color palette.
//! 

extern crate rand;

use rand::prelude::*;

mod color;
pub use color::Color;

/// Container for a vector of colors.
/// You can also use it to store your own custom palette of you so desire. 
pub struct ColorPalette {
    pub colors: Vec<Color>
}

pub enum PaletteType {
    Random,
    Pastel,
    Dark,
}

impl ColorPalette {
    pub fn new(count: u32, palette_type: PaletteType, adjacent_colors: bool) -> ColorPalette {
        let mut rng = rand::thread_rng();

        // generate a random color but prevent it from being completely white or black
        let mut hue: f32;
        let mut saturation: f32;
        let mut value: f32;

        match palette_type {
            PaletteType::Random => {
                hue = rng.gen_range(0.0, 360.0);
                saturation = rng.gen_range(0.5, 1.0);
                value = rng.gen_range(0.3, 1.0);
            },
            PaletteType::Pastel => {
                hue = rng.gen_range(0.0, 360.0);
                saturation = rng.gen_range(0.1, 0.4);
                value = rng.gen_range(0.7, 1.0);
            },
            PaletteType::Dark => {
                hue = rng.gen_range(0.0, 360.0);
                saturation = rng.gen_range(0.5, 1.0);
                value = rng.gen_range(0.0, 0.4);
            }
        }

        let mut palette: Vec<Color> = vec![];
        let mut base_divergence = 80.0;

        if adjacent_colors == true {
            base_divergence = 25.0;
        }

        base_divergence -= (count as f32) / 2.6;

        for i in 0..count {
            let rgb = Color::hsv_to_rgb(hue, saturation, value);

            match palette_type {
                PaletteType::Random => {
                    ColorPalette::palette_random(&mut hue, &mut saturation, &mut value, i as f32, base_divergence);
                },
                PaletteType::Pastel => {
                    ColorPalette::palette_pastel(&mut hue, &mut saturation, &mut value, i as f32, base_divergence);
                },
                PaletteType::Dark => {
                    ColorPalette::palette_dark(&mut hue, &mut saturation, &mut value, i as f32, base_divergence);
                }
            }

            palette.push(Color {
                red: rgb.red,
                green: rgb.green,
                blue: rgb.blue
            });
        }

        ColorPalette {
            colors: palette
        }
    }

    fn palette_dark(hue: &mut f32, saturation: &mut f32, value: &mut f32, iteration: f32, divergence: f32) {
        let f = (iteration * 43.0).cos().abs();
        let mut div = divergence;

        if div < 15.0 {
            div = 15.0;
        }

        *hue = (*hue + div + f).abs() % 360.0;
        *saturation = 0.32 + ((iteration * 0.75).sin() / 2.0).abs();
        *value = 0.1 + (iteration.cos() / 6.0).abs();
    }

    fn palette_pastel(hue: &mut f32, saturation: &mut f32, value: &mut f32, iteration: f32, divergence: f32) {
        let f = (iteration * 25.0).cos().abs();
        let mut div = divergence;

        if div < 15.0 {
            div = 15.0;
        }

        *hue = (*hue + div + f).abs() % 360.0;
        *saturation = ((iteration * 0.35).cos() / 5.0).abs();
        *value = 0.5 + (iteration.cos() / 2.0).abs();
    }

    fn palette_random(hue: &mut f32, saturation: &mut f32, value: &mut f32, iteration: f32, divergence: f32) {
        let f = (iteration * 55.0).tan().abs();
        let mut div = divergence;

        if div < 15.0 {
            div = 15.0;
        }

        *hue = (*hue + div + f).abs() % 360.0;
        *saturation = (iteration * 0.35).sin().abs();
        *value = ((6.33 * iteration) * 0.5).cos().abs();

        if *saturation < 0.4 {
            *saturation = 0.4;
        }

        if *value < 0.2 {
            *value = 0.2;
        } else if *value > 0.85 {
            *value = 0.85;
        }        
    }
}

#[cfg(test)]
mod tests {
    use super::ColorPalette;
    use super::PaletteType;

    #[test]
    fn generates_palette() {
        let palette = ColorPalette::new(7, PaletteType::Random, false);

        for color in palette.colors {
            assert!(color.red >= 0.0);
            assert!(color.red <= 1.0);

            assert!(color.green >= 0.0);
            assert!(color.green <= 1.0);

            assert!(color.blue >= 0.0);
            assert!(color.blue <= 1.0);
        }        
    }
}
