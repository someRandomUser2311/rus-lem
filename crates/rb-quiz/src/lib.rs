//! Checking a learner's answer to a question.

use rb_content::{Answer, Question};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// What the learner submitted, shaped like [`Answer`].
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[ts(export)]
pub enum Response {
    Keys { keys: Vec<String> },
    Text { text: String },
    Blanks { blanks: Vec<String> },
    Order { order: Vec<String> },
    Lines { lines: Vec<u32> },
    Pairs { pairs: Vec<(String, String)> },
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Checked {
    pub correct: bool,
    /// Explanation keys to show: the options the learner picked plus the correct ones.
    pub explain: Vec<String>,
    /// For typed answers: the matching "common wrong answer", if the learner gave one.
    pub matched_wrong: Option<usize>,
}

/// Output comparison ignores whitespace at line ends and trailing blank lines.
pub fn normalize_output(s: &str) -> String {
    let s = s.replace("\r\n", "\n");
    s.lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end_matches('\n')
        .trim_start_matches('\n')
        .to_string()
}

fn norm_token(s: &str) -> String {
    s.trim()
        .trim_matches('`')
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn check(q: &Question, r: &Response) -> Checked {
    let mut explain = Vec::new();
    let mut matched_wrong = None;
    let correct = match (&q.answer, r) {
        (Answer::Keys { keys }, Response::Keys { keys: got }) => {
            let mut a = keys.clone();
            let mut b = got.clone();
            a.sort();
            b.sort();
            b.dedup();
            explain = q
                .options
                .iter()
                .map(|o| o.key.clone())
                .filter(|k| got.contains(k) || keys.contains(k))
                .collect();
            a == b
        }
        (Answer::Text { text, accepted }, Response::Text { text: got }) => {
            let g = normalize_output(got);
            let ok =
                normalize_output(text) == g || accepted.iter().any(|a| normalize_output(a) == g);
            if !ok {
                matched_wrong = q.wrong_answers.iter().position(|w| {
                    let shown = w.answer_md.replace("` / `", "\n");
                    normalize_output(shown.trim_matches('`')) == g
                });
            }
            ok
        }
        (Answer::Blanks { blanks }, Response::Blanks { blanks: got }) => {
            let ok = blanks.len() == got.len()
                && blanks
                    .iter()
                    .zip(got)
                    .all(|(acc, g)| acc.iter().any(|a| norm_token(a) == norm_token(g)));
            if !ok && got.len() == 1 {
                matched_wrong = q
                    .wrong_answers
                    .iter()
                    .position(|w| norm_token(&w.answer_md) == norm_token(&got[0]));
            }
            ok
        }
        (Answer::Orders { orders }, Response::Order { order }) => {
            let ok = orders.iter().any(|o| o == order);
            if !ok {
                let joined = order.join(", ");
                matched_wrong = q
                    .wrong_answers
                    .iter()
                    .position(|w| w.answer_md.trim() == joined);
            }
            ok
        }
        (Answer::Lines { lines }, Response::Lines { lines: got }) => {
            let mut a = lines.clone();
            let mut b = got.clone();
            a.sort();
            b.sort();
            b.dedup();
            if a != b {
                matched_wrong = got.first().and_then(|l| {
                    q.wrong_answers.iter().position(|w| {
                        w.answer_md
                            .trim()
                            .eq_ignore_ascii_case(&format!("line {l}"))
                    })
                });
            }
            a == b
        }
        (Answer::Pairs { pairs }, Response::Pairs { pairs: got }) => {
            let mut a = pairs.clone();
            let mut b = got.clone();
            a.sort();
            b.sort();
            explain = pairs.iter().map(|(l, r)| format!("{l}-{r}")).collect();
            a == b
        }
        _ => false,
    };
    Checked {
        correct,
        explain,
        matched_wrong,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(answer: Answer) -> Question {
        serde_json::from_value(serde_json::json!({
            "id": "Q", "uid": "Q", "kind": "single", "difficulty": 1, "concepts": [], "prompt_md": "",
            "options": [{"key": "A", "text_md": ""}, {"key": "B", "text_md": ""}, {"key": "C", "text_md": ""}],
            "answer": answer,
            "wrong_answers": [{"answer_md": "`ab` / `c`", "why_md": ""}, {"answer_md": "line 3", "why_md": ""}]
        }))
        .unwrap()
    }

    #[test]
    fn keys_are_order_free() {
        let q = q(Answer::Keys {
            keys: vec!["A".into(), "C".into()],
        });
        assert!(
            check(
                &q,
                &Response::Keys {
                    keys: vec!["C".into(), "A".into()]
                }
            )
            .correct
        );
        assert!(
            !check(
                &q,
                &Response::Keys {
                    keys: vec!["A".into()]
                }
            )
            .correct
        );
    }

    #[test]
    fn output_ignores_trailing_space() {
        let q = q(Answer::Text {
            text: "abc\nde".into(),
            accepted: vec![],
        });
        assert!(
            check(
                &q,
                &Response::Text {
                    text: "abc  \nde\n\n".into()
                }
            )
            .correct
        );
        let w = check(
            &q,
            &Response::Text {
                text: "ab\nc".into(),
            },
        );
        assert!(!w.correct);
        assert_eq!(w.matched_wrong, Some(0));
    }

    #[test]
    fn blanks_accept_alternatives() {
        let q = q(Answer::Blanks {
            blanks: vec![vec!["&mut".into(), "& mut".into()]],
        });
        assert!(
            check(
                &q,
                &Response::Blanks {
                    blanks: vec![" & mut ".into()]
                }
            )
            .correct
        );
    }

    #[test]
    fn spot_bug_lines() {
        let q = q(Answer::Lines { lines: vec![2] });
        assert!(check(&q, &Response::Lines { lines: vec![2] }).correct);
        assert_eq!(
            check(&q, &Response::Lines { lines: vec![3] }).matched_wrong,
            Some(1)
        );
    }
}
