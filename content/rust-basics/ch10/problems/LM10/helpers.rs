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

/// The course generator with high-bit picks: pick(k) = (next() >> 16) % k.
struct Lm9Rng(u32);
impl Lm9Rng {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
        self.0
    }
    fn hp(&mut self, k: u32) -> u32 {
        (self.next() >> 16) % k
    }
}
fn lm9_hx(s: &str) -> String {
    let mut o = String::new();
    for b in s.bytes() {
        o.push_str(&format!("{b:02x}"));
    }
    o
}
fn lm9_er(e: &crate::LibraryError) -> String {
    match e {
        crate::LibraryError::BadCommand(s) => format!("Bad:{}", lm9_hx(s)),
        other => format!("{other:?}"),
    }
}
fn lm9_cm(c: &crate::Command) -> String {
    match c {
        crate::Command::Add { title, author, year } => format!("Add:{}:{}:{year}", lm9_hx(title), lm9_hx(author)),
        crate::Command::Checkout { book_id, member_id, today } => format!("Co:{book_id}:{member_id}:{today}"),
        crate::Command::Return { book_id, today } => format!("Re:{book_id}:{today}"),
        crate::Command::Find(q) => format!("Find:{}", lm9_hx(q)),
        crate::Command::Report => "Report".to_string(),
    }
}
const LM9_T: [&str; 4] = ["Rust", "rust book", "東京", "Dune"];
const LM9_P: [u32; 4] = [1, 2, 1, 4294967295];
const LM9_AT: [&str; 4] = ["Rust", " The  Hobbit ", "", "東京"];
const LM9_Y: [&str; 5] = ["1937", "+7", "65535", "65536", "x"];
const LM9_BADADD: [&str; 4] = ["add Rust|Ann", "add a|b|1|2", "add", "Add Rust|Ann|1"];
const LM9_MQ: [u32; 5] = [1, 1, 1, 4294967295, 3];
const LM9_D: [&str; 5] = ["0", "10", "4294967267", "4294967268", "+5"];
const LM9_R: [u32; 8] = [0, 14, 15, 21, 22, 57, 58, 4294967295];
const LM9_FIND: [&str; 5] = ["find rust", "find  RUST  book ", "find 東京", "find zzz", "find"];
const LM9_REP: [&str; 3] = ["report", " report\t", "report now"];
const LM9_BAD: [&str; 8] = ["", "   ", "checkout 1 2", "return 1", "FIND rust", "checkout 1 1 1 1", "return 1 x", "remove 1"];
const LM9_NL: [&str; 3] = ["find a\nb", "report\r\nreport", "checkout 1 1 5\n"];
const LM9_MS: [u32; 4] = [1, 2, 4294967295, 3];
/// LM9's generated suite (seed 0x09030000): case `want` of 300, replayed from the start
/// because the draws run across cases. Returns the setup line, then one line per step:
/// the parse and execute results, then the counts, every book and four members.
fn lm9_case(want: u32) -> String {
    use crate::{Library, Status, parse_command};
    let mut r = Lm9Rng(0x09030000);
    let mut log = String::new();
    for case in 0..=want {
        let mut l = Library::new();
        let mut n: u32 = 0;
        let nb = 4 + r.hp(5);
        for _ in 0..nb {
            l.add_book(LM9_T[r.hp(4) as usize], "Ann", 0);
            n += 1;
        }
        let mut setup = Vec::new();
        for _ in 0..3 {
            let id = LM9_P[r.hp(4) as usize];
            let prem = r.hp(2) == 1;
            setup.push(match l.add_member(id, "M", prem) { Ok(()) => "ok".to_string(), Err(e) => lm9_er(&e) });
        }
        if case == want {
            log.push_str(&format!("setup {}\n", setup.join(",")));
        }
        for step in 0..20 {
            let form = r.hp(12);
            let line = match form {
                0 => {
                    let t = LM9_AT[r.hp(4) as usize];
                    format!("add {t}|Ann|{}", LM9_Y[r.hp(5) as usize])
                }
                1 => LM9_BADADD[r.hp(4) as usize].to_string(),
                2 | 3 | 4 => {
                    let b = if r.hp(8) != 0 { 1 + r.hp(n) } else { [0, n + 1][r.hp(2) as usize] };
                    let m = LM9_MQ[r.hp(5) as usize];
                    let d = LM9_D[r.hp(5) as usize];
                    format!("checkout {b} {m} {d}")
                }
                5 | 6 => {
                    let mut first = None;
                    for id in 1..=n {
                        if first.is_none() && l.book(id).unwrap().borrower().is_some() {
                            first = Some(id);
                        }
                    }
                    let b = match (r.hp(4) == 0, first) {
                        (true, Some(id)) => id,
                        _ => r.hp(n + 2),
                    };
                    format!("return {b} {}", LM9_R[r.hp(8) as usize])
                }
                7 => LM9_FIND[r.hp(5) as usize].to_string(),
                8 => LM9_REP[r.hp(3) as usize].to_string(),
                9 => LM9_BAD[r.hp(8) as usize].to_string(),
                10 => LM9_NL[r.hp(3) as usize].to_string(),
                _ => String::new(),
            };
            let res = if form == 11 {
                let id = LM9_P[r.hp(4) as usize];
                let prem = r.hp(2) == 1;
                format!("member {}", match l.add_member(id, "M", prem) { Ok(()) => "ok".to_string(), Err(e) => lm9_er(&e) })
            } else {
                match parse_command(&line) {
                    Err(e) => format!("parse E:{}", lm9_er(&e)),
                    Ok(c) => {
                        let p = lm9_cm(&c);
                        let x = match l.execute(c) {
                            Ok(t) => format!("X:{}", lm9_hx(&t)),
                            Err(e) => format!("E:{}", lm9_er(&e)),
                        };
                        format!("parse P:{p} {x}")
                    }
                }
            };
            n = l.count_by_status().0 as u32 + l.count_by_status().1 as u32 + l.count_by_status().2 as u32;
            let mut state = format!("{:?}", l.count_by_status());
            for id in 0..n + 2 {
                state.push_str(&match l.book(id) {
                    None => " bN".to_string(),
                    Some(b) => match b.status {
                        Status::Available => format!(" b{}:{}:{}:A", lm9_hx(&b.title), lm9_hx(&b.author), b.year),
                        Status::Borrowed { member_id, due_day } => format!(" b{}:{}:{}:B{member_id}/{due_day}", lm9_hx(&b.title), lm9_hx(&b.author), b.year),
                        Status::Lost => " L".to_string(),
                    },
                });
            }
            for id in LM9_MS {
                state.push_str(&match l.member(id) {
                    None => " mN".to_string(),
                    Some(m) => format!(" m{}:{}", m.premium, m.borrowed),
                });
            }
            if case == want {
                log.push_str(&format!("{step} {res} | {state}\n"));
            }
        }
    }
    log
}

