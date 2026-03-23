#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Ready,
    Missing,
}

pub fn saturating_increment(value: u32) -> u32 {
    value.saturating_add(1)
}

pub fn classify(input: Option<u32>) -> Result<u32, Status> {
    match input {
        Some(value) if value > 0 => Ok(saturating_increment(value)),
        Some(_) => Err(Status::Missing),
        None => Err(Status::Ready),
    }
}

pub fn sum_prefix(values: &[u32], count: usize) -> u32 {
    let mut acc = 0;
    let mut i = 0;
    while i < count && i < values.len() {
        acc += values[i];
        i += 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_some_value() {
        assert_eq!(classify(Some(2)), Ok(3));
    }

    #[test]
    fn classify_zero() {
        assert_eq!(classify(Some(0)), Err(Status::Missing));
    }

    #[test]
    fn prefix_sum_is_bounded() {
        assert_eq!(sum_prefix(&[1, 2, 3], 2), 3);
        assert_eq!(sum_prefix(&[1, 2, 3], 10), 6);
    }
}
