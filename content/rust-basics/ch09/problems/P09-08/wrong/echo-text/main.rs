use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let _number: u32 = line.trim().parse()?;
    println!("{}", line.trim());
    Ok(())
}
