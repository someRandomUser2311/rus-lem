pub trait Plugin {
    fn name(&self) -> String;
    fn run(&self, input: &str) -> String;
    fn create() -> Self
    where
        Self: Sized;
}

pub struct Upper;

pub struct Reverse;

pub struct Trim;

impl Plugin for Upper {
    fn name(&self) -> String {
        String::from("upper")
    }

    fn run(&self, input: &str) -> String {
        input.to_uppercase()
    }

    fn create() -> Self {
        Upper
    }
}

impl Plugin for Reverse {
    fn name(&self) -> String {
        String::from("reverse")
    }

    fn run(&self, input: &str) -> String {
        input.chars().rev().collect()
    }

    fn create() -> Self {
        Reverse
    }
}

impl Plugin for Trim {
    fn name(&self) -> String {
        String::from("trim")
    }

    fn run(&self, input: &str) -> String {
        input.trim().to_string()
    }

    fn create() -> Self {
        Trim
    }
}

pub fn make(kind: &str) -> Option<Box<dyn Plugin>> {
    match kind {
        "upper" => Some(Box::new(Upper::create())),
        "reverse" => Some(Box::new(Reverse::create())),
        "trim" => Some(Box::new(Trim::create())),
        _ => None,
    }
}

pub fn build<T: Plugin + 'static>() -> Box<dyn Plugin> {
    Box::new(T::create())
}

pub fn apply_all(plugins: &[Box<dyn Plugin>], input: &str) -> String {
    plugins
        .iter()
        .fold(input.to_string(), |acc, plugin| plugin.run(&acc))
}

pub fn chain(kinds: &[&str], input: &str) -> Result<String, String> {
    let mut plugins = Vec::new();
    for kind in kinds {
        if let Some(p) = make(kind) {
            plugins.push(p);
        }
    }
    Ok(apply_all(&plugins, input))
}
