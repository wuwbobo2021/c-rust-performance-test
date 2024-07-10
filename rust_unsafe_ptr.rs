use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
	let num_max = 100_000_000;
	
	let t_start = SystemTime::now().duration_since(UNIX_EPOCH)
		.unwrap().as_millis();

	let num_max_sqr = (num_max as f64).sqrt() as usize;

	let mut vec_sieve: Vec<bool> = vec!(false; num_max);
	let mut vec_result: Vec<usize> = Vec::with_capacity(num_max);

	unsafe {
		let p_sieve = vec_sieve.as_mut_ptr();
		let mut p_result = vec_result.as_mut_ptr();

		let mut p1 = p_sieve.add(1);
		for i in 2..=num_max_sqr {
			if *p1 == false {
				*p_result = i; p_result = p_result.add(1);

				let mut p2 = p_sieve.add(2*i - 1);
				for _ in (2*i ..= num_max).step_by(i) {
					*p2 = true; p2 = p2.add(i);
				}
			}
			p1 = p1.add(1);
		}

		p1 = p_sieve.add(num_max_sqr);
		for i in num_max_sqr + 1 ..= num_max {
			if *p1 == false {
				*p_result = i; p_result = p_result.add(1);
			}
			p1 = p1.add(1);
		}

		vec_result.set_len(p_result.offset_from(vec_result.as_ptr()) as usize);
	}

	let t_end = SystemTime::now().duration_since(UNIX_EPOCH)
		.unwrap().as_millis();

	println!("rust_unsafe_ptr: {} s", (t_end - t_start) as f64 / 1000.0);
	println!("{:?} .. {:?}",
		&vec_result[0..10],
		&vec_result[vec_result.len() - 10..]);
}
