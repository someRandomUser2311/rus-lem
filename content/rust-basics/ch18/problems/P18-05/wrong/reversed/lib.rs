pub trait Stage {
    fn name(&self) -> String;
    fn apply(&mut self, value: i64) -> i64;
}

pub struct Add(pub i64);

pub struct Scale(pub i64);

pub struct Counter {
    pub calls: i64,
}

impl Stage for Add {
    fn name(&self) -> String {
        format!("add {}", self.0)
    }

    fn apply(&mut self, value: i64) -> i64 {
        value.saturating_add(self.0)
    }
}

impl Stage for Scale {
    fn name(&self) -> String {
        format!("scale {}", self.0)
    }

    fn apply(&mut self, value: i64) -> i64 {
        value.saturating_mul(self.0)
    }
}

impl Stage for Counter {
    fn name(&self) -> String {
        String::from("counter")
    }

    fn apply(&mut self, value: i64) -> i64 {
        self.calls += 1;
        value.saturating_add(self.calls)
    }
}

pub fn pipeline(stages: &mut [Box<dyn Stage>], input: i64) -> i64 {
    let mut value = input;
    for stage in stages.iter_mut().rev() {
        value = stage.apply(value);
    }
    value
}

pub fn run_all(stages: &mut [Box<dyn Stage>], inputs: &[i64]) -> Vec<i64> {
    inputs.iter().map(|&i| pipeline(stages, i)).collect()
}

pub fn describe(stages: &[Box<dyn Stage>]) -> String {
    stages
        .iter()
        .map(|s| s.name())
        .collect::<Vec<_>>()
        .join(" -> ")
}
