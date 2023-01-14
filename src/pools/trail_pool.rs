use std::io::Write;

use rand::Rng;

use crate::player::*;
use crate::functions::*;


pub fn fur_trader(p: &mut Player) -> bool {
	println!("\n\n\tAfter travelling on foot for hours, making sure to document your travels, you come across what seems to be a white merchant\n\tselling animal furs. He comes up to you, trying to sell his stock.");
	loop {
		println!("\n\tHow many furs do you want to buy? Each fur is 25 money.\t\tMONEY: {}", p.money);
		let amount:u16 = get_input("\nEnter amount: ");
		if amount * 25 > p.money {
			println!("\n\n\t\t\tYou cannot afford that many furs. Please enter again.");
			continue;
		} else {
			p.money -= amount * 25;
			p.furs += amount;
			println!("\n\n\t\t\t\t\tYou bought {amount} furs.");
			println!("\n\n\t\t\t\t\tMONEY LEFT: {}", p.money);
			break;
		}
		}
	
	return true;
}


pub fn river_fee(p: &mut Player) -> bool {
	println!("\n\n\tYou come across a clean, fast moving river. You take the opportunity to fill your canteens with fresh water.");
	println!("\n\n\t\t\tTHIRST REFILLED.");
	p.thirst = 100;
	println!("\n\n\tAfter filling your canteens, you quickly try to leave.");
	next();
	println!("\n\tA native comes walking towards you, and it seems like he made a camp nearby.\n\tHe says that you need to give him 1 fur, or 25 money so he can buy his own.\n\tHe says this is the fee for using the river water.");
	loop {
		println!("\n\t\tYour fur count: {}\t\tYour money: {}", p.furs, p.money);
		println!("\n\t\tWhat is your choice?\n\t1. Pay him\n\t2. Don't pay him");
		let cost:u8 = get_input("\nEnter choice: ");
		match cost {
			1 => {
				if p.furs != 0 {
					p.furs -= 1;
					println!("\n\n\t\t\t\tYou pay him 1 fur. He seems pleased.");
				} else if p.money >= 25 {
					p.money -= 25;
					println!("\n\n\t\t\t\tYou pay him 25 money. He seems pleased.");
				} else {
					println!("\n\n\tYou cannot pay him.");
					continue;
				}
				return true;
			},
			2 => {
				println!("\n\n\tYou say you will not pay him. He is very aggravated, and pulls out a knife. He threatens to stab you unless you give money and all of your furs.\n\tTo preserve your life, you obey.");
				p.furs = 0;
				p.money -= p.money / 2;
				println!("\n\n\t\t\t\t\tFURS: {}\t\tMONEY: {}", p.furs, p.money);
				return false;
			},
			_ => {println!("\n\n\t\t\tEnter a valid choice.");continue;},
		}
	}
}

pub fn find_sac(p: &mut Player) -> bool {
	println!("\n\n\t\tYou stumble across a native village filled with native Indians. Toussaint Charbonneau, your translator, realized what was going on at this village.\n\t\tThis was the Hidatsa tribe, who had taken a group of young girls from their own tribes.\n\t\tCharbonneau seemed to take a liking to one of these woman, and proposes that he should buy her to be his wife.");
	println!("\n\t\tWhat is your choice?\n\t1. Let him buy the girl\n\t2. Do not let him buy the girl.");
	loop {
	let num:u8 = get_input("\n\n\t\tEnter choice: ");
	match num {
		1 => {
			println!("\n\n\tCharbonneau looks gleefully as he spends his own money to buy this girl. The girl, happy that she at least is no longer imprisoned, joins your party.\n\tCharbonneau says that her name is Sacagawea, and that she could help the expedition along.");
			p.sac = 1;
			return true;
		}
		2 => {
			println!("\n\n\tCharbonneau looks extremely sad, pouts like a sad dog, and you continue on your journey.\n\tMaybe you could have used a native to help your expedition? Who knows.");
			return false;
		}
		_ => {
			println!("Please enter 1-2.");
			continue;		
		}
	}
}
}

