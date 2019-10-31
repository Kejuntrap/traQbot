#[test]
fn test_parse_command() {
    use crate::router::functions::{Command,parse_command};

    // white box test
    assert!(parse_command("this is not a command").is_none());
    assert!(parse_command("@bot_xecua_odai").is_none());
    assert_eq!(parse_command("@bot_xecua_odai help").unwrap(), Command::Help);
    assert_eq!(parse_command("random 1").unwrap(), Command::Random(vec![String::from("1")]));
}