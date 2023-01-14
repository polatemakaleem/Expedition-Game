use std::io::Write;

use crate::player::*;
use crate::functions::*;

pub fn rabbit(p: &mut Player) -> bool {
	println!("\n\n\t\tYou start hunting, hoping to find some prey to keep you fed for a while.\n\t\tAfter some time walking, you come across ball shaped feces laying in some grass.\n\t\tLater on, you find a tree with some bark gnawed off. The tree's branches have also been bitten off at approximately 45 degree angles.");
	loop {
		println!("\n\n\t\tWhat animal is this?\n\t1. White-tailed deer\n\t2. Cottontail rabbit\n\t3. Coyote\n\t4. Beaver");
		if p.sac == 1 {
			print!("\t5. Ask Sacagawea for help\n");
			std::io::stdout().flush().unwrap();
		}
		let choice:u8 = get_input("\tEnter option: ");
		match choice {
			1 => {println!("\n\n\tYou begin hunting for a white-tailed deer. After searching for a while, you never come across one.");return false;},
			2 => 
			{
				println!("\n\n\t\tYou figure if you are hunting a rabbit, you should get some bait and set up a trap.\n\t\tAfter putting down a small carrot on the ground, you wait for your prey.\n\t\tEventually, a rabbit comes hopping up and meets its fate!");
				p.add_hunger(75);
				return true;
			},
			3 => {println!("\n\n\tYou start trying to find the tracks of a coyote, and realize later that you shouldn't be hunting a coyote.");return false;},
			4 => {println!("\n\n\tYou wait by the water for any signs of a beaver. Lots of time pass, and you give up.");return false;},
			5 => {
				if p.sac != 1 {println!("\n\n\tPlease enter a correct choice.");continue;}

				println!("\n\n\t\tShe says the animal tracks belong to a rabbit. She says she knows where to hunt the rabbit.\n\t\tAfter waiting for a couple minutes, she sees the rabbit and shoots it with her bow.");
				p.add_hunger(75);
				p.sac = 2;
				return true;
			}
			_ => {println!("\n\n\tPlease enter a correct choice.");continue;},
		}
	}
}

pub fn deer(p: &mut Player) -> bool {
	println!("\n\n\t\tAfter looking for signs of an animal for a while, you come across pellet shaped feces stacked in a pile.\n\t\tOn a closeby tree, you can see marks made into the bark. There also seems to be dead grass in an oval shape on a hill you can see.");
	loop {
		println!("\n\n\n\t\tWhat animal is this?\n\t1. White-tailed deer\n\t2. Cottontail rabbit\n\t3. Coyote\n\t4. Beaver");
		if p.sac == 1 {
			println!("\t5. Ask Sacagawea for help");
		}
		let choice:u8 = get_input("\tEnter option: ");
		match choice {
			1 => 
			{
				println!("\n\n\tYou begin keeping near the oval-shaped dead grass cluster, as you think it is the deer's bedding.\n\tAfter some time, you spot a full-grown buck and you shoot it dead.");
				p.add_hunger(100);
				return true;
			},
			2 => {println!("\n\n\tYou begin looking for a rabbit. You set up a trap, wait for your prey, and nothing shows up.");return false;},
			3 => {println!("\n\n\tYou start trying to find the tracks of a coyote, and realize later taht you shouldn't be hunting a coyote.");return false;},
			4 => {println!("\n\n\tYou wait by the water for any signs of a beaver. Lots of time pass, and you give up.");return false;},
			5 => {
				if p.sac != 1 {println!("\n\n\tPlease enter a correct choice.");continue;}

				println!("\n\n\tShe says the animal tracks belong to a deer. She says she knows where to hunt the deer.\n\tAfter waiting for a couple minutes, she sees the deer and shoots it with her bow.");
				p.add_hunger(100);
				p.sac = 2;
				return true;
			}
			_ => {println!("\n\n\tPlease enter a correct choice.");continue;},
		}
	}
}

