pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub struct Student(u32,String,String);


pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
