
/// Computes `dividend / divisor`, rounded up to the next integer.
/// This only works for positive numbers, it must NOT be called with possible negative values.
macro_rules! ceil_divide {
    ($dividend:expr, $divisor:expr) => {
        ($dividend + $divisor - 1) / $divisor
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn ceil_divide_doesnt_round_clean_divisions() {
        assert_eq!(ceil_divide!(0, 1), 0);
        assert_eq!(ceil_divide!(0, 64), 0);
        assert_eq!(ceil_divide!(0, 97), 0);

        assert_eq!(ceil_divide!(1, 1), 1);

        assert_eq!(ceil_divide!(6, 6), 1);
        assert_eq!(ceil_divide!(6, 3), 2);
        assert_eq!(ceil_divide!(6, 2), 3);
        assert_eq!(ceil_divide!(6, 1), 6);
    }

    #[test]
    fn ceil_divide_rounds_unclean_divisions_up() {
        assert_eq!(ceil_divide!(1, 2), 1);
        assert_eq!(ceil_divide!(1, 64), 1);
        assert_eq!(ceil_divide!(1, 97), 1);

        assert_eq!(ceil_divide!(7, 13), 1);
        assert_eq!(ceil_divide!(7, 6), 2);
        assert_eq!(ceil_divide!(7, 3), 3);
        assert_eq!(ceil_divide!(7, 2), 4);

        assert_eq!(ceil_divide!(101, 102), 1);
        assert_eq!(ceil_divide!(101, 97), 2);
        assert_eq!(ceil_divide!(101, 7), 15);
    }
}
