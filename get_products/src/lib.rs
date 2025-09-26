pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
        if arr.len() == 1 {
            return  Vec::new();
        }
        arr.iter()
        .map(|&num| arr.iter().filter(|&&x| x != num).product())
        .collect()
}