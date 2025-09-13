pub fn stars(n: u32) -> String {
    let nmb_of_stars = 2_i32.pow(n);
    "*".repeat(nmb_of_stars as usize)

}