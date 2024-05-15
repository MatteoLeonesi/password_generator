extern crate password_generator;

#[test]
fn test_integration_generate_password() {
    let password = password_generator::generate_password(16);
    assert_eq!(password.len(), 16);
    // Optional: Additional checks for character set, etc.
}