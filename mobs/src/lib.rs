// src/lib.rs
pub mod mobs;


#[cfg(test)]
mod tests {
    use super::mobs::{Mob, Boss};

    #[test]
    fn create_mob() {
        let boss = Boss::new("Tony", 45);
        let mut mob = Mob {
            name: "The Shadows".into(),
            boss,
            members: Default::default(),
            cities: Default::default(),
            wealth: 1000,
        };

        mob.recruit(("Joe", 30));
        assert_eq!(mob.members.len(), 1);
    }
}
