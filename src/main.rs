use field::Field;
use piston_window::*;

mod cell;
mod field;

const TITLE: &'static str = "game of life";
const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const CELL_SIZE: usize = 20;
const CELL_COUNT_ROW: usize = WIDTH / CELL_SIZE;
const CELL_COUNT_COLUMN: usize = HEIGHT / CELL_SIZE;
const FOREGROUND: [f32; 4] = [0.375, 0.375, 0.375, 1.0];
const BACKGROUND: [f32; 4] = [0.046875, 0.046875, 0.046875, 0.9375];

#[derive(PartialEq)]
enum Game {
	WAIT,
	RUN,
}

impl Game {
	fn create(window: &mut PistonWindow) -> Self {
		window.set_position([
			(1920 / 2 - WIDTH / 2) as i32,
			(1080 / 2 - HEIGHT / 2) as i32,
		]);
		window.set_max_fps(3);
		window.set_ups(3);
		window.set_title(format!("{} - wait", TITLE));
		Game::WAIT
	}
}

fn main() {
	let mut window: PistonWindow = WindowSettings::new(TITLE, [WIDTH as u32, HEIGHT as u32])
		.exit_on_esc(true)
		.transparent(true)
		.build()
		.unwrap();
	let mut state = Game::create(&mut window);
	let mut field = Field::new(CELL_COUNT_ROW, CELL_COUNT_COLUMN);
	while let Some(event) = window.next() {
		event.mouse_scroll(|[_, scroll]| {
			let ups = window.get_event_settings().ups as f64 + scroll;
			if ups < 1.0 {
				window.set_max_fps(1);
				window.set_ups(1);
			} else if ups < 12.0 {
				window.set_max_fps(ups as u64);
				window.set_ups(ups as u64);
			}
		});
		event.press(|button| {
			if let Button::Mouse(btn) = button {
				if btn == MouseButton::Left {
					state = Game::RUN;
					window.set_title(format!("{} - run", TITLE));
					field.generate();
				}
				if btn == MouseButton::Right {
					state = Game::WAIT;
					window.set_title(format!("{} - wait", TITLE));
					field = Field::new(CELL_COUNT_ROW, CELL_COUNT_COLUMN);
				}
			}
		});
		if state == Game::RUN {
			event.update(|_| field.update());
		}
		window.draw_2d(&event, |context, graphics, _| {
			clear(BACKGROUND, graphics);
			let (mut x, mut y) = (0.0, 0.0);
			for row in field.get_field() {
				for &cell in row {
					if cell.is_alive() {
						rectangle(
							FOREGROUND,
							[x, y, CELL_SIZE as f64, CELL_SIZE as f64],
							context.transform,
							graphics,
						);
					}
					x += CELL_SIZE as f64;
				}
				x = 0.0;
				y += CELL_SIZE as f64;
			}
		});
	}
}
