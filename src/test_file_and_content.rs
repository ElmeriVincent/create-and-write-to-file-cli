use file_and_content::create_and_write_to_file;

#[test]
#[ignore] // cargo test -- test_file_creation_and_writing --ignored

/// GIVEN: A predetermined file name and file content
/// WHEN: The `create_and_write_to_file` function is called
/// THEN: A new file with the given file name is created and contains the

fn test_file_creation_and_writing() {
    // Create a file name and file content for the test.
    let test_filename = "test_file.txt";
    let test_file_content = "This is a test file.";

    // Call the create_and_write_to_file function.
    let result = create_and_write_to_file();
    if let Err(e) = result {
        eprintln!("Error creating or writing to file: {}", e);
        return;
    }

    // Read the contents of the file and store it in a variable.
    let mut file = match File::open(test_filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    // Print out the contents of the file for debugging.
    println!("File contents: {}", file_contents);

    // Assert that the contents of the file are equal to the expected file content.
    assert_eq!(file_contents, test_file_content);
}
