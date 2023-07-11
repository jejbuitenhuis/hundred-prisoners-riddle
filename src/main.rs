use rand::prelude::SliceRandom;

const OPEN_COUNT: u32 = 50;
const PRISONER_COUNT: u32 = 100;

fn main() {
	let mut rng = rand::thread_rng();

	let prisoners = (1..=PRISONER_COUNT).collect::< Vec<_> >();
	let mut boxes = (1..=PRISONER_COUNT).into_iter().collect::< Vec<_> >();

	boxes.shuffle(&mut rng);

	println!("Boxes: {:?}", boxes);

	let mut escaped_prisoners = Vec::with_capacity(PRISONER_COUNT as usize);

	for prisoner in prisoners {
		let mut index = (prisoner - 1) as usize;

		for _ in 1..OPEN_COUNT {
			let opened_box = boxes.get(index).expect("box should exist");

			if opened_box == &prisoner {
				escaped_prisoners.push(prisoner);

				break;
			}

			index = ( opened_box.clone() - 1 ) as usize;
		}
	}

	println!(
		"Escaped prisoners ({}/{}): {:?}",
		escaped_prisoners.len(),
		PRISONER_COUNT,
		escaped_prisoners,
	);
}
