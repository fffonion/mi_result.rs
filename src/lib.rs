/// Extension trait that mirrors `Result::is_ok` with a friendlier name.
pub trait MiResult {
    fn are_you_ok(&self) -> bool;
}

impl<T, E> MiResult for Result<T, E> {
    #[inline]
    fn are_you_ok(&self) -> bool {
        self.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_ok_with_ok() {
        let result: Result<i32, &str> = Ok(42);
        assert!(result.are_you_ok());
        assert_eq!(result.are_you_ok(), result.is_ok());
    }

    #[test]
    fn test_are_you_ok_with_err() {
        let result: Result<i32, &str> = Err("something went wrong");
        assert!(!result.are_you_ok());
        assert_eq!(result.are_you_ok(), result.is_ok());
    }

    #[test]
    fn test_are_you_ok_with_different_types() {
        let string_result: Result<String, std::io::Error> = Ok("success".to_string());
        assert!(string_result.are_you_ok());

        let vec_result: Result<Vec<u8>, Box<dyn std::error::Error>> = Err("error".into());
        assert!(!vec_result.are_you_ok());
    }

    #[test]
    fn test_behavior_matches_is_ok() {
        let results = vec![
            Ok::<_, ()>(1),
            Ok::<_, ()>(2),
            Err(()),
            Ok::<_, ()>(3),
        ];

        for result in results.iter() {
            assert_eq!(result.are_you_ok(), result.is_ok());
        }
    }
}
