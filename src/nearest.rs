pub fn nearest(
    orig: &[u8],
    orig_width: u32,
    orig_height: u32,
    new_width: u32,
    new_height: u32,
) -> Vec<u8> {
    let new_size = new_width as usize * new_height as usize;
    let mut new = vec![0; new_size * 4];

    let old_per_new_w = orig_width as f32 / new_width as f32;
    let old_per_new_h = orig_height as f32 / new_height as f32;

    for index in 0..new_size {
        let x = index % new_width as usize;
        let y = index / new_width as usize;

        let old_x = (old_per_new_w * x as f32).floor() as usize;
        let old_y = (old_per_new_h * y as f32).floor() as usize;
        let old_index = (old_y * orig_width as usize + old_x) * 4;

        new[index * 4] = orig[old_index];
        new[index * 4 + 1] = orig[old_index + 1];
        new[index * 4 + 2] = orig[old_index + 2];
        new[index * 4 + 3] = orig[old_index + 3];
    }

    new
}
