use std::io;
fn main() {
    let mut index_text = String::new();
    io::stdin()
        .read_line(&mut index_text)
        .expect("Failed to read line");
    match index_text.trim().parse::<usize>() {
        Ok(index) => {
            let days = [31, 28, 31, 30, 31];
            let i: usize = index;
            let mut found = 0;
            let mut n = 0;
            for d in days {
                if n == i {
                    found = d;
                }
                n += 1;
            }
            if i >= 5 {
                panic!("index out of bounds: the len is 5 but the index is {i}");
            }
            println!("{found}");
        }
        Err(_) => println!("Input error"),
    }
}
