use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Cannot convert"),
        }
    }
}


impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        
        if value == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut res = Vec::new();

        let mapping: &[(u32, &[RomanDigit])] = &[
            (1000, &[M]),
            (900, &[C, M]),
            (500, &[D]),
            (400, &[C, D]),
            (100, &[C]),
            (90, &[X, C]),
            (50, &[L]),
            (40, &[X, L]),
            (10, &[X]),
            (9, &[I, X]),
            (5, &[V]),
            (4, &[I, V]),
            (1, &[I]),
        ];

        for &(num, digits) in mapping {
            while value >= num {
                res.extend_from_slice(digits);
                value -= num;
            }
        }

        RomanNumber(res)
    }
}
impl Iterator for RomanNumber {
    type Item = RomanNumber;
    fn next(&mut self) -> Option<Self::Item> {
      let res =  decipher(self) ;

      Some(Self::from(res+1))

        
    }
}
pub fn decipher(roman: &RomanNumber) -> u32 {
    let values = |d: RomanDigit| -> u32 {
        match d {
            RomanDigit::Nulla => 0,
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
        }
    };

    let mut total = 0;
    let digits = &roman.0;
    let len = digits.len();

    let mut i = 0;
    while i < len {
        let curr = values(digits[i]);
        let next = if i + 1 < len {
            values(digits[i + 1])
        } else {
            0
        };

        if curr < next {
            total += next - curr;
            i += 2;
        } else {
            total += curr;
            i += 1;
        }
    }

    total
}