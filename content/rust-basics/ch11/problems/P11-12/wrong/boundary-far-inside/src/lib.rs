#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_moves_money() -> Result<(), MoneyError> {
        let mut a = Account::new(100);
        let mut b = Account::new(5);
        transfer(&mut a, &mut b, 30)?;
        assert_eq!(a.balance(), 70, "the sender is debited 30");
        assert_eq!(b.balance(), 35, "the receiver is credited 30");
        Ok(())
    }

    #[test]
    fn zero_transfer_succeeds() -> Result<(), MoneyError> {
        let mut a = Account::new(0);
        let mut b = Account::new(0);
        transfer(&mut a, &mut b, 0)?;
        assert_eq!(a.balance(), 0, "nothing leaves the sender");
        assert_eq!(b.balance(), 0, "nothing reaches the receiver");
        Ok(())
    }

    #[test]
    fn insufficient_funds_change_nothing() {
        let mut a = Account::new(10);
        let mut b = Account::new(0);
        assert_eq!(transfer(&mut a, &mut b, 11), Err(MoneyError::Insufficient), "11 is more than 10");
        assert_eq!(a.balance(), 10, "the sender keeps all its money");
        assert_eq!(b.balance(), 0, "the receiver gets nothing");
    }

    #[test]
    fn overflow_changes_nothing() {
        let mut a = Account::new(10);
        let mut b = Account::new(u32::MAX - 5);
        assert_eq!(transfer(&mut a, &mut b, 6), Err(MoneyError::Overflow), "6 more would pass u32::MAX");
        assert_eq!(a.balance(), 10, "the sender is not debited when the receiver can't take it");
        assert_eq!(b.balance(), u32::MAX - 5, "the receiver is unchanged");
    }

    #[test]
    fn can_receive_up_to_the_maximum() {
        assert!(can_receive(0, 5), "an empty account can take 5");
        assert!(!can_receive(u32::MAX - 5, 6), "one more would overflow");
    }
}
