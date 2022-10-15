use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
	ALIVE,
	DEAD,
}

impl Cell {
	pub fn random() -> Self {
		let mut rng = rand::thread_rng();
		if rng.gen_bool(0.25) {
			Cell::ALIVE
		} else {
			Cell::DEAD
		}
	}

	pub fn is_alive(&self) -> bool {
		match self {
			Cell::ALIVE => true,
			_ => false,
		}
	}
}
