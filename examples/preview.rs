extern crate piston_window;

use piston_window::*;
use colourado::*;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 4 {
		panic!("You must supply exactly 2 parameters. The palette type and the number of colors to generate!");
	}

	let num_colors = args[2].parse().unwrap_or(4);
	let palette_type = match args[1].to_lowercase().as_ref() {
		"pastel" => PaletteType::Pastel,
		"dark" => PaletteType::Dark,
		_ => PaletteType::Random
	};
	let adjacent = match args[3].to_lowercase().as_ref() {
		"adjacent" => true,
		"spread" => false,
		_ => false
	};

	let palette = ColorPalette::new(num_colors, palette_type, adjacent);

    let mut window: PistonWindow = WindowSettings::new("Color palette preview", [1280, 720]).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
		window.draw_2d(&event, |context, graphics| {
			clear([1.0; 4], graphics);

			for (i, color) in (&palette.colors).iter().enumerate() {
				rectangle([color.red, color.green, color.blue, 1.0], 
					  [(120.0 * i as f64) % 1200.0, ((120.0 * i as f64) / 1200.0).floor() * 120.0, 100.0, 100.0],
                      context.transform,
                      graphics);
			}
        });
    }
}