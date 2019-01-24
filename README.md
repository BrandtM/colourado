# Colourado  

A small and minimalistic library to generate a random color palette.  
The user-facing `Color` struct contains RGB colors ranging from 0 to 1.  
All colors are of type f32 (no exceptions)  

# Usage

```rust
use colourado::{Color, ColorPalette};

let palette = ColorPalette::new(4);
let random_color = palette[0].red;
let color_array: [f32; 3] = palette[1].to_array();
let hue = 315.0;
let saturation = 0.5;
let value = 0.3;
let rgb_color: Color = Color::hsv_to_rgb(hue, saturation, value);
```

## Example  

A color palette might look like this when rendered:  

![Example image](https://raw.githubusercontent.com/BrandtM/colourado/master/examples/example.png)  

