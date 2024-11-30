pub fn nearest<T: Copy>(
	orig: &[T],
	samples: usize,
	orig_width: u32,
	orig_height: u32,
	new_width: u32,
	new_height: u32,
) -> Vec<T> {
	let new_size = new_width as usize * new_height as usize;
	let mut new = vec![orig[0]; new_size * samples];

	nearest_buffer(
		orig,
		samples,
		orig_width,
		orig_height,
		&mut new,
		new_width,
		new_height,
	);

	new
}

pub fn nearest_buffer<T: Copy>(
	orig: &[T],
	samples: usize,
	orig_width: u32,
	orig_height: u32,
	new: &mut [T],
	new_width: u32,
	new_height: u32,
) {
	let new_size = new_width as usize * new_height as usize;

	for index in 0..new_size {
		let x = index % new_width as usize;
		let y = index / new_width as usize;
		let index = index * samples;

		let orig_x = (orig_width as usize * x) / new_width as usize;
		let orig_y = (orig_height as usize * y) / new_height as usize;
		let orig_index = (orig_y * orig_width as usize + orig_x) * samples;

		unsafe {
			let orig = orig[orig_index..orig_index + samples].as_ptr();
			let new = new[index..index + samples].as_mut_ptr();

			std::ptr::copy_nonoverlapping(orig, new, samples);
		}
	}
}
