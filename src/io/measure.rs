use std::{collections::HashMap, time::Instant};

const ENABLED: bool = true;

pub struct Measurer {
    results: HashMap<&'static str, usize>,
}

impl Measurer {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    #[inline]
    pub fn measure<F, R>(&mut self, stage: &'static str, mut func: F) -> R
    where
        F: FnMut() -> R,
    {
        if !ENABLED {
            func()
        } else {
            let start = Instant::now();
            let result = func();
            let sum = start.elapsed().as_millis() as usize + self.results.get(stage).unwrap_or(&0);
            self.results.insert(stage, sum);
            result
        }
    }

    pub fn dump(&self) {
        if ENABLED {
            for (stage, time) in &self.results {
                crate::info!("Total stage time: {stage} = {time}ms");
            }
        }
    }
}