pub fn beaver(p: &mut Player) -> bool {
	println!("\n\n\tAfter walking on a trail for a decent amount of time, you come across a tree with a giant chunk taken out the side of it.\n\tNearby, you find a footprint that kind of looks human, except the heel is shrunken. The sound of rushing water nearby overwhelms every other sound.");
	println!("\n\n\t\tWhat animal is this?\n\t1. White-tailed Deer\n\t2. Cottontail Rabbit\n\t3. Coyote\n\t4. Beaver");
	if p.sac == 1 {
		print!("\t5. Ask Sacagawea for help\n");
		std::io::stdout().flush().unwrap();
	}
	loop {
		let num:u8 = get_input("\n\nEnter choice: ");
		match num {
			1 => {println!("\n\n\tYou begin hunting for a white-tailed deer. After searching for a while, you never come across one.");return false;},
			2 => {println!("\n\n\tYou begin looking for a rabbit. You set up a trap, wait for your prey, and nothing shows up.");return false;},
			3 => {println!("\n\n\tYou start trying to find the tracks of a coyote, and realize later taht you shouldn't be hunting a coyote.");return false;},
			4 => {
				println!("\n\n\tThinking it is a beaver, you stalk out the river nearby in the hopes of finding its den. You find a hole near a tree that looks to be the den of the beaver.\n\tWaiting there for a small while, the beaver comes back with bark in its mouth.\n\tYou kill it, giving food for your party.");
				p.add_hunger(75);
				println!("\n\n\tThe beaver is shot near a river, which you can easily get water from.\n\n\t\t\t\t\tTHIRST REFILLED");
				p.thirst = 100;
				return true;
			}
			5 => {
				if p.sac != 1 {println!("\n\n\tPlease enter a correct choice.");continue;}

				println!("\n\n\tShe says the animal tracks belong to a beaver. She says she knows where to hunt the beaver.\n\tAfter waiting for a couple minutes, she sees the beaver and shoots it with her bow.");
				p.add_hunger(75);
				println!("\n\n\tThe beaver is shot near a river, which you can easily get water from.\n\n\t\t\t\t\tTHIRST REFILLED");
				p.thirst = 100;
				p.sac = 2;
				return true;

			}
			_ => {println!("\n\n\tPlease enter a correct choice.");continue;},
		}
	}
}

pub fn pronghorn(p: &mut Player) -> bool {
	println!("\n\n\t\tHungry for food and craving new and exciting wildlife, you trek on looking for new prey.\n\n\t\tYou come across a footstep of a hoof that looks remarkably similar to a deer, except that the outside of the track is more concave than usual.\n\t\tYou look for more signs of animals nearby, coming across what also looks like typical deer feces, except a bit flattened.\n\n\t\tWhat animal is this?\n\t1. White-tailed deer\n\t2. Beaver\n\t3. Coyote\n\t4. Pronghorn");
	if p.sac == 1 {
		print!("\t5. Ask Sacagawea for help\n");
		std::io::stdout().flush().unwrap();
	}
	loop {
		let num:u8 = get_input("\n\nEnter choice: ");
		match num {
			1 => {println!("\n\n\t\tThinking of a white-tailed deer, you stalk out where you can usually find them.\n\t\tAfter waiting for a long time, you eventually give up.");return false;},
			2 => {println!("\n\n\t\tThinking of a beaver, you look for a water source nearby.\n\t\tYou look for hours, even getting lost. You find your way back towards your goal and move on.");return false;},
			3 => {println!("\n\n\t\tYou start hunting for a coyote. Then, you realize that you shouldn't be hunting for a coyote.");return false},
			4 => {
				println!("\n\n\t\tYou think this is a new type of animal, and you start following the tracks and find a weird looking deer.\n\t\tWell, it's close enough to a regular deer that it died when you shot it!");
				p.add_hunger(75);
				return true;
			}
			5 => {
				if p.sac != 1 {println!("\n\n\tPlease enter a valid choice.");continue;}

				println!("\n\n\t\tShe says these tracks look like a pronghorn, which are very similar to regular white-tailed deer.\n\n\t\tShe knows how to hunt these animals, and instantly finds it and shoots it.");
				p.sac = 2;
				p.add_hunger(75);
				return true;
			},
			_ => {println!("\n\n\t\tPlease enter a valid choice.");continue;},
		}
	}
}