pub fn village_store(p: &mut Player) -> bool {
	println!("\n\n\t\tAfter walking by the river, keeping track of the land you've crossed. You eventually find a native village that has some sort of sale going on.\n\t\tYour translator recommends you check out their shop.");
	let mut water = true;let mut coffee = true;let mut meat = true;let mut map = true;
	loop {
		println!("\n\t\t\tSTOCK!\n");
		if water {print!("\t1. Pouch full of water (1 fur)\n");}
		if coffee {print!("\t2. Energy-giving Coffee (2 furs) \n");}
		if meat {print!("\t3. Cooked meat (1 fur) \n");}
		if map {print!("\t4. Tribal map (1 fur) \n");}
		print!("\t5. Leave\n");
		print!("\n\t\t\tFURS: {}", p.furs);
		std::io::stdout().flush().unwrap();
		
		let num:u8 = get_input("\n\tEnter selection: ");
		match num {
			1 => {
			if water {
				if p.pay_fur(1) {
					println!("\n\t\t\tTHIRST REFILLED");
					p.thirst = 100;
					water = false;
					continue;
				}
				else {
					println!("\n\tYou cannot pay for this item. Please select another option.");
					continue;
				}
			}
		}
			2 => {
				if coffee && p.sac != 0 {
					if p.pay_fur(2) {
					println!("\n\t\t\tSacagewea takes the coffee from you and chugs it.");
					p.sac = 1;
					coffee = false;
					continue;
				}
				else {
					println!("\n\tYou cannot pay for this item. Please select another option.");
					continue;
				}
			}
				println!("\n\t\tYou enjoy the coffee thinking about your next choices.");
				continue;
		}
			3 => {
				if meat {
					if p.pay_fur(1) {
					println!("\n\t\t\tYour party shares the cooked meat. Hunger restored.");
					p.add_hunger(100);
					meat = false;
					continue;				
				}
				else {
					println!("\n\tYou cannot pay for this item. Please select another option.");
					continue;
				}
			}
		}
			4 => {
				if map {
					if p.pay_fur(1) {
						println!("\n\t\t\tYou study the map and add it to your own edition. 50 total score added.");
						p.score += 50;
						map = false;
						continue;
					}
					else {
						println!("\n\tYou cannot pay for thsi item. Please select another option.");
						continue;
					}
				}
			}
			5 => {
				println!("\n\n\t\tGetting all you wanted, you leave the tribal shop and continue on your journey.");
				return true;
			}
			_ => {println!("\n\n\tPlease enter a valid selection.");continue;},
		}
	}
}

pub fn river_cross(p: &mut Player) -> bool {
	println!("\n\n\t\tWhile walking across the plateaus, you come across a river that you seemingly need to cross.\n\n\t\t\t\t\t\tTHIRST REFILLED\n\n\t\tThere are plenty of trees around, so it is possible to build a boat. However, that might take a lot of time.\n\n\t\tWhat is your decision?\n\t1. Build a boat\n\t2. Find another way");
	p.thirst = 100;
	loop {
	let num:u8 = get_input("\n\tEnter selection: ");
	match num {
		1 => {return boat(p);},
		2 => {return another_way(p);},
		_ => {println!("\n\tPlease enter a valid selection.");continue;},
	}
}
}