const LM10_TT: [&str; 6] = ["Rust", " The  Rust Book ", "東京  物語", "İSTANBUL", "ΟΣ", "Straße"];
const LM10_AU: [&str; 4] = ["Ada", " ann lee ", "É", "Bo\u{3000}"];
const LM10_YR: [u16; 3] = [0, 1965, 65535];
const LM10_MI: [u32; 5] = [1, 2, 7, 4294967295, 0];
const LM10_MN: [&str; 4] = ["Ann", "Bo Lee", "  ada  ", "東京"];
const LM10_Q: [&str; 10] = ["", "rust", "RUST book", "ada", "lee ann", "i\u{307}", "ος", "STRASSE", "東京", "the rust"];

/// Case `want` of LM10's seeded suite (seed 0x0A000201): which `add_member` calls failed,
/// the ids `find` returns, `members_sorted`, member `search` through references, and
/// every book's `Display` text.
fn lm10_case(want: u32) -> String {
    let mut r = Lm9Rng(0x0A000201);
    for case in 0..=want {
        let mut l = crate::Library::new();
        let nb = 1 + r.hp(5);
        let mut ids = Vec::new();
        for _ in 0..nb {
            let t = LM10_TT[r.hp(6) as usize];
            let a = LM10_AU[r.hp(4) as usize];
            let y = LM10_YR[r.hp(3) as usize];
            ids.push(l.add_book(t, a, y));
        }
        let mut rejected = Vec::new();
        for _ in 0..r.hp(4) {
            let id = LM10_MI[r.hp(5) as usize];
            let name = LM10_MN[r.hp(4) as usize];
            rejected.push(l.add_member(id, name, false).is_err());
        }
        let q = LM10_Q[r.hp(10) as usize];
        if case < want {
            continue;
        }
        let mut found = Vec::new();
        for b in l.find(q) {
            found.push(b.id);
        }
        let sorted = l.members_sorted();
        let mut members = Vec::new();
        for m in &sorted {
            members.push(m.id);
        }
        let mut matched = Vec::new();
        for m in crate::search(&sorted, q) {
            matched.push(m.id);
        }
        let mut shown = Vec::new();
        for id in ids {
            shown.push(match l.book(id) {
                Some(b) => format!("{b}"),
                None => String::new(),
            });
        }
        return format!("rejected {rejected:?} find {found:?} members {members:?} search {matched:?} books {shown:?}");
    }
    unreachable!()
}
