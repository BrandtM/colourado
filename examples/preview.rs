extern crate piston_window;

use piston_window::*;
use colourado::*;
use std::env;

fn main() {
	let num_colors = env::args()
		.nth(1)
		.unwrap_or("4".to_string())
		.parse()
		.unwrap_or(4);

	let palette = ColorPalette::new(num_colors);

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);

			for (i, color) in (&palette.colors).iter().enumerate() {
				rectangle([color.red, color.green, color.blue, 1.0], 
					  [120.0 * i as f64, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
			}
        });
    }
}