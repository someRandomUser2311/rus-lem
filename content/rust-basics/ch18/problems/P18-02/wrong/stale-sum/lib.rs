pub struct AveragedCollection {
    list: Vec<i32>,
    sum: i64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            sum: 0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.sum += value as i64;
    }

    pub fn remove(&mut self) -> Option<i32> {
        let value = self.list.pop()?;
        Some(value)
    }

    pub fn average(&self) -> Option<f64> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.sum as f64 / self.list.len() as f64)
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}
