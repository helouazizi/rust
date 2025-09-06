pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center:Point ,
	pub radius:f64
}

impl Circle {
    pub  fn new(x:f64,y:f64,r:f64) ->Self {
        Self {
            center : Point(x,y),
            radius : r
        }
    }

    pub fn area(&self) -> f64 {
      std::f64::consts::PI * self.radius * self.radius 
    }

    pub fn diameter(&self) -> f64{
        self.radius * 2.0
    }

    pub fn intersect(&self, other_circle : Circle) -> bool {
        let dis = self.center.distance(other_circle.center);
        dis <= (self.radius + other_circle.radius)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64 ,  pub f64);

impl Point {
   pub  fn distance(&self, other_point :Point) -> f64 {
        let dx = other_point.0 - self.0 ;
        let dy =  other_point.1 - self.1;
        (dx*dx + dy*dy).sqrt()
    }
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
