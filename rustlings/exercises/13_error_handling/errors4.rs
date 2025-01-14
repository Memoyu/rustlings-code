use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.

        // 方式1: 手动撸if
        // match value {
        //     v if v < 0 => Err(CreationError::Negative),
        //     v if v == 0 => Err(CreationError::Zero),
        //     _ => Ok(Self(value as u64)),
        // }

        // 方式2: 使用cmp，匹配Ordering枚举
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Greater => Ok(Self(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
