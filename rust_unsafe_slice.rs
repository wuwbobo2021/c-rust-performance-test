use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
	let num_max = 100_000_000;
	
	let t_start = SystemTime::now().duration_since(UNIX_EPOCH)
		.unwrap().as_millis();

	let num_max_sqr = (num_max as f64).sqrt() as usize;

	let mut vec_sieve: Vec<bool> = vec!(false; num_max);
	let mut vec_result: Vec<usize> = Vec::with_capacity(num_max);

	unsafe {
		let s_sieve = &mut vec_sieve;
		let s_result = &mut vec_result;
		let mut cnt_result = 0;

		for i in 2..=num_max_sqr {
			if *s_sieve.get_unchecked(i - 1) { continue; }

			*s_result.get_unchecked_mut(cnt_result) = i;
			cnt_result += 1;
			for j in (2*i - 1 .. num_max).step_by(i) {
				*s_sieve.get_unchecked_mut(j) = true;
			}
		}

		for i in num_max_sqr + 1 ..= num_max {
			if ! *s_sieve.get_unchecked(i - 1) {
				*s_result.get_unchecked_mut(cnt_result) = i;
				cnt_result += 1;
			}
		}

		vec_result.set_len(cnt_result);
	}

	let t_end = SystemTime::now().duration_since(UNIX_EPOCH)
		.unwrap().as_millis();

	println!("rust_unsafe_slice: {} s", (t_end - t_start) as f64 / 1000.0);
	println!("{:?} .. {:?}",
		&vec_result[0..10],
		&vec_result[vec_result.len() - 10..]);
}
