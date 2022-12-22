use crate::player::*;
use crate::functions::*;

pub fn rabbit(p: &mut Player) -> bool {
	println!("\n\n\tYou start hunting, hoping to find some prey to keep you fed for a while.\n\tAfter some time walking, you come across ball shaped feces laying in some grass.\n\tLater on, you find a tree with some bark gnawed off. The tree's branches have also been bitten off at approximately 45 degree angles.");
	loop {
		println!("\n\n\n\t\tWhat animal is this?\n\t\t\t1. White-tailed deer\n\t\t\t2. Cottontail rabbit\n\t\t\t3. Coyote\n\t\t\t4. Beaver");
		let choice:u8 = get_input("\tEnter option: ");
		match choice {
			1 => {println!("\n\n\tYou begin hunting for a white-tailed deer. After searching for a while, you never come across one.");return false;},
			2 => 
			{
				println!("\n\n\tYou figure if you are hunting a rabbit, you should get some bait and set up a trap. After putting down a small carrot on the ground, you wait for your prey.\n\tEventually, a rabbit comes hopping up and meets its fate!");
				p.add_hunger(65);
				return true;
			},
			3 => {println!("\n\n\tYou start trying to find the tracks of a coyote, and realize later that you shouldn't be hunting a coyote.");return false;},
			4 => {println!("\n\n\tYou wait by the water for any signs of a beaver. Lots of time pass, and you give up.");return false;},
			_ => {println!("\n\n\tPlease enter a correct choice.");continue;},
		}
	}
}

pub fn deer(p: &mut Player) -> bool {
	println!("\n\n\tAfter looking for signs of an animal for a while, you come across pellet shaped feces stacked in a pile.\n\tOn a closeby tree, you can see marks made into the bark. There also seems to be dead grass in an oval shape on a hill you can see.");
	loop {
		println!("\n\n\n\t\tWhat animal is this?\n\t\t\t1. White-tailed deer\n\t\t\t2. Cottontail rabbit\n\t\t\t3. Coyote\n\t\t\t4. Beaver");
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
			_ => {println!("\n\n\tPlease enter a correct choice.");continue;},
		}
	}
}