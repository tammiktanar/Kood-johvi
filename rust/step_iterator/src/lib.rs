use std::ops::Add;

pub trait Scalar: Add<Output=Self> + PartialOrd + Copy {}

impl<T> Scalar for T
where T: Add<Output=Self> + PartialOrd + Copy {}

pub struct StepIterator<T: Scalar> {
    current: T,
    end: T,
    step: T,
}

impl<T: Scalar> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        assert!(beg <= end);

        Self {
            current: beg,
            end,
            step,
        }
    }
}

impl<T: Scalar> Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }

        let ret = self.current;

        self.current = self.current + self.step;

        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for v in StepIterator::new(0, 100, 10) {
            print!("{},", v);
        }
        println!();

        for v in StepIterator::new(0, 100, 12) {
            print!("{},", v)
        }
        println!();
    }
}
