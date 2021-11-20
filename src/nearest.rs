pub fn nearest(
    orig: &[u8],
    orig_width: u32,
    orig_height: u32,
    new_width: u32,
    new_height: u32,
) -> Vec<u8> {
    let new_size = new_width as usize * new_height as usize;
    let mut new = vec![0; new_size * 4];

    for index in 0..new_size {
        let x = index % new_width as usize;
        let y = index / new_width as usize;
        let index = index * 4;

        let orig_x = (orig_width as usize * x) / new_width as usize;
        let orig_y = (orig_height as usize * y) / new_height as usize;
        let orig_index = (orig_y * orig_width as usize + orig_x) * 4;

        new[index] = orig[orig_index];
        new[index + 1] = orig[orig_index + 1];
        new[index + 2] = orig[orig_index + 2];
        new[index + 3] = orig[orig_index + 3];
    }

    new
}
