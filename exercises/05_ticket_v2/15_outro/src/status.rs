// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use std::error::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = StatusError;
    fn try_from(value: String) -> Result<Self, StatusError> {
        match value.to_uppercase().as_str() {
            "TODO" => Ok(Status::ToDo),
            "INPROGRESS" => Ok(Status::InProgress),
            "DONE" => Ok(Status::Done),
            _ => Err(StatusError::InvalidStatusError),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;
    fn try_from(value: &str) -> Result<Self, StatusError> {
        value.to_owned().try_into()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StatusError {
    #[error("Invalid status")]
    InvalidStatusError,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
