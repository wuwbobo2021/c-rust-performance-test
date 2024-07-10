//use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
	//let mut num_max = String::new();
	//io::stdin().read_line(&mut num_max)
	//	.expect("failed to read line");

	//let num_max = match num_max.trim().parse() {
	//	Ok(num) => num,
	//	Err(_) => 0,
	//};
	let num_max = 100_000_000;
	
	let t_start = SystemTime::now().duration_since(UNIX_EPOCH)
		.unwrap().as_millis();

	let num_max_sqr = (num_max as f64).sqrt() as usize;

	let mut vec_sieve: Vec<bool> = vec!(false; num_max);
	let mut vec_result: Vec<usize> = Vec::with_capacity(num_max);

	for i in 2..=num_max_sqr {
		if vec_sieve[i - 1] { continue; }

		vec_result.push(i);
		for b in (&mut vec_sieve[2*i - 1 .. num_max]).into_iter().step_by(i) {
			*b = true;
		}
	}

	for (i, &b) in vec_sieve[num_max_sqr..num_max].iter().enumerate() {
		if b == false {
			vec_result.push(num_max_sqr + 1 + i);
		}
	}

	let t_end = SystemTime::now().duration_since(UNIX_EPOCH)
		.unwrap().as_millis();

	println!("rust_prog: {} s", (t_end - t_start) as f64 / 1000.0);
	println!("{:?} .. {:?}",
		&vec_result[0..10],
		&vec_result[vec_result.len() - 10..]);
}
