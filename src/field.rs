use crate::cell::Cell;

pub struct Field {
	width: usize,
	height: usize,
	field: Vec<Vec<Cell>>,
}

impl Field {
	pub fn new(width: usize, height: usize) -> Self {
		let field: Vec<Vec<Cell>> = vec![vec![Cell::DEAD; width]; height];
		Self {
			width,
			height,
			field,
		}
	}

	pub fn generate(&mut self) {
		for i in 0..self.height {
			for j in 0..self.width {
				self.field[i][j] = Cell::random();
			}
		}
	}

	pub fn get_field(&self) -> &Vec<Vec<Cell>> {
		&self.field
	}

	pub fn update(&mut self) {
		let field_copy = self.field.clone();
		for i in 0..self.height {
			for j in 0..self.width {
				let neighbour_count = Field::neighbour_count(&field_copy, i, j);
				if neighbour_count == 3 {
					self.field[i][j] = Cell::ALIVE;
				} else if neighbour_count < 2 || neighbour_count > 3 {
					self.field[i][j] = Cell::DEAD;
				}
			}
		}
	}

	fn neighbour_count(field_copy: &Vec<Vec<Cell>>, x: usize, y: usize) -> u8 {
		let mut count = 0;
		for i in 0..3 {
			for j in 0..3 {
				let skip = (x == 0 && i == 0)
					|| (y == 0 && j == 0)
					|| (x == field_copy.len() - 1 && i == 2)
					|| (y == field_copy[x].len() - 1 && j == 2);
				if skip {
					continue;
				}
				if field_copy[x + i - 1][y + j - 1].is_alive() {
					if i != 1 || j != 1 {
						count += 1;
					}
				}
			}
		}
		count
	}
}
