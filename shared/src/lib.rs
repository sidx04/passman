pub mod protocol;

#[cfg(test)]
mod tests {
    use super::protocol::*;
    use serde_json;

    #[test]
    fn test_command_serialization() {
        let cmd = Command::Add {
            username: "alice".into(),
            password: "password123".into(),
            service: "github".into(),
            secret: "ghp_token".into(),
        };

        let json = serde_json::to_string(&cmd).unwrap();
        let back: Command = serde_json::from_str(&json).unwrap();

        match back {
            Command::Add { username, .. } => {
                assert_eq!(username, "alice");
            }
            _ => panic!("Wrong variant"),
        }
    }
}
