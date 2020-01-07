use std::collections::HashMap;

pub struct Cacher<A, R, T>
where
    T: Fn(A) -> R,
{
    calculation: T,
    results: HashMap<A, R>,
}

impl<A, R, T> Cacher<A, R, T>
where
    A: std::cmp::Eq,
    A: std::hash::Hash,
    A: Copy,
    R: Copy,
    T: Fn(A) -> R,
{
    pub fn new(calculation: T) -> Cacher<A, R, T> {
        Cacher {
            calculation,
            results: HashMap::new(),
        }
    }

    pub fn value(self: &mut Cacher<A, R, T>, arg: A) -> R {
        let calculation = &mut self.calculation;
        *self
            .results
            .entry(arg)
            .or_insert_with(|| (calculation)(arg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }
}
