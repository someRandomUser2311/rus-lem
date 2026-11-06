pub trait Pilot {
    fn fly(&self) -> String;
}

pub trait Wizard {
    fn fly(&self) -> String;
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    pub fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn all_flights(person: &Human) -> Vec<String> {
    vec![Pilot::fly(person), Wizard::fly(person), person.fly()]
}

pub fn baby_names() -> (String, String) {
    (<Dog as Animal>::baby_name(), Dog::baby_name())
}
