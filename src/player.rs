pub struct Player {
		pub hunger: i16,
		pub thirst: i16,
		pub money: u16,
		pub furs: u16,
		pub score: u16,
		pub sac: u8,
		pub dead: bool,
		pub party: bool,
}
impl Player {
		pub fn add_hunger(&mut self, num:i16) {
		if self.hunger + num <= 100 {
			self.hunger += num;
		} else {
			self.hunger = 100;
		}
		}
		pub fn pay_fur(&mut self, fur:u16) -> bool {
			if self.furs >= fur {
				self.furs -= fur;
				return true;
			}
			return false;
		}

}