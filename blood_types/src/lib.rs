#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            "AB" => Ok(Antigen::AB),
            _ => Err("Invalid antigen".to_string()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err("Invalid Rh factor".to_string()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then_with(|| self.rh_factor.cmp(&other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigon, rh) = if s.ends_with("+") {
            (&s[..s.len() - 1], "+")
        } else if s.ends_with("-") {
            (&s[..s.len() - 1], "-")
        } else {
            return Err("Invalid Blood Type".to_string());
        };

        let antigen = antigon.parse()?;
        let rh = rh.parse()?;
        Ok(BloodType {
            antigen: antigen,
            rh_factor: rh,
        })
    }
}

use std::fmt::{self, Debug};



impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}


impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Positive {
            return false;
        }
        match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let all = Self::all();
        all.into_iter()
            .filter(|d| self.can_receive_from(d))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
    
		        let all = Self::all();
        all.into_iter().filter(|r| r.can_receive_from(self)).collect()
    }
    pub fn all() -> Vec<Self> {
        let antigens = [Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rh = [RhFactor::Negative, RhFactor::Positive];
        antigens
            .iter()
            .flat_map(|a| rh.iter().map(move |r| BloodType {
                antigen: a.clone(),
                rh_factor: r.clone(),
            }))
            .collect()
    }
	
}
