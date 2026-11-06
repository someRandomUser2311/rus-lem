pub type Step = Box<dyn Fn(i64) -> Option<i64>>;

pub struct Pipeline {
    steps: Vec<(String, Step)>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline { steps: Vec::new() }
    }

    pub fn add(&mut self, name: &str, step: impl Fn(i64) -> Option<i64> + 'static) -> &mut Self {
        self.steps.push((name.to_string(), Box::new(step)));
        self
    }

    pub fn add_fn(&mut self, name: &str, f: fn(i64) -> Option<i64>) -> &mut Self {
        self.add(name, f)
    }

    pub fn names(&self) -> Vec<String> {
        self.steps.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn run(&self, input: i64) -> Result<i64, String> {
        let mut value = input;
        for (name, step) in &self.steps {
            value = step(value).ok_or(format!("step {name} failed at {value}"))?;
        }
        Ok(value)
    }
}

pub fn double(x: i64) -> Option<i64> {
    x.checked_mul(2)
}

pub fn make_limit(max: i64) -> impl Fn(i64) -> Option<i64> {
    move |x| if x < max { Some(x) } else { None }
}
