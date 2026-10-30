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

/// LM8's generated suite (seed 0x08020000): case `case` of 300, a fresh catalogue of 3–7
/// books and three members driven by 20 drawn operations. Some draws depend on the
/// catalogue's state (which book is out), and the stream runs across cases, so this resumes
/// where the previous case stopped (or replays from the start when called out of order).
/// Returns one line per step: the result, then every book's and member's full record.
fn lm8_case(case: u32) -> String {
    use crate::library::Library;
    use std::cell::Cell;
    thread_local! {
        static AT: Cell<(u32, u32)> = const { Cell::new((0, 0x08020000)) };
    }
    struct G(u32);
    impl G {
        fn pick(&mut self, k: u32) -> usize {
            self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
            ((self.0 >> 16) % k) as usize
        }
    }
    const T: [&str; 6] = ["The Cat", "cat", "A Dog", "Dog", "an Owl", "東京"];
    const A: [&str; 5] = ["Ann", " ann ", "ANN", "Bob", "Émile"];
    const QS: [&str; 4] = ["", "CAT", "dog", "x"];
    const P: [u32; 4] = [0, 1, 2, 4294967295];
    const MQ: [u32; 6] = [0, 0, 0, 1, 4294967295, 3];
    const Q: [u32; 5] = [0, 1, 2, 4294967295, 3];
    const OUT: [u32; 4] = [0, 7, 100, 4294967267];
    const RET: [u32; 14] = [0, 14, 15, 21, 22, 28, 29, 35, 36, 57, 58, 71, 72, 4294967295];
    let (mut next, mut state) = AT.with(|a| a.get());
    if next > case {
        next = 0;
        state = 0x08020000;
    }
    let mut r = G(state);
    let mut log = String::new();
    while next <= case {
        log.clear();
        let mut l = Library::new();
        let mut n: u32 = 0;
        let nb = 3 + r.pick(5);
        for _ in 0..nb {
            let t = T[r.pick(6)];
            let a = A[r.pick(5)];
            l.add_book(t, a, 0);
            n += 1;
        }
        let mut setup = Vec::new();
        for j in 0..3 {
            let id = if j == 0 { 0 } else { P[r.pick(4)] };
            let prem = r.pick(2) == 1;
            setup.push(format!("add_member({id}, {prem}) = {}", l.add_member(id, "M", prem)));
        }
        log.push_str(&format!("setup: {}\n", setup.join(", ")));
        for step in 0..20 {
            let res = match r.pick(6) {
                0 => {
                    let t = T[r.pick(6)];
                    let a = A[r.pick(5)];
                    let id = l.add_book(t, a, 0);
                    n += 1;
                    format!("add_book({t:?}, {a:?}) = {id}")
                }
                1..=3 => {
                    let b = if r.pick(4) != 0 {
                        1 + r.pick(n) as u32
                    } else {
                        [0, n + 1][r.pick(2)]
                    };
                    let m = MQ[r.pick(6)];
                    let d = OUT[r.pick(4)];
                    format!("checkout({b}, {m}, {d}) = {}", l.checkout(b, m, d))
                }
                4 => {
                    let mut first = None;
                    for id in 1..=n {
                        if first.is_none() && l.book(id).is_some_and(|b| b.borrower().is_some()) {
                            first = Some(id);
                        }
                    }
                    let b = match (r.pick(3) != 0, first) {
                        (true, Some(id)) => id,
                        _ => r.pick(n + 2) as u32,
                    };
                    let d = RET[r.pick(14)];
                    format!("return_book({b}, {d}) = {:?}", l.return_book(b, d))
                }
                _ => {
                    let a = A[r.pick(5)];
                    let q = QS[r.pick(4)];
                    let mut by = Vec::new();
                    for b in l.books_by(a) {
                        by.push(b.id);
                    }
                    format!(
                        "books_by({a:?}) = {by:?}; search_titles({q:?}) = {:?}; authors() = {:?}",
                        l.search_titles(q),
                        l.authors()
                    )
                }
            };
            let mut state = format!("{:?}", l.count_by_status());
            for id in 0..n + 2 {
                state.push_str(&format!(" | book {id}: {:?}", l.book(id)));
            }
            for id in Q {
                state.push_str(&format!(" | member {id}: {:?}", l.member(id)));
            }
            log.push_str(&format!("{step}: {res} || {state}\n"));
        }
        next += 1;
    }
    AT.with(|a| a.set((next, r.0)));
    log
}
