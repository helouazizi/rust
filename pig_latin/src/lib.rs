pub fn pig_latin(text: &str) -> String {
    let vowels = ['e', 'i', 'u', 'o', 'a'];
    let mut res: String = String::new();

    for (i, c) in text.chars().enumerate() {
        // println!("{}{}", c, i);
        // check if firs c is vowe

        if i == 0 && vowels.contains(&c) {
            res.push_str(text);
            res.push_str("ay");
            return res; 
        }

        

        // check if the tetx start with vowes folowwed by qu

        if i == 0 && !vowels.contains(&c) && text[1..3] == *"qu" {
            res.push_str(&text[3..text.len()]);
            res.push_str(&text[0..3]);
            res.push_str("ay");
             return res; 
        }
        // check if the twxt stars with simple letters intul it found the vowel
        if !vowels.contains(&c) && vowels.contains(&text.chars().nth(i+1).unwrap())  {
            res.push_str(&text[i+1..]);
            res.push_str(&text[0..i+1]);
                res.push_str("ay");
            return res; 

        }

    }

    res
}
