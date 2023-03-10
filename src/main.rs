mod pools;
mod player;
mod functions;

use rand::prelude::*;

use functions::*;
use player::*;
use pools::hunt_pool::*;
use pools::research_pool::*;
use pools::trail_pool::*;


const NUMTRAIL:i8 = 6; //field for number of trail events


fn main() {
	loop {

		println!("\t\t\t\tLEWIS & CLARK EXPEDITION");

		let mut p = Player {hunger:100, thirst:100, score:0, money:150, furs:0, sac:0, dead:false, party:false}; //starting values

    	game(&mut p);

		println!("\n\n\t\t\t\t      FINAL SCORE: {}\n\n", p.score);

		next();
		if p.dead {
			println!("\n\n\t\tYour objective fails and you die in obscurity. The president decides to not let anyone move to the West for the duration of his term as president, in fear of the danger that lies in the West.\n\t\tThis leads to the United States not growing as a country until its too late. The United States gets invaded by Spain and meets its end.\n\n\t\t\t\t\tGAME OVER");		
		} else if p.score <= 500 {
			println!("\n\n\t\tYou make it back home from your adventure, but you didn't learn much about this new territory.\n\t\tThe president allows access, but hesitantly. The fact that people knew less about the new lands meant that there were more casualties.\n\n\t\t\t\t\tGAME OVER");
		} else {
			println!("\n\n\t\tYou make it back home and you are hailed as gods amongst men. The president is extremely happy with your work, and happily allows people to move out to the West.\n\t\tNot only does he allow people to move over, he also makes the land completely free of charge.\n\n\t\t\t\t\tGAME OVER");
		}
		loop {
			println!("\n\t\tDo you want to play again? (y/n)");
			let response:String = get_input("\nENTER RESPONSE (Y/N): ");
			let c:char = match response.trim().chars().nth(0) {
				Some(val) => val,
				None => {println!("\n\n\t\tPlease try entering (y/n) again.");continue;},
			};
			if  c.to_ascii_uppercase() == 'Y' {break;}
			else if c.to_ascii_uppercase() == 'N' {sources();next();return;}
			else {println!("\n\n\t\tPlease try entering (y/n) again.");continue;}
	}
	}
}

fn game(p: &mut Player) {
	println!("\n\n\t\tYou start your journey by getting a new job from the current president of the United States.\n\t\tHe recently bought a huge chunk of land and he isn't sure what it contains.\n\t\tHe needs a group of people to go out and research the land.\n\n\t\tHe sees you as trustworthy, and allows you to build your own crew.\n\t\tNaturally, you bring your best friend with you.\n\t\tThe party naturally grows as you allow more and more people to join.\n\n\t\tYou start your adventure through the West.");

	let mut first_trail = true;
	const WEEK_AMOUNT:usize = 4; //amount of days
	const TURN_AMOUNT:usize = 3; //turns per day
	let mut rng = rand::thread_rng();
    

	next();

    for i in 0..WEEK_AMOUNT {
		if p.sac == 2 {
			println!("\n\t\tSacagewea looks like she is ready to help again.");
			p.sac = 1;
		}
		let mut pass:bool;
		for j in 0..TURN_AMOUNT {
			let mut selection:u8;
			loop {
			println!("\n\n\t\t\t\t\tWEEK {}", i+1);
			println!("\n\t\t\t\t    Weeks left: {}", (WEEK_AMOUNT- i));
			println!("\n\t\t\t\t\tTurn {}\n\n\t\tHunger: ({} / 100)\t\t\tThirst: ({} / 100)\n\n\t\t1. Hunt for food\n\t\t2. Do research on current location\n\t\t3. Continue on trail", j+1, p.hunger, p.thirst);
			selection = get_input("\tEnter choice for turn: ");

			

			pass = match selection { //pass = if they succeeded for failed on event
				1 => {if first_trail {println!("\n\n\t\tPlease start the expedition by Continuing on the trail.");continue;} p.hunger -= 20; p.thirst -= 20; hunt(rng.gen_range(1..=4), p)}, //hunt with random event from hunt pool
				2 => {if first_trail {println!("\n\n\t\tPlease start the expedition by Continuing on the trail.");continue;} p.hunger -= 20; p.thirst -= 20; research(rng.gen_range(1..=3), p)}, //research with random event from research pool
				3 => {
					p.hunger -= 20;
					p.thirst -= 20;
					if first_trail {
						println!("\n\n\tBefore beginning your journey, a man named Toussaint Charbonneau proposes he joins.\n\tHe says he knows English and enough native tongue to be able to translate between the two. He asks if he can join.");
						loop {
							let choice:u8 = get_input("\n\n\t\t\tDo you let him join?\n\t\t1. Let him join\n\t\t2. Do not let him join\n\n\tEnter choice: ");
							match choice {
								1 => {println!("\n\tKnowing that a translator will definitely be needed in communicating with natives, you let him join.");break;},
								2 => {println!("\n\tAfter you decline his offer, he insists that without his help you could be murdered by a bunch of natives mistaking you for soldiers trying to steal their land.");},
								_ => {println!("\n\tPlease enter 1 or 2.");continue;},
							} 

						}
						first_trail = false;
						true
					}
					else {
						trail(rng.gen_range(1..=NUMTRAIL), p) //random trail event from trail pool
					}
				},
					_ => {println!("\n\n\t\t\t\tPlease enter a valid selection.");continue;}, 
			};
			break;
		}
			
			
			if !pass {
				println!("\n\n\t\t\t\t\tEVENT FAILED.");
				p.hunger -= 20;
			} else {
				println!("\n\n\t\t\t\t\tEVENT PASSED!");

				if selection == 1 {p.score += 10;} else {p.score += 20}; // if they went hunting, get less score
				if p.sac != 0 {
					p.score += 20;
				}
			} 

			if p.dead {
				println!("\n\n\t\t\t\t\t   YOU DIED!");
				return;
			}

			if p.hunger <= 0 {
				println!("\n\n\t\t\t\t\tYOU DIED FROM STARVATION.");
				p.dead =true;
				return;
			}
			if p.thirst <= 0 {
				println!("\n\n\t\t\t\t\tYOU DIED FROM THIRST.");
				p.dead = true;
				return;
			}
			p.score += 20;
			next();
		}
    }
	println!("\n\n\t\t\t\tYOU REACHED THE WEST COAST!");
}

