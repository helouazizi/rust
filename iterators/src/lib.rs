// #[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {

     type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None;
        }
        
        let current = self.v;
        self.v = if self.v % 2 == 0 {
            self.v / 2
        } else {
            self.v * 3 + 1
        };

        Some(current)
    }
    


}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n <= 1 {
        return  0
    }
    if n % 2 == 0 {
       
       return 1 + collatz(n/2)
    }else {
       return   1 + collatz((n*3) +1 )
    }
}