fn boat(p: &mut Player) -> bool {
	println!("\n\n\t\tAfter taking the time to cut down nearby trees and building a boat that is capable of floating, you push it into the water and begin trying to cross.");
	let mut rng = rand::thread_rng();
	if rng.gen_range(0..=4) == 2 {
		println!("\n\t\tHowever, the river is too fast, and it starts pushing you downstream ... right into a waterfall. What do you do?\n\t1. Jump out of the boat and start swimming to shore\n\t2. Fight the stream\n\t3. Grab onto nearby branches of trees\n\t4. Paddle the boat closer to shore");
		let random = rng.gen_range(0..=4);
		loop {
		let num:u8 = get_input("\n\tEnter your choice: ");
		match num {
			1 => {
				if random == 1 {
					println!("\n\t\tYou try to jump out of the boat, but your legs get caught. Your head goes straight into the water and hits a sharp rock.");
					p.dead = true;
					return false;
				}
				println!("\n\t\tYou successfully jump out of the boat and swim with your crew to the nearest shore.");
				return true;
			}
			2 => {
				if random == 2 {
					println!("\n\t\tYou try to fight the stream, but it is too strong to handle. You fall off the edge of the waterfall to meet your painful demise.");
					p.dead = true;
					return false;
				}
				println!("\n\t\tSurprisingly, the tide isnt strong enough to keep you spiraling down to your death. You, and your crew, make it out safe.");
				return true;
			}
			3 => {
				if random == 3 {
					println!("\n\t\tYou see a tree you think is nearby enough to grab. You reach out for a branch, but it is too far away.\n\tYou topple over the side of the boat, snapping your neck on impact with the riverbed.");
					p.dead = true;
					return false;
				}
				println!("\n\t\tYou grab onto a nearby tree's branch, and pull the boat to safety. Everyone exits, happy for their lives.");
				return true;
			}
			4 => {
				if random == 4 {
					println!("\n\t\tYour quickly-made paddle snaps in half as you try to paddle closer to the shore. The boat is tore apart with you in it as it flies through the waterfall");
					p.dead = true;
					return false;
				}
				println!("\n\t\tYou paddle the boat closer to shore. You and your crew jump out when it's safe.");
				return true;
			}
			_ => {println!("\n\tPlease enter a valid option.");continue;},
		}
	}
}
	println!("\n\n\t\tYou successfully make it across. The waters seem calm today.");
	return true;
}

fn another_way(p: &mut Player) -> bool {
	println!("\n\t\tAfter walking along the river for a ways, you come across a fallen tree that spans the river that looks like its been bitten off by a beaver.\n\tIt looks stable enough to walk on. Do you cross it?\n\t1. Cross the log\n\t2. Build a boat");
	loop {
	let num:u8 = get_input("\n\tEnter choice: ");

	match num {
		1 => {
			let mut rng = rand::thread_rng();
			println!("\n\n\t\tYou begin walking across the log, careful not to slip.");
			let random = rng.gen_range(0..=100);
			if random >= 0 && random <= 35 {
				println!("\n\n\t\tThe log snaps, leaving you falling into the river. The river takes you downstream to a waterfall, which you cannot save yourself from.");
				p.dead = true;
				return false;
			}
			println!("\n\n\t\tYou make it across the log. Your entire crew is safe.");
			if rng.gen_range(0..=4) != 4 {
				println!("\n\t\tWhile waiting for your crew to finish the walk across the log, you spot a beaver making a dam underneath it. You take aim with your bow and shoot it.\n\n\t\t\t\tSome hunger restored.");
				p.add_hunger(75);
			}
			return true;
		}
		2 => {return boat(p);}
		_ => {println!("\n\n\t\tPlease enter a valid selection.");continue;},
	}
}
}



