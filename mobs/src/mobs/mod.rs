pub mod boss;
pub mod member;
pub use boss::*;
pub use member::*;

use std::collections::{HashMap, HashSet};


#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        let member = Member {
            role: Role::Associate,
            age,
        };
        self.members.insert(name.to_string(), member);
    }

    fn power_score(&self) -> u32 {
        self.members
            .values()
            .map(|m| match m.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            })
            .sum()
    }

    fn remove_youngest_members(&mut self) {
        if self.members.is_empty() {
            return;
        }
        let youngest_age = self.members.values().map(|m| m.age).min().unwrap();
        let to_remove: Vec<String> = self
            .members
            .iter()
            .filter_map(|(k, m)| if m.age == youngest_age { Some(k.clone()) } else { None })
            .collect();

        for key in to_remove {
            self.members.remove(&key);
        }
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_score = self.power_score();
        let other_score = other.power_score();

        // If draw, attacker (self) loses
        let (loser, winner) = if self_score < other_score {
            (self, other)
        } else if self_score > other_score {
            (other, self)
        } else {
            (self, other)
        };

        loser.remove_youngest_members();

        if loser.members.is_empty() {
            for city in loser.cities.drain() {
                winner.cities.insert(city);
            }
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let steal_amount = amount.min(target.wealth);
        target.wealth -= steal_amount;
        self.wealth += steal_amount;
    }

    pub fn conquer_city(&mut self, others: &[&Mob], city: String) {
        let city_taken = others.iter().any(|mob| mob.cities.contains(&city));
        if !city_taken {
            self.cities.insert(city);
        }
    }
}
