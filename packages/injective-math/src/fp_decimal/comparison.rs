use crate::fp_decimal::FPDecimal;
use std::cmp::Ordering;

impl Ord for FPDecimal {
    fn cmp(&self, other: &FPDecimal) -> Ordering {
        if self.sign == other.sign {
            let mut ordering = self.num.cmp(&other.num);

            if self.sign == 0 {
                ordering = ordering.reverse()
            }

            return ordering;
        }

        if self.sign == 1 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq for FPDecimal {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.sign == other.sign
    }
}

impl PartialOrd for FPDecimal {
    fn partial_cmp(&self, other: &FPDecimal) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign < other.sign;
        }

        if self.sign == 0 {
            return self.num >= other.num;
        }

        self.num < other.num
    }

    fn le(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign < other.sign;
        }

        if self.sign == 0 {
            return self.num >= other.num;
        }

        self.num <= other.num
    }

    fn gt(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign > other.sign;
        }

        if self.sign == 0 {
            return self.num < other.num;
        }

        self.num > other.num
    }

    fn ge(&self, other: &Self) -> bool {
        if self.sign != other.sign {
            return self.sign > other.sign;
        }

        if self.sign == 0 {
            return self.num <= other.num;
        }

        self.num >= other.num
    }
}

impl FPDecimal {
    pub fn maximum(&self, other: &Self) -> FPDecimal {
        match self.cmp(other) {
            Ordering::Greater | Ordering::Equal => *self,
            Ordering::Less => *other,
        }
    }

