use std::ops::{Add, AddAssign};

pub struct StatsNumberList<T>
    where T: Copy + Ord + num_traits::Zero + Add<Output = T> + AddAssign + Into<f64>
{
    pub values: Vec<T>,
    pub min: T,
    pub max: T,
    pub sum: T,
    pub mean: f64,
    pub median: f64,
}

impl <T> StatsNumberList<T>
    where T: Copy + Ord + num_traits::Zero + Add<Output = T> + AddAssign + Into<f64>
{
    pub fn new(values: &[T]) -> Self {
        let mut values = values.to_vec();
        values.sort();

        let count = values.len();
        assert!(count > 0);

        let min = values[0];
        let max = *values.last().unwrap();

        let mut sum = T::zero();
        for value in values.iter() {
            sum += *value;
        }

        let mean = Self::generic_to_f64(sum) / count as f64;

        let median = if count % 2 == 0 {
            // Even number of items.
            let index_high = count / 2;
            let index_low = index_high - 1;
            Self::generic_to_f64(values[index_low] + values[index_high]) / 2.0
        } else {
            // Odd number of items.
            let index = (count - 1) / 2;
            Self::generic_to_f64(values[index])
        };

        Self {
            values,
            min,
            max,
            sum,
            mean,
            median,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    fn generic_to_f64(value: T) -> f64 {
        value.into()
    }
}

