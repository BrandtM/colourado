trait InRange {
    fn in_range(&self, begin: Self, end: Self) -> bool;
}

impl InRange for f32 {
    fn in_range(&self, begin: f32, end: f32) -> bool {
        *self >= begin && *self < end
    }
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