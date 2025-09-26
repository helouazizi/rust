#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().copied().max()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut nums: Vec<u32> = self.numbers.to_vec();
        nums.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending
        nums.into_iter().take(3).collect()
    }
}
