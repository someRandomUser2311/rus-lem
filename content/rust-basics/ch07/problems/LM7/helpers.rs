/// Starts from `Member { id: 7, name: " unchanged ", premium, borrowed }` and runs each
/// operation (`b` record_borrow, `r` record_return), logging its result and the count.
fn drive(premium: bool, borrowed: u8, ops: &str) -> String {
    use crate::member::Member;
    let mut m = Member {
        id: 7,
        name: " unchanged ".to_string(),
        premium,
        borrowed,
    };
    let mut log = String::new();
    for op in ops.chars() {
        let result = if op == 'b' {
            format!("{:?}", m.record_borrow())
        } else {
            m.record_return();
            "()".to_string()
        };
        log.push_str(&format!("{op} -> {result} {} {}\n", m.borrowed, m.limit()));
    }
    log.push_str(&format!("{:?}", m));
    log
}

/// One case of LM6's generated suite (case `i`, three draws): the operation's result,
/// then the complete Book and Member afterwards.
fn lm6_case(i: u32, member_id: u32, due: u32, raw_day: u32) -> String {
    use crate::{book::{Book, Status}, member::Member};
    let mode = i % 10;
    let premium = (i / 10) % 2 == 1;
    let limit: u8 = if premium { 6 } else { 3 };
    let mut b = Book::new(1, "x", "y", 1);
    let mut m = Member::new(member_id, "A", premium);
    let today = raw_day % 4294967268;
    let result = match mode {
        0..=3 => {
            m.borrowed = [0, limit, 255, limit - 1][mode as usize];
            format!("{:?}", b.checkout(&mut m, today))
        }
        4 | 5 => {
            b.status = if mode == 4 {
                Status::Lost
            } else {
                Status::Borrowed { member_id, due_day: due }
            };
            format!("{:?}", b.checkout(&mut m, today))
        }
        _ => {
            let day = [99, 100, 101, 107, 108, 143, 144, 4294967295][((i / 10) % 8) as usize];
            match mode {
                6 => {
                    b.status = Status::Borrowed { member_id, due_day: 100 };
                    m.borrowed = 1;
                }
                7 => {
                    b.status = Status::Borrowed { member_id: member_id ^ 1, due_day: 100 };
                    m.borrowed = 1;
                }
                8 => m.borrowed = 0,
                _ => {
                    b.status = Status::Borrowed { member_id, due_day: 100 };
                    m.borrowed = if (i / 10) % 2 == 0 { 0 } else { 255 };
                }
            }
            format!("{:?}", b.return_to_shelf(&mut m, day))
        }
    };
    format!("{result} {b:?} {m:?}")
}
