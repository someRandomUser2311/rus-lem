#[derive(Debug, Clone, PartialEq)]
pub enum Msg {
    Ping,
    Echo(Vec<u8>),
    Add(u8, u8),
    Move { x: i8, y: i8 },
}

pub fn parse(msg: &[u8]) -> Result<Msg, String> {
    match msg {
        [] => Err(String::from("empty")),
        [0x01] => Ok(Msg::Ping),
        [0x01, ..] => Err(String::from("ping takes no payload")),
        [0x02, _len, data @ ..] => Ok(Msg::Echo(data.to_vec())),
        [0x02, ..] => Err(String::from("bad echo length")),
        [0x03, a, b] => Ok(Msg::Add(*a, *b)),
        [0x04, x, y] => Ok(Msg::Move {
            x: *x as i8,
            y: *y as i8,
        }),
        [0x03 | 0x04, ..] => Err(String::from("expected 2 bytes")),
        [op, ..] => Err(format!("unknown opcode {op:#04x}")),
    }
}

pub fn respond(msg: &[u8]) -> String {
    match parse(msg) {
        Ok(Msg::Ping) => String::from("pong"),
        Ok(Msg::Echo(data)) => String::from_utf8_lossy(&data).into_owned(),
        Ok(Msg::Add(a, b)) => (a as u16 + b as u16).to_string(),
        Ok(Msg::Move { x: 0, y: 0 }) => String::from("stay"),
        Ok(Msg::Move { x, y: 0 }) => format!("horizontal {x}"),
        Ok(Msg::Move { x: 0, y }) => format!("vertical {y}"),
        Ok(Msg::Move { .. }) => String::from("diagonal"),
        Err(e) => format!("error: {e}"),
    }
}
