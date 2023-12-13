#[test]
fn test_text() {
    use std::fs;
    use std::process::Command;
    fs::write("test_in.txt", "Hello").expect("Couldn't create file");
    let output = Command::new("../target/debug/encoder")
        .arg("--input=test_in.txt")
        .arg("--output=test_out.txt")
        .output()
        .expect("Failed to execute process");

    assert!(output.stderr.is_empty());
    assert!(output.status.success());
    let result = fs::read_to_string("test_out.txt").expect("Error reading");
    assert_eq!(result, "SGVsbG8=");
    fs::remove_file("test_in.txt").expect("Couldn't clean up");
    fs::remove_file("test_out.txt").expect("Couldn't clean up");
}

#[test]
fn image() {
    use std::fs;
    use std::process::Command;

    let output = Command::new("../target/debug/encoder")
        .arg("--input=cat.jpg")
        .arg("--output=cat_out.txt")
        .output()
        .expect("Failed to execute process");

    assert!(output.stderr.is_empty());
    assert!(output.status.success());
    let result = fs::read_to_string("cat_out.txt").expect("Error reading");
    let expected = fs::read_to_string("cat_result.txt").expect("Error reading");
    assert_eq!(result, expected);
    fs::remove_file("cat_out.txt").expect("Couldn't clean up");
}
