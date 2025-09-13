
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}


impl Color {


    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut first_field = None;
        let mut second_field = None;

        if self.r == first {
            first_field = Some("r");
        } else if self.g == first {
            first_field = Some("g");
        } else if self.b == first {
            first_field = Some("b");
        } else if self.a == first {
            first_field = Some("a");
        }

        if self.r == second {
            second_field = Some("r");
        } else if self.g == second {
            second_field = Some("g");
        } else if self.b == second {
            second_field = Some("b");
        } else if self.a == second {
            second_field = Some("a");
        }

        if first_field.is_none() || second_field.is_none() {
            return self;
        }

        match (first_field.unwrap(), second_field.unwrap()) {
            ("r", "g") | ("g", "r") => std::mem::swap(&mut self.r, &mut self.g),
            ("r", "b") | ("b", "r") => std::mem::swap(&mut self.r, &mut self.b),
            ("r", "a") | ("a", "r") => std::mem::swap(&mut self.r, &mut self.a),
            ("g", "b") | ("b", "g") => std::mem::swap(&mut self.g, &mut self.b),
            ("g", "a") | ("a", "g") => std::mem::swap(&mut self.g, &mut self.a),
            ("b", "a") | ("a", "b") => std::mem::swap(&mut self.b, &mut self.a),
            _ => {}
        }

        self
    }
}
