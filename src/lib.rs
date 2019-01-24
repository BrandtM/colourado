//! A small and minimalistic library to generate a random color palette.
//! The user-facing `Color` struct contains RGB colors ranging from 0 to 1.
//! All colors are of type f32 (no exceptions)
//! 
//! # Usage
//! 
//! ```rust
//! use colourado::{Color, ColorPalette};
//! 
//! let palette = ColorPalette::new(4);
//! let random_color = palette[0].red;
//! let color_array: [f32; 3] = palette[1].to_array();
//! let hue = 315.0;
//! let saturation = 0.5;
//! let value = 0.3;
//! let rgb_color: Color = Color::hsv_to_rgb(hue, saturation, value);
//! ```

extern crate rand;

use rand::prelude::*;

trait InRange {
    fn in_range(&self, begin: Self, end: Self) -> bool;
}

impl InRange for f32 {
    fn in_range(&self, begin: f32, end: f32) -> bool {
        *self >= begin && *self < end
    }
}

/// Container for a vector of colors.
/// You can also use it to store your own custom palette of you so desire. 
pub struct ColorPalette {
    pub colors: Vec<Color>
}

/// A simple struct containing the three main color components of RGB color space.
/// Colors are stored as f32 values ranging from 0.0 to 1.0 
#[derive(Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

impl Color {
    pub fn to_array(&self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }

    /// Convert HSV to RGB. Plain and simple
    pub fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> Color {
        let chroma = value * saturation;
        let hue2 = hue / 60.0;
        let tmp = chroma * (1.0 - ((hue2 % 2.0) - 1.0).abs());
        let color2: (f32, f32, f32);

        match hue2 {
            h if h.in_range(0.0, 1.0) => color2 = (chroma, tmp, 0.0),
            h if h.in_range(1.0, 2.0) => color2 = (tmp, chroma, 0.0),
            h if h.in_range(2.0, 3.0) => color2 = (0.0, chroma, tmp),
            h if h.in_range(3.0, 4.0) => color2 = (0.0, tmp, chroma),
            h if h.in_range(4.0, 5.0) => color2 = (tmp, 0.0, chroma),
            h if h.in_range(5.0, 6.0) => color2 = (chroma, 0.0, tmp),
            _ => color2 = (0.0, 0.0, 0.0)
        }

        let m = value - chroma;
        let red = color2.0 + m;
        let green = color2.1 + m;
        let blue = color2.2 + m;

        Color {
            red, 
            green, 
            blue
        }
    }
}


impl ColorPalette {
    pub fn new(count: u32) -> ColorPalette {
        let mut rng = rand::thread_rng();

        // generate a random color but prevent it from being completely white or black
        let mut hue: f32 = rng.gen_range(0.0, 360.0);
        let mut saturation: f32 = rng.gen_range(0.5, 1.0);
        let value: f32 = rng.gen_range(0.3, 1.0);

        let mut palette: Vec<Color> = vec![];

        for i in 0..count {
            let rgb = Color::hsv_to_rgb(hue, saturation, value);
            hue = (hue + 85.0 + (i as f32) * 55.0) % 360.0;
            saturation = ((i as f32) * 0.5).sin();

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
}

#[cfg(test)]
mod tests {
    use super::ColorPalette;

    #[test]
    fn generates_palette() {
        let palette = ColorPalette::new(7);

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
