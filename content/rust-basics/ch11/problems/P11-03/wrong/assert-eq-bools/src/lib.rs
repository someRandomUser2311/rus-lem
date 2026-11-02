#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_rect_fits() {
        let a = Rect { width: 8, height: 7 };
        let b = Rect { width: 5, height: 1 };
        assert_eq!(a.fits(&b), true);
    }

    #[test]
    fn same_size_does_not_fit() {
        let a = Rect { width: 8, height: 7 };
        let b = Rect { width: 8, height: 7 };
        assert_eq!(a.fits(&b), false);
    }

    #[test]
    fn wider_but_shorter_does_not_fit() {
        let a = Rect { width: 8, height: 3 };
        let b = Rect { width: 5, height: 4 };
        assert_eq!(a.fits(&b), false);
    }

    #[test]
    fn taller_but_narrower_does_not_fit() {
        let a = Rect { width: 3, height: 8 };
        let b = Rect { width: 5, height: 4 };
        assert_eq!(a.fits(&b), false);
    }
}