pub fn unfriendly_village(p: &mut Player) -> bool {
	println!("\n\n\t\tWhile walking on unmarked trails for a while, you come across another native Indian village.\n\t\tJust like any other, you approach with your hands up in peace, making sure the natives understand you are no threat.\n\t\tHowever, this time the natives don't seem so friendly. They all immediately pull out their weapons and walk towards you, prepared for any move.");
	if p.sac != 0 {
		println!("\n\n\t\tSacagawea walks in front of your group, as if shielding all of you from being stabbed.\n\t\tShe speaks in her own tongue, and eventually the other soldiers put their weapons down.");
		println!("\n\n\t\tWhat seems to be the village chief walks out in front of his tribe.\n\n\t\tHe has a conversation with Sacagawea, and seems to be enjoying it based on his laughter. The other people put their weapons down.\n\n\t\tThe translator whispers in your ear 'It seems we have been invited to a party'.");
		println!("\n\n\t\tYou party all night, the tribe keeping you well-fed and filling your pouches.\n\n\t\tWhen you wake up in the morning, you wish you could stay with this tribe forever.");
		p.hunger = 100;
		p.thirst = 100;
		p.party = true;
		return true;
	}
	else {
		println!("\n\n\t\tWhile backed into a corner, the native that seems to be a leader comes in front of his miniature army.\n\t\tHe speaks complete gibberish, at least to your ears. Your translator then whispers in your ear that the chief wants to know why the party is there.\n\n\t\tWhat should the translator say?\n\t1. 'We are just passing by'\n\t2. 'We are here to have a good time'\n\t3. 'We are here to take your land'");
		loop {
			let num:u8 = get_input("\n\tEnter selection: ");
			match num {
				1 => {
					println!("\n\n\t\tThe translator says everyone is just passing by, and this village is a pitstop on your quest to the west coast.\n\t\tThe chief looks at the translator with a suspicious look in his eye, and every tribal member backs up, weapons still drawn.\n\t\t'We are permitting you to leave,' says the chief.\n\n\t\tWhat do you do?\n\t1. Leave in peace\n\t2. Sneak and try to steal some supplies");
					return let_go(p);
				},
				2 => {
					println!("\n\n\t\tThe chief eyes you suspiciously. 'Oh, really?' he says. 'The only way for us to trust you is to show some coin. I believe 75 coins will do.'\n\t\tDo you pay them to let you go?\n\t1. Yes\n\t2. No\n\n\t\t\tMONEY: {}", p.money);
					loop {
					let num = get_input("\n\n\t\tPlease make a selection: ");
					match num {
						1 => {
							if p.money < 75 {println!("\n\n\t\tYou cannot make this purchase. Try again.");continue;}
							p.money -= 75;
							println!("\n\n\t\tYou pay the chief. He gets really excited.\n\n\t\tYou spend the rest of the night with this tribe, who happily feed you and give you water.\n\n\t\tAfter waking up in the morning, you wish you could just live with these people. But, you have a job to do.");
							p.hunger = 100;
							p.thirst = 100;
							return true; 
						},
						2 => {println!("\n\n\t\tYou say you change your mind, and instead just want to walk away. The chief allows you to leave the property.\n\n\t\tWhat do you do?\n\t1. Leave the property\n\t2. Try to steal supplies"); return let_go(p);}
						_ => {println!("\n\n\t\tPlease enter a valid selection. ");continue;},
					}
				}
				},
				3 => {
					println!("\n\n\t\tThe chief looks at the translator with pure rage in his eyes. The soldiers advance, and you all draw your last breaths.");
					p.dead = true;
					return false;
				}
				_ => {println!("\n\n\t\tPlease enter a valid selection.");continue;},
			}

		}
	}
}


fn let_go(p: &mut Player) -> bool {
	loop {
	let num = menu(2);
	match num {
		1 => {println!("\n\n\t\tYou politely and peacefully leave the village. You think of what you could have stolen.\n\t\tHowever, it was for the best that you didn't aggravate native Indians.");return true;},
		2 => {
			let mut rng = rand::thread_rng();
			if rng.gen_bool(1.0 / 4.0) {
				println!("\n\n\t\tYou sneakily hide behind some of their huts while pretending to leave.\n\t\tYou slowly walk up to what appears to be the chief's hut, and search inside.\n\t\tYou find some furs on the ground, and what seems to be a few dollars stolen from other travelers.\n\n\t\t\t\t\tFUR STOLEN");
				p.furs += 1;
				let stolen:u16 = if rng.gen_bool(1.0 / 2.0) {25} else {50};
				println!("\n\n\t\t\t\t\t{stolen} MONEY STOLEN");
				p.money += stolen;
				return false;
			}
			else {
				println!("\n\n\t\tYou sneakily hide behind some of their huts while pretending to leave.\n\n\n\t\tOr so you thought. The chief saw you from the start, and performed a punishment he felt deserving.");
				p.dead = true;
				return false;
			}
		}
		_ => {continue;},
	}
	}
}