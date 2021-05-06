use crate::format;

pub struct StatsUsize {
    values: Vec<usize>,
    calculated: bool,
    min: Option<usize>,
    max: Option<usize>,
    sum: Option<usize>,
    mean: Option<f32>,
    median: Option<f32>,
}

impl StatsUsize {
    pub fn new() -> Self {
        Self {
            values: vec![],
            calculated: false,
            min: None,
            max: None,
            sum: None,
            mean: None,
            median: None,
        }
    }

    pub fn new_filled(values: Vec<usize>) -> Self {
        let mut stats = Self::new();
        stats.values = values;
        stats.calc();
        stats
    }

    pub fn push(&mut self, value: usize) {
        self.values.push(value);
        self.calculated = false;
        self.min = None;
        self.max = None;
        self.sum = None;
        self.mean = None;
        self.median = None;
    }

    pub fn calc(&mut self) {
        let mut values = self.values.clone();
        let count = values.len();
        assert!(count > 0);
        self.min = Some(*values.iter().min().unwrap());
        self.max = Some(*values.iter().max().unwrap());
        let sum = values.iter().sum::<usize>();
        self.sum = Some(sum);
        self.mean = Some(sum as f32 / count as f32);

        values.sort();
        self.median = Some(if count % 2 == 0 {
            // Even number of items.
            let index_high = count / 2;
            let index_low = index_high - 1;
            (values[index_low] + values[index_high]) as f32 / 2.0
        } else {
            // Odd number of items.
            let index = (count - 1) / 2;
            values[index] as f32
        });

        self.calculated = true;
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn min(&self) -> usize {
        assert!(self.calculated, "calc() must be called before any calls to min(), max(), sum(), mean(), or median()");
        self.min.unwrap()
    }

    pub fn max(&self) -> usize {
        assert!(self.calculated, "calc() must be called before any calls to min(), max(), sum(), mean(), or median()");
        self.max.unwrap()
    }

    pub fn sum(&self) -> usize {
        assert!(self.calculated, "calc() must be called before any calls to min(), max(), sum(), mean(), or median()");
        self.sum.unwrap()
    }

    pub fn mean(&self) -> f32 {
        assert!(self.calculated, "calc() must be called before any calls to min(), max(), sum(), mean(), or median()");
        self.mean.unwrap()
    }

    pub fn median(&self) -> f32 {
        assert!(self.calculated, "calc() must be called before any calls to min(), max(), sum(), mean(), or median()");
        self.median.unwrap()
    }

    pub fn min_max_mean(&self, mean_precision: usize) -> String {
        format!("min = {}; max = {}; mean = {}",
            format::format_count(self.min()),
            format::format_count(self.max()),
            format::format_float(self.mean(), mean_precision))
    }

    pub fn min_max_mean_median(&self, mean_precision: usize, median_precision: usize) -> String {
        format!("{}; median = {}",
            self.min_max_mean(mean_precision),
            format::format_float(self.median(), median_precision))
    }
}

