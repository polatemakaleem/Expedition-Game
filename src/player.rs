    pub struct Player {
		pub hunger: i16,
		pub thirst: i16,
		pub money: u16,
		pub furs: u16,
		pub score: u16,
}
    impl Player {
		pub fn add_hunger(&mut self, num:i16) {
		if self.hunger + num <= 100 {
			self.hunger += num;
		} else {
			self.hunger = 100;
		}
		}
}