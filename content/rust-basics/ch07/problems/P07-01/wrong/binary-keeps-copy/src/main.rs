fn greeting(name: &str) -> String {
    format!("Ready, {name}!")
}

fn main() {
    println!("{}", greeting("reader"));
}
