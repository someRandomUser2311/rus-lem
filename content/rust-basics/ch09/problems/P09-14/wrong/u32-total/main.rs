use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut total: u32 = 0;
    for line in io::stdin().lines() {
        let line = line?;
        let n: u32 = line.trim().parse()?;
        total += n;
    }
    println!("{total}");
    Ok(())
}
