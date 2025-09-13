pub fn get_diamond(c: char) -> Vec<String> {
    let arr: [char; 26] = ('A'..='Z').collect::<Vec<_>>().try_into().unwrap();
    let index = arr.iter().position(|x| *x == c).unwrap();
    let size = index * 2 + 1;
    let mut res: Vec<String> = vec![String::new(); size];

    for i in 0..=index {
        let outer_space = index - i;
        let inner_space = if i == 0 { 0 } else { i * 2 - 1 };
        let ch = arr[i];

        let mut line = String::new();
        line.push_str(&" ".repeat(outer_space));
        line.push(ch);
        if inner_space > 0 {
            line.push_str(&" ".repeat(inner_space));
            line.push(ch);
        }
        line.push_str(&" ".repeat(outer_space));

        // Assign to both the top and bottom rows
        res[i] = line.clone();
        res[size - 1 - i] = line;
    }

    res
}