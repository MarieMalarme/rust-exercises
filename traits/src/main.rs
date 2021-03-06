// Imagine you are designing a new video game and you have to create
// food that they players can take to gain strength there are two
// types of food, for now, fruits and meet

// Create a trait `Food` that requires the method gives() -> u32.
// Implement `Food` for both `Fruit` and `Meat` types
// fruit give 4 units of strength for kilogram
// Meat gives 4 units of strength for kilogram of non fat content and
// 9 units of strength for every kilogram of fat

// Implement the std::fmt::Display trait of the Player structure so using
// the template {} inside a println! macro will print in the first
// line the name of the player
// in the second line the strength, score and the money
// and in the third line the weapons
#[derive(Debug)]
struct Player {
	name: String,
	strength: u32,
	score: i32,
	money: i32,
	weapons: Vec<String>,
}

struct Fruit {
	weight_in_kg: f64,
}

struct Meat {
	weight_in_kg: f64,
	fat_content: f64,
}

fn main() {
	let apple = Fruit { weight_in_kg: 1.0 };
	assert_eq!(apple.gives(), 4);
	let steak = Meat {
		weight_in_kg: 1.0,
		fat_content: 1.0,
	};

	let mut player1 = Player {
		name: String::from("player1"),
		strength: 1,
		score: 0,
		money: 0,
		weapons: vec![String::from("knife")],
	};
	println!("Before eating {}", player1);
	player1.eat(apple);
	println!("After eating an apple\n{}", player1);
	player1.eat(steak);
	println!("After eating a steak\n{}", player1);
}

impl Player {
	fn eat<T>(&mut self, food: T) {
		self.strength += food.gives();
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_gives() {
		let apple = Fruit { weight_in_kg: 1.0 };
		assert_eq!(apple.gives(), 4);
		let steak = Meat {
			weight_in_kg: 1.0,
			fat_content: 1.0,
		};
		assert_eq!(steak.gives(), 9);
	}

	#[test]
	fn test_eat() {
		let apple = Fruit { weight_in_kg: 1.0 };
		assert_eq!(apple.gives(), 4);
		let steak = Meat {
			weight_in_kg: 1.0,
			fat_content: 1.0,
		};

		let mut player1 = Player {
			name: String::from("player1"),
			strength: 1,
			score: 0,
			money: 0,
			weapons: vec![String::from("knife")],
		};
		player1.eat(apple);
		assert_eq!(player1.strength, 5);
		player1.eat(steak);
		assert_eq!(player1.strength, 14);
	}
}