    pub fn minimum(&self, other: &Self) -> FPDecimal {
        match self.cmp(other) {
            Ordering::Less | Ordering::Equal => *self,
            Ordering::Greater => *other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::FPDecimal;
    use std::str::FromStr;

    #[test]
    fn test_is_greater() {
        let mut is_greater = FPDecimal::from_str("1.0").unwrap() > FPDecimal::from_str("-5.0").unwrap();
        assert!(is_greater);

        is_greater = FPDecimal::from_str("1.0").unwrap() > FPDecimal::from_str("0.99999").unwrap();
        assert!(is_greater);

        is_greater = FPDecimal::from_str("-1.0").unwrap() > FPDecimal::from_str("-2.0").unwrap();
        assert!(is_greater);

        is_greater = FPDecimal::from_str("4.0").unwrap() > FPDecimal::from_str("1.0").unwrap();
        assert!(is_greater);
    }

    #[test]
    fn test_is_equal() {
        let mut is_equal = FPDecimal::from_str("1.0").unwrap() == FPDecimal::from_str("1.0").unwrap();
        assert!(is_equal);

        is_equal = FPDecimal::from_str("1.0").unwrap() == FPDecimal::from_str("-1.0").unwrap();
        assert!(!is_equal);
    }

    #[test]
    fn test_is_not_equal() {
        let mut is_not_equal = FPDecimal::from_str("1.0").unwrap() != FPDecimal::from_str("1.0").unwrap();
        assert!(!is_not_equal);

        is_not_equal = FPDecimal::from_str("1.0").unwrap() != FPDecimal::from_str("-1.0").unwrap();
        assert!(is_not_equal);
    }

    #[test]
    fn test_is_not_greater() {
        let mut is_greater = FPDecimal::from_str("-5.0").unwrap() > FPDecimal::from_str("1.0").unwrap();
        assert!(!is_greater);

        is_greater = FPDecimal::from_str("0.99999").unwrap() > FPDecimal::from_str("1.0").unwrap();
        assert!(!is_greater);

        is_greater = FPDecimal::from_str("-2.0").unwrap() > FPDecimal::from_str("-1.0").unwrap();
        assert!(!is_greater);

        is_greater = FPDecimal::from_str("1.0").unwrap() > FPDecimal::from_str("4.0").unwrap();
        assert!(!is_greater);

        is_greater = FPDecimal::from_str("1.0").unwrap() > FPDecimal::from_str("1.0").unwrap();
        assert!(!is_greater);
    }

    #[test]
    fn test_is_greater_equal() {
        let mut is_greater_equal = FPDecimal::from_str("1.0").unwrap() >= FPDecimal::from_str("-5.0").unwrap();
        assert!(is_greater_equal);

        is_greater_equal = FPDecimal::from_str("1.0").unwrap() >= FPDecimal::from_str("0.99999").unwrap();
        assert!(is_greater_equal);

        is_greater_equal = FPDecimal::from_str("-1.0").unwrap() >= FPDecimal::from_str("-2.0").unwrap();
        assert!(is_greater_equal);

        is_greater_equal = FPDecimal::from_str("4.0").unwrap() >= FPDecimal::from_str("1.0").unwrap();
        assert!(is_greater_equal);

        is_greater_equal = FPDecimal::from_str("-2.3").unwrap() >= FPDecimal::from_str("-2.3").unwrap();
        assert!(is_greater_equal);

        is_greater_equal = FPDecimal::from_str("2.3").unwrap() >= FPDecimal::from_str("2.3").unwrap();
        assert!(is_greater_equal);

        is_greater_equal = FPDecimal::from_str("0.0").unwrap() >= FPDecimal::from_str("0.0").unwrap();
        assert!(is_greater_equal);
    }

    #[test]
    fn test_is_not_greater_equal() {
        let mut is_greater_equal = FPDecimal::from_str("-5.0").unwrap() >= FPDecimal::from_str("1.0").unwrap();
        assert!(!is_greater_equal);

        is_greater_equal = FPDecimal::from_str("0.99999").unwrap() >= FPDecimal::from_str("1.0").unwrap();
        assert!(!is_greater_equal);

        is_greater_equal = FPDecimal::from_str("-2.0").unwrap() >= FPDecimal::from_str("-1.0").unwrap();
        assert!(!is_greater_equal);

        is_greater_equal = FPDecimal::from_str("1.0").unwrap() >= FPDecimal::from_str("4.0").unwrap();
        assert!(!is_greater_equal);
    }

    #[test]
    fn test_is_lesser() {
        let mut is_lesser = FPDecimal::from_str("-5.0").unwrap() < FPDecimal::from_str("1.0").unwrap();
        assert!(is_lesser);

        is_lesser = FPDecimal::from_str("0.99999").unwrap() < FPDecimal::from_str("1.0").unwrap();
        assert!(is_lesser);

        is_lesser = FPDecimal::from_str("-2.0").unwrap() < FPDecimal::from_str("-1.0").unwrap();
        assert!(is_lesser);

        is_lesser = FPDecimal::from_str("1.0").unwrap() < FPDecimal::from_str("4.0").unwrap();
        assert!(is_lesser);
    }

    #[test]
    fn test_is_not_lesser() {
        let mut is_lesser = FPDecimal::from_str("1.0").unwrap() < FPDecimal::from_str("-5.0").unwrap();
        assert!(!is_lesser);

        is_lesser = FPDecimal::from_str("1.0").unwrap() < FPDecimal::from_str("0.99999").unwrap();
        assert!(!is_lesser);

        is_lesser = FPDecimal::from_str("-1.0").unwrap() < FPDecimal::from_str("-2.0").unwrap();
        assert!(!is_lesser);

        is_lesser = FPDecimal::from_str("4.0").unwrap() < FPDecimal::from_str("1.0").unwrap();
        assert!(!is_lesser);

        is_lesser = FPDecimal::from_str("1.0").unwrap() < FPDecimal::from_str("1.0").unwrap();
        assert!(!is_lesser);
    }

    #[test]
    fn test_is_lesser_equal() {
        let mut is_lesser_equal = FPDecimal::from_str("-5.0").unwrap() <= FPDecimal::from_str("1.0").unwrap();
        assert!(is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("0.99999").unwrap() <= FPDecimal::from_str("1.0").unwrap();
        assert!(is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("-2.0").unwrap() <= FPDecimal::from_str("-1.0").unwrap();
        assert!(is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("1.0").unwrap() <= FPDecimal::from_str("4.0").unwrap();
        assert!(is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("-2.3").unwrap() <= FPDecimal::from_str("-2.3").unwrap();
        assert!(is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("2.3").unwrap() <= FPDecimal::from_str("2.3").unwrap();
        assert!(is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("0.0").unwrap() <= FPDecimal::from_str("0.0").unwrap();
        assert!(is_lesser_equal);
    }

    #[test]
    fn test_is_not_lesser_equal() {
        let mut is_lesser_equal = FPDecimal::from_str("1.0").unwrap() <= FPDecimal::from_str("-5.0").unwrap();
        assert!(!is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("1.0").unwrap() <= FPDecimal::from_str("0.99999").unwrap();
        assert!(!is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("-1.0").unwrap() <= FPDecimal::from_str("-2.0").unwrap();
        assert!(!is_lesser_equal);

        is_lesser_equal = FPDecimal::from_str("4.0").unwrap() <= FPDecimal::from_str("1.0").unwrap();
        assert!(!is_lesser_equal);
    }

    #[test]
    fn test_maximum_of_values() {
        let lhs = FPDecimal::from_str("-1.0").unwrap();
        let rhs = FPDecimal::from_str("1.0").unwrap();
        assert_eq!(rhs, lhs.maximum(&rhs));

        let lhs = FPDecimal::from_str("1.0").unwrap();
        let rhs = FPDecimal::from_str("1.0").unwrap();
        assert_eq!(lhs, lhs.maximum(&rhs));

        let lhs = FPDecimal::from_str("1.0").unwrap();
        let rhs = FPDecimal::from_str("-1.0").unwrap();
        assert_eq!(lhs, lhs.maximum(&rhs));
    }

    #[test]
    fn test_minimum_of_values() {
        let lhs = FPDecimal::from_str("-1.0").unwrap();
        let rhs = FPDecimal::from_str("1.0").unwrap();
        assert_eq!(lhs, lhs.minimum(&rhs));

        let lhs = FPDecimal::from_str("1.0").unwrap();
        let rhs = FPDecimal::from_str("1.0").unwrap();
        assert_eq!(rhs, lhs.minimum(&rhs));

        let lhs = FPDecimal::from_str("1.0").unwrap();
        let rhs = FPDecimal::from_str("-1.0").unwrap();
        assert_eq!(rhs, lhs.minimum(&rhs));
    }
}
