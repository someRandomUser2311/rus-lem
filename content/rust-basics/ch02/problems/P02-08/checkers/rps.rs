//! Checker for P02-08 (the RPS transcript predicate in problems.md): rebuilds the
//! transcript the input requires and accepts any valid computer move in each round.
//! Arguments: the stdin file and the program's stdout file. Exit 0 = pass; otherwise
//! stdout says what's wrong.

use std::process::exit;

const ASK: &str = "Your move (rock, paper, scissors, again, quit):\n";

fn name(m: u32) -> &'static str {
    match m {
        1 => "rock",
        2 => "paper",
        _ => "scissors",
    }
}

fn result(player: u32, computer: u32) -> &'static str {
    match (player, computer) {
        (p, c) if p == c => "It's a tie!\n",
        (1, 3) | (2, 1) | (3, 2) => "You win!\n",
        _ => "You lose!\n",
    }
}

struct Out<'a> {
    text: &'a str,
    pos: usize,
    /// What the input line was, for messages.
    context: String,
}

impl Out<'_> {
    fn rest(&self) -> &str {
        &self.text[self.pos..]
    }

    fn fail(&self, want: &str) -> ! {
        let got: String = self.rest().chars().take(80).collect();
        let got = if got.is_empty() {
            "nothing (the output ends here)".to_string()
        } else {
            format!("{got:?}")
        };
        println!("{}: expected {want:?}, but the output has {got}.", self.context);
        exit(1)
    }

    fn expect(&mut self, s: &str) {
        if self.rest().starts_with(s) {
            self.pos += s.len();
        } else {
            self.fail(s);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let stdin = std::fs::read_to_string(&args[1]).expect("stdin file");
    let stdout = std::fs::read(&args[2]).expect("stdout file");
    let Ok(stdout) = String::from_utf8(stdout) else {
        println!("The output isn't valid UTF-8 text.");
        exit(1)
    };
    // LF-delimited lines; a final unterminated line still counts.
    let mut lines: Vec<&str> = stdin.split('\n').collect();
    if stdin.is_empty() || stdin.ends_with('\n') {
        lines.pop();
    }
    let mut lines = lines.into_iter();
    let mut out = Out {
        text: &stdout,
        pos: 0,
        context: "At the start".into(),
    };
    out.expect("Rock, paper, scissors!\n");
    let mut last = 0;
    let mut round = 0;
    loop {
        out.expect(ASK);
        let Some(line) = lines.next() else {
            out.context = "At the end of the input".into();
            out.expect("Thanks for playing!\n");
            break;
        };
        out.context = format!("After the input line {line:?}");
        let player = match line.trim() {
            "rock" | "r" => 1,
            "paper" | "p" => 2,
            "scissors" | "s" => 3,
            "again" => last,
            "quit" => {
                out.expect("Thanks for playing!\n");
                break;
            }
            _ => {
                out.expect("I don't know that move.\n");
                continue;
            }
        };
        if player == 0 {
            out.expect("No previous move yet.\n");
            continue;
        }
        last = player;
        round += 1;
        out.context = format!("In round {round} (input line {line:?})");
        out.expect(&format!("You chose {}. Computer chose ", name(player)));
        let computer = match out.rest() {
            r if r.starts_with("rock.\n") => 1,
            r if r.starts_with("paper.\n") => 2,
            r if r.starts_with("scissors.\n") => 3,
            _ => out.fail("rock.\n, paper.\n or scissors.\n"),
        };
        out.expect(&format!("{}.\n", name(computer)));
        out.expect(result(player, computer));
    }
    if out.pos != stdout.len() {
        out.context = "After the game ended".into();
        out.fail("no more output");
    }
}
