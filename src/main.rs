mod pools;
mod player;
mod functions;

use rand::prelude::*;
use functions::*;
use player::*;
use pools::hunt_pool::*;
use pools::research_pool::*;
use pools::trail_pool::*;

fn main() {
	println!("\t\t\t\tLEWIS & CLARK EXPEDITION");

	let mut p = Player {hunger:100, thirst:100, score:0, money:150, furs:0}; //starting values

    game(&mut p);

	next();

	println!("\n\n\t\t\t\t\tFINAL SCORE: {}", p.score);
}

fn game(p: &mut Player) {
	// variables used throughout
	const WEEK_AMOUNT:usize = 5; //amount of days
	const TURN_AMOUNT:usize = 3; //turns per day
	let mut rng = rand::thread_rng();
    

	next();

    for i in 0..WEEK_AMOUNT {
		let mut pass:bool;
		for j in 0..TURN_AMOUNT {
			loop {
			println!("\n\n\t\t\t\t\tWEEK {}", i+1);
			println!("\n\t\t\t\t\tTurn {}\n\n\t\tHunger: ({} / 100)\t\t\tThirst: ({} / 100)\n\n\t\t1. Hunt for food\n\t\t2. Do research on current location\n\t\t3. Continue on trail", j+1, p.hunger, p.thirst);
			let selection:u8 = get_input("\tEnter choice for turn: ");
			p.hunger -= 20;
			p.thirst -= 20;
			pass = match selection { //pass = if they succeeded for failed on event
				1 => hunt(rng.gen_range(1..=2), p), //hunt with random event from hunt pool
				2 => research(rng.gen_range(1..=3), p), //research with random event from research pool
				3 => trail(rng.gen_range(1..=2), p), //random trail event from trail pool
				_ => {println!("\n\n\t\t\t\tPlease enter a valid selection.");continue;}, 
			};
			break;
			}
			
			if !pass {
				println!("\n\n\t\t\t\t\tEVENT FAILED.");
				p.hunger -= 10;
			} else {
				println!("\n\n\t\t\t\t\tEVENT PASSED!");
				p.score += 20;
			} 

			if p.hunger <= 0 {
				println!("\n\n\t\t\t\t\tYOU DIED FROM STARVATION.");
				return;
			}
			if p.thirst <= 0 {
				println!("\n\n\t\t\t\t\tYOU DIED FROM THIRST.");
				return;
			}

			p.score += 20;
			next();
		}
    }
	println!("\n\n\t\t\tYOU REACHED THE WEST COAST!");
}

fn hunt(num:i8, p: &mut Player) -> bool {
	match num {
		1 => {rabbit(p)},
		2 => {deer(p)},
		3 => {return false},
		_ => {println!("\n\n\t\t\tThis should be impossible");return false},
	}
}

fn research(num: i8, p: &mut Player) -> bool {
	match num {
		1 => {wild_plant(p)},
		_ => {println!("This should be impossible");return false; },
	}
}

fn trail(num:i8, p: &mut Player) -> bool {
	match num {
		1 => {fur_trader(p)},
		2 => {river_fee(p)},
		_ => {println!("This should be impossible");return false},
	}
}

