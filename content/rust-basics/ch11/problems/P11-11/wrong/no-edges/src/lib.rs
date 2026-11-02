#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_configs() -> Result<(), Error> {
        let padded = parse(" 8080 , true ")?;
        assert_eq!(padded, Config { port: 8080, debug: true });
        let lowest = parse("1,false")?;
        assert_eq!(lowest, Config { port: 1, debug: false });
        let highest = parse("65535,true")?;
        assert_eq!(highest.port, 65535);
        Ok(())
    }

    #[test]
    fn needs_exactly_two_fields() {
        assert_eq!(parse("8080"), Err(Error::Fields));
    }

    #[test]
    fn port_must_be_1_to_65535() {
        assert_eq!(parse("65536,true"), Err(Error::Port));
        assert_eq!(parse("x,true"), Err(Error::Port));
    }

    #[test]
    fn debug_must_be_true_or_false() {
        assert_eq!(parse("80,yes"), Err(Error::Debug));
    }

    #[test]
    fn the_port_is_checked_before_debug() {
        assert_eq!(parse("x,yes"), Err(Error::Port));
    }
}
