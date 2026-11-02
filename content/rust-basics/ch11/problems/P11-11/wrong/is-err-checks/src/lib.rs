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
        assert!(parse("8080").is_err());
        assert!(parse("8080,true,x").is_err());
    }

    #[test]
    fn port_must_be_1_to_65535() {
        assert!(parse("0,true").is_err());
        assert!(parse("65536,true").is_err());
        assert!(parse("x,true").is_err());
    }

    #[test]
    fn debug_must_be_true_or_false() {
        assert!(parse("80,yes").is_err());
        assert!(parse("80,TRUE").is_err());
    }

    #[test]
    fn the_port_is_checked_before_debug() {
        assert!(parse("x,yes").is_err());
    }
}
