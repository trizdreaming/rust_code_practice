extern crate rand;
use rand::distributions::{IndependentSample, Range};
use std::io;
use std::io::Write;


fn main() {
	println!("This is Fack BaseBall Game!");
	println!("if you Quit the Game, Press 'x'");
	
	//let mut targetArr = [0i32; 3];
	//let mut inputArr =[0i32; 3];

	let mut targets:Vec<i32> = vec![];
	let mut inputs:Vec<i32> = vec![];

	loop {
		ballpark_setup(&mut targets, 3);
		println!("park state : {} {} {}", targets[0], targets[1], targets[2]);

		let mut batter_state = batter_bet(&mut inputs);
		//println!("batter state : {} {} {}", inputs[0], inputs[1], inputs[2]);		
		//println!("{}", batter_state);

		let mut result = showdown(&mut inputs, &mut targets);
		println!("strike : {}, ball : {}, out : {}", result.0, result.1, result.2);
		inputs.clear();
	}
}


fn generate_i32() -> i32 {
	let range = Range::new(0i32, 9);
	let mut rng = rand::thread_rng();

	range.ind_sample(&mut rng)
}


fn ballpark_setup(targets: &mut Vec<i32>, len: i32) {
	if targets.is_empty() == false {
		targets.clear();
	}

	for x in 0..len {
		targets.push(generate_i32());
	}
}

fn batter_bet(inputs: &mut Vec<i32>) -> bool{

	println!("Bet your Count in 0 ~ 9, ex) 1,2,3");
	io::stdout().flush();

	let mut input = String::new();
	let raw = io::stdin().read_line(&mut input).ok().expect("unable to read input!");

	let mut v:Vec<&str> = input.trim().split(',').collect();
	let length = v.len();

	for x in v.iter(){
		let option_value:Option<i32> = x.parse().ok();
		inputs.push(option_value.unwrap());
		//println!("{}", x);
	}
	
	true
}

fn showdown(inputs: &mut Vec<i32>, targets: &mut Vec<i32>) -> (i32, i32, i32) {
	let mut strike = 0;
	let mut ball = 0;
	let mut out = 0;

	for (idx, input) in inputs.iter().enumerate() {
		for (match_idx, target) in targets.iter().enumerate() {
			if input == target {
				if idx == match_idx {
					strike += 1;
				}
				else {
					ball += 1;
				}
			}
		}
	}

	out = 3 - (strike + ball);

	(strike, ball, out)
}