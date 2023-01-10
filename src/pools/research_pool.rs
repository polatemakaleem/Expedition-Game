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
				println!("\n\n\t\tIt tastes disgusting, and you feel incredibly nauseous.\n\n\t\t\t\t\tSOME HUNGER REMOVED.");
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

pub fn fruit(p: &mut Player) -> bool {
	println!("\n\n\t\tYou decide to survey the current landscape for information.\n\t\tYou determine that the land is pretty dry, but there are enough plants to make you think that the soil is nutrient.\n\t\tYou come across some bushes that look like they have fruits growing on them.\n\t\tThe fruits themselves are medium-sized and red, and they look pretty juicy.\n\t\tWhat do you do?");
	println!("\n\n\t1. Eat some yourself\n\t2. Give some to your partner\n\t3. Leave it be");
	loop {
		let num:u8 = get_input("\n\n\t\tEnter choice: ");
		match num {
			1 => {return sick(p)},
			2 => {
				println!("\n\n\t\tYou give some to your friend, who looks happy to see you are also interested in the wild plants the new land has to offer.\n\t\tHe takes a few of them at once and gulps them all down in one try.");next();println!("\n\n\t\tAfter a few hours, he looks kind of sick and about to throw up. He runs off, away from everybody, to vomit next to a tree in peace.");
				p.score += 15;
				return false;
			}
			3 => {
				println!("\n\n\t\tYou decide not to test out the wildlife of the environment. Which is, you know, your job.");
				return false;
			}
			_ => {println!("\n\n\t\t\tPlease enter a valid choice.");continue;},
		}

	}
}

fn sick(p: &mut Player) -> bool {
	p.score += 25;
	println!("\n\n\t\tYou delightly scarf down the fruit. Tastes pretty good, and you move on.");
	next();
	println!("\n\n\t\tAfter a few hours, you feel absolutely horrible. The fruit didn't sit well in your stomach, and you have to vomit. Where do you vomit?\n\n\t1. The river nearby\n\t2. A tree nearby\n\t3. Your partner");
	loop {
		let num:u8 = get_input("\n\n\t\tEnter selection: ");
		match num {
			1 => {
				println!("\n\n\t\tYou puke into the river nearby, and you see your vomit spread across the river like food coloring into a glass of water.\n\t\tHowever, this river stands in the way your party planned to go.\n\t\tThey all looked at you, each of them almost throwing up themselves. There are only two options now.\n\n\t1. Cross river filled with vomit\n\t2. Find a way around going into the water");
				loop {
					let num:u8 = get_input("\n\n\t\tEnter your selection: ");
					match num {
						1 => {
							println!("\n\n\t\tYou tell your party to cross through the river, the vomit wont make much of a difference. Each of you walk through the river as cautiously as possible, squirming at the viscosity.\n\n\t\t\t\t\tSOME HUNGER REMOVED.");
							p.hunger -= 10;
							return false;
						},
						2 => {
							println!("\n\n\t\tYou come across a makeshift bridge that spans the river. It looks like it was made by some of the native Indians. Do you cross it?\n\n\t1. Yes\n\t2. Find another way around");
							loop {
								let selection:u8 = get_input("\n\n\t\tEnter selection: ");
								match selection {
									1 => {return bridge(p);}
									2 => {
										println!("\n\n\t\tYou decide that the natives cannot be trusted with bridge-building. You keep on walking, finding a rotten, fallen log later on.\n\t\tIs this good enough to cross?\n\n\t1. Cross on the log\n\t2. Go back to bridge");
										loop {
											let s:u8 = get_input("\n\n\t\tEnter selection: ");
											match s {
												1 => {
													println!("\n\n\t\tYou begin walking over the rotten log.\n\t\tYour partners think that this idea would be stupid to do, and instead walk through the vomit-filled river.\n\t\tThe rotten log breaks (big surprise), and you fall through and right into the vomit.\n\t\tLuckily, your group members are there to save you from hurting yourself badly, and you all safely get out, even if you are covered in vomit.\n\n\t\t\t\t\tSOME HUNGER REMOVED.");
													p.score -= 15;
													p.hunger -= 10;
													return false;
												}
												2 => {return bridge(p);},
												_ => {println!("\n\n\t\tPlease enter a valid selection.");continue;},
											}
										}
									}
									_ => {println!("\n\n\t\tPlease enter a valid selection.");continue;},
								}
							}
						}
						_ => {println!("\n\n\t\t\tPlease enter a valid selection.");continue;},
					}
				}
			}
			2 => {
				println!("\n\n\t\tYou puke on a tree nearby. Your party members look towards you and mark down that the fruit was not safe to eat. Each of them make the point to avoid that spot on the ground.");
				println!("\n\n\t\tUsing the water from the river nearby, which you did not vomit in, you refill your water pouch.\n\n\n\t\t\t\t\tTHIRST REFILLED");
				p.thirst = 100;
				return true;
			}
			3 => {
				println!("\n\n\t\tWHAT IS WRONG WITH YOU? Well alright, fine. You puke on your best friend that you've known for years.\n\t\tWhen he gets covered in vomit, he goes into a rage and tears you apart piece by piece.\n\t\tYou died. Are you happy now?");
				p.dead = true;
				return false;
			}
			_ => {println!("\n\n\t\t\tPlease enter a valid selection.");continue;},
		}
	}
}

fn bridge(p: &mut Player) -> bool {
	println!("\n\n\t\tYou cross the bridge, taking time to appreciate the craftmanship of the locals. Your entire party gets across without having to swim through vomit.");
	p.score += 15;
	return true;
}



pub fn native_craft(p: &mut Player) {
	
}