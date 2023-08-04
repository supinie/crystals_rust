#[cfg(test)]
mod field_tests {
    use crate::{params::*, field_ops::*};    
    use more_asserts::{assert_gt, assert_lt};

    #[test]
    pub fn montgomery_reduce_test() {
        assert_eq!(montgomery_reduce(i32::MAX), 32599);
        assert_eq!(montgomery_reduce(i32::MIN), -32768);
    }

    #[test]
    pub fn to_mont_test() {
        assert_eq!(to_mont(i16::MAX), 56);
        assert_eq!(to_mont(i16::MIN), 988);
    }

    #[test]
    pub fn barrett_reduce_test() {
        assert_eq!(barrett_reduce(i16::MAX), 2806);
        assert_eq!(barrett_reduce(i16::MIN), 522);
    }

    #[test]
    pub fn cond_sub_q_test() {
        assert_eq!(cond_sub_q(i16::MAX), 29438);
        assert_eq!(cond_sub_q(-29439), -29439);
    }
}