fn hunt(num:i8, p: &mut Player) -> bool {
	match num {
		1 => {rabbit(p)},
		2 => {deer(p)},
		3 => {beaver(p)},
		4 => {pronghorn(p)},
		_ => {println!("This should be impossible");return false},
	}
}

fn research(num: i8, p: &mut Player) -> bool {
	match num {
		1 => {wild_plant(p)},
		2 => {fruit(p)},
		3 =>{native_craft(p)},
		_ => {println!("This should be impossible");return false; },
	}
}

pub fn trail(num:i8, p: &mut Player) -> bool {
	match num {
		1 => { if p.money == 0 {let mut rng = rand::thread_rng(); return trail(rng.gen_range(1..=NUMTRAIL), p);} fur_trader(p) }, // cannot encounter trader if no money
		2 => {let mut rng = rand::thread_rng(); if rng.gen_bool(1.0 / 3.0) {return trail(rng.gen_range(1..=NUMTRAIL), p);} river_fee(p)}, // 2/3 chance of trying another event
		3 => {
			if p.sac == 0 { find_sac(p) }
			else {
				let mut rng = rand::thread_rng();
				trail(rng.gen_range(1..=NUMTRAIL), p)
			}
		},
		4 => { if p.furs == 0 {let mut rng = rand::thread_rng(); return trail(rng.gen_range(1..=NUMTRAIL), p);} village_store(p) }, // cannot enter store if you don't have furs
		5 => {river_cross(p)},
		6 => { if !p.party {unfriendly_village(p)} else {let mut rng = rand::thread_rng();return trail(rng.gen_range(1..=NUMTRAIL), p);}},
		_ => {println!("This should be impossible");return false},
	}
}





fn sources() { //MLA FORMATTED
	println!("\n\n\n\t\t\t\t\tSOURCES:");

	println!("\n\tHoward, H. (2018). Sacagawea. National Parks Service. Retrieved January 3, 2023, from \n\t\thttps://www.nps.gov/lecl/learn/historyculture/sacagawea.htm#:~:text=Historians\n\t\t%20generally%20believe%20that%20Sacagawea,woman%2C%20Sacagawea%20helped%20the%20Corps.");
	println!("\n\tU.S. Department of the Interior. (2021, September 1). Trade, tribes, and transition on\n\t\tthe Missouri. National Parks Service. Retrieved January 13, 2023, from https://www.\n\t\tnps.gov/mnrr/learn/historyculture/trade-tribes-and-transition-on-the-missouri.htm");
	println!("\n\tRoos, D. (2020, January 7). Lewis and Clark's travels included dozens of astonishing\n\t\tanimal encounters. History.com. Retrieved January 13, 2023, from https://www.history.\n\t\tcom/news/lewis-and-clark-animals-american-west ");
}