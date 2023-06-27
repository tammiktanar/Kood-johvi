use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use crate::Float;

#[derive(Debug, Clone)]
pub struct Reservoir<T, R> {
    current: Option<(T, Float)>,
    total_weight: Float,
    items_seen: usize,

    rng: R,
}

impl<T, R> Reservoir<T, R>
where
    R: Rng,
{
    pub fn new(rng: R) -> Self {
        Self {
            current: None,
            total_weight: 0.0,
            items_seen: 0,
            rng,
        }
    }

    pub fn insert(&mut self, item: T, weight: Float) {
        assert!(weight >= 0.0, "got weight of {weight}");
        self.insert_inner(Some((item, weight)), weight)
    }

    fn insert_inner(&mut self, item: Option<(T, Float)>, weight: Float) {
        if self
            .rng
            .gen_bool((weight / (self.total_weight + weight)) as f64)
        {
            self.current = item
        }

        self.total_weight += weight;
        self.items_seen += 1;
    }

    pub fn item(&self) -> Option<&T> {
        self.current.as_ref().map(|(item, _)| item)
    }

    pub fn weight(&self) -> Option<Float> {
        self.current.as_ref().map(|(_, weight)| *weight)
    }

    pub fn total_weight(&self) -> Float {
        self.total_weight
    }

    pub fn items_seen(&self) -> usize {
        self.items_seen
    }

    pub fn into_result(self) -> Option<(T, Float)> {
        self.current
    }

    pub fn combine(mut self, other: Self) -> Self {
        self.insert_inner(other.current, other.total_weight);
        self
    }
}

impl<T> Default for Reservoir<T, SmallRng> {
    fn default() -> Self {
        Self::new(SmallRng::from_entropy())
    }
}
