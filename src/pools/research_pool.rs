use crate::player::*;
use crate::functions::*;
use rand::Rng;

pub fn wild_plant(p: &mut Player) -> bool {
	println!("\n\n\tAs this is your first time in this new land, your partner immediately goes to trying to find edible plants.\n\tHe comes across some weird flower that he believes is edible. He wants you to eat it.");
	loop {
	println!("\n\n\t\tDo you eat the plant?\n\t1. Yes\n\t2. No");
	let choice:u8 = get_input("\nEnter choice: ");
	match choice {
		1 => {
			let mut rng = rand::thread_rng();
			println!("\n\n\t\tYou take a bite out of the plant.");
			if rng.gen_range(1..=4) == 1 {
				println!("\n\n\t\tIt tastes really good! Hunger recovered.");
				p.add_hunger(20);
			} else {
				println!("\n\n\t\tIt tastes disgusting, and you feel incredibly nauseous. Hunger remvoed.");
				p.hunger -= 20;
			}
			p.score += 25;
			return true;
		},
		2 => {
			println!("\n\n\t\tYour partner, scoffing, eats the plant in front of you and boasts about how good it tastes.");
			return false;
		},
		_ => {
			println!("\n\n\t\t\\tEnter a valid choice.");
			continue;
		},
	} 
}
}



