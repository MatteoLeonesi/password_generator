use password_generator::generate_password;

extern crate password_generator;

// returns a string of specified length
#[test]
fn test_returns_string_of_specified_length() {
    let length = 10;
    let password = generate_password(length);
    assert_eq!(password.len(), length);
}
