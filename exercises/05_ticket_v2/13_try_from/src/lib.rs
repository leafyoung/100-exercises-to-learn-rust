// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq, Clone)]
struct StatusError {}

impl std::fmt::Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status Error")
    }
}

impl Error for StatusError {}

impl TryFrom<String> for Status {
    type Error = StatusError;
    fn try_from(value: String) -> Result<Status, StatusError> {
        match value.to_uppercase().as_str() {
            "TODO" => Ok(Status::ToDo),
            "INPROGRESS" => Ok(Status::InProgress),
            "DONE" => Ok(Status::Done),
            _ => Err(StatusError {}),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusError;
    fn try_from(value: &str) -> Result<Status, StatusError> {
        value.to_owned().try_into()
    }
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

        let empty = String::new();
        assert!(Status::try_from(empty).is_err());

        let invalid = "InValid".to_string();
        assert!(Status::try_from(invalid).is_err());
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);

        assert!(Status::try_from("").is_err());
        assert!(Status::try_from("invalid").is_err());
    }
}
