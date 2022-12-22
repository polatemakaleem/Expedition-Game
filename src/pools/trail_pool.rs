use crate::player::*;
use crate::functions::*;



pub fn fur_trader(p: &mut Player) -> bool {
	println!("\n\n\tAfter travelling on foot for hours, making sure to document your travels, you come across what seems to be a merchant selling animal furs. He comes up to you, trying to sell his stock.");
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
					println!("\n\n\t\t\t\t\tYou pay him 1 fur. He seems pleased.");
				} else if p.money >= 25 {
					p.money -= 25;
					println!("\n\n\t\t\t\t\tYou pay him 25 money. He seems pleased.");
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