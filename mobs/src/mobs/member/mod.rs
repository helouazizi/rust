#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Member {
    pub role: Role,
    pub age: u32,
}

impl Member {
    pub fn get_promotion(&mut self) {
        use Role::*;
        self.role = match self.role {
            Associate => Soldier,
            Soldier => Caporegime,
            Caporegime => Underboss,
            Underboss => panic!("Underboss cannot be promoted further!"),
        };
    }
}
