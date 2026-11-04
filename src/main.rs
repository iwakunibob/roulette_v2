// Roulette Game in RUST Programming Language
use std::io;
use std::collections::HashMap;
use rand::prelude::*;
use std::time::Instant;
use colored::Colorize;

const EUROPEAN_WHEEL: [u8; 37] = [0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 
	30, 8, 23, 10, 5, 24, 16, 33, 1, 20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26];
const AMERICAN_WHEEL: [u8; 38] = [0, 28, 9, 26, 30, 11, 7, 20, 32, 17, 5, 22, 34, 15, 3, 
	24, 36, 13, 1, 100, 27, 10, 25, 29, 12, 8, 19, 31, 18, 6, 21, 33, 16, 4, 23, 35, 14, 2];
const RED_NUMBERS: [u8; 18] = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
const BLACK_NUMBERS: [u8; 18] = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];
// Formerly version 1 program was 168ms for 1M spins attempting to speed up using arrays not vectors
fn main() {
	println!("start");
	let wheel_type = select_wheel();
	let wheel = create_wheel(&wheel_type);
	let colors: HashMap<u8, char> = create_color_map(&wheel);
	let start = Instant::now();
	std::thread::sleep(std::time::Duration::from_millis(100));
	let many_spins = spin_x(&wheel, 1_000_000);
	// println!("{}", string_pockets(&many_spins, &colors));
	print_results(&wheel_type, &wheel, &many_spins, &colors);
	println!();
	print_wheel(&wheel_type, &wheel, &colors);
	// print_pocket(&spin(&wheel), &colors);
	// println!();
	// print_pocket(&spin(&wheel), &colors);
	// println!();
	let few_spins = spin_x(&wheel, 60);
	print_pockets(&few_spins, &colors);
	print_results(&wheel_type, wheel, &few_spins, &colors);
	println!();
	print_wheel(&wheel_type, &wheel, &colors);
	let elapsed = start.elapsed();
	println!("Elapsed time: {:.2?}", elapsed);
	println!("Done")
}

fn select_wheel() -> String {
	let mut input = String::new();
	loop {
		println!("How many green pockets are on the wheel (1 or 2)? ");
		input.clear();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		let input_trim = input.trim();
		if input_trim == "1" {
			println!("You selected a European roulette wheel with one green pocket (0)");
			return "European".to_string();
		} else if input_trim == "2" {
			println!("You selected an American roulette wheel with two green pockets (0, 00)");
			return "American".to_string();
		} else {
			println!("\nERROR: European wheels have one green pocket (0)\
				\nAmerican wheels have to two green pockets (0 and 00).");
		}
	}	
}

fn create_wheel(wheel_type: &String) -> &'static[u8] {
	if wheel_type == "American" {
		&AMERICAN_WHEEL
	} else{
		&EUROPEAN_WHEEL
	} 
}

fn create_color_map(wheel: &[u8])-> HashMap<u8, char> {
	let mut color_map: HashMap<u8, char> = HashMap::new();
	for pocket in wheel {
		if RED_NUMBERS.contains(&pocket) {
			color_map.insert(*pocket, 'R');
		} else if BLACK_NUMBERS.contains(&pocket) {
			color_map.insert(*pocket,'B');
		} else {
			color_map.insert(*pocket,'G');
		}
	}
	return color_map;
}

fn print_wheel(wheel_type: &String, wheel: &[u8], colors: &HashMap<u8, char>) {
	for pocket in wheel {
		print_pocket(pocket, colors);
	}
	println!("\nTotal {} Wheel Pockets = {}\n", wheel_type, wheel.len());
}

fn print_pocket(pocket: &u8, colors: &HashMap<u8, char>){
	let color =colors.get(pocket).copied().unwrap_or(' ');	
	let mut str_pocket = pocket.to_string();
	if str_pocket == "100" {
		str_pocket = "00".to_string();
	}
	str_pocket =  format!("{:>2}{}", str_pocket, color);
		match color {
		'R' => print!("{} ", str_pocket.on_red()),
		'B' => print!("{} ", str_pocket.on_bright_black()),
		'G' => print!("{} ", str_pocket.on_green()),
		_ => print!("{} ", str_pocket)
	};
}

fn print_pockets(pockets: &Vec<u8>, colors: &HashMap<u8, char>) {
	for (index, pocket) in pockets.iter().enumerate() {
		print!("Spin {:>2}: ", index + 1);
		print_pocket(pocket, colors);
		println!();
	}
}

fn spin(wheel: &[u8]) -> u8 {
	let idx:usize = rand::rng().random_range(0..= wheel.len()-1);
	return wheel[idx];
}

fn spin_x(wheel: &[u8], spins: usize) -> Vec<u8> {
	let mut results: Vec<u8> = vec![];
	for _i in 1..=spins {
		let pocket = spin(&wheel);
		results.push(pocket);
	}
	return results;
}

fn print_results(wheel_type: &String, wheel: &[u8], pockets: &Vec<u8>, colors: &HashMap<u8, char>) {
	let color_order = ['R', 'B', 'G'];
	let color_word: HashMap<char, &str> = HashMap::from([('R'," Red "), ('B', "Black"), ('G', "Green")]);
	println!("\nResults for {} Roulette wheel after {} spins", wheel_type, pockets.len());
	println!("----------------------------------------");
	let mut freq_color:HashMap<char, i32> = HashMap::from([('R',0), ('B', 0), ('G', 0),]); // Create color counters
	let mut freq_number: HashMap<u8, u32> = HashMap::new();
	for number in wheel {  // Create Number Frequecy HashMap Totals
		freq_number.insert(*number, 0);
	}
	for pocket in pockets { // Counter increment loop
		*freq_number.entry(*pocket).or_insert(0) += 1;
		*freq_color.entry(*colors.get(pocket).unwrap_or(&'w')).or_insert(0) += 1;
	}
	for pocket in wheel {
		let percent_pocket = ((freq_number[&pocket]) as f64 / pockets.len() as f64) * 100.0;
		print!("For ");
		print_pocket(pocket, colors);
		println!("pocket = {:2} occurances {:5.2}%", freq_number[&pocket], percent_pocket);
	}
	println!("----------------------------------------");
	for color in color_order {
		let str_color = color_word[&color].to_string() + " pockets";
		match color {
			'R' => print!("For {}", str_color.on_red()),
			'B' => print!("For {}", str_color.on_bright_black()),
			'G' => print!("For {}", str_color.on_green()),
			_ => print!("{} ", str_color)
		};
		let percent_color = freq_color[&color] as f64 / pockets.len() as f64 * 100.0;
		println!(" = {:>2} occurances {:5.2}%", freq_color[&color], percent_color);
	}
	println!();
}