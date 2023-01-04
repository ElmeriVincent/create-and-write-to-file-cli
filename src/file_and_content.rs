use std::fs::File;
use std::io::prelude::*;

/// Prompts the user for a file name and file content, creates a new file with the given filename, and writes the content to the file.
pub fn create_and_write_to_file() -> std::io::Result<()> {
    // Prompt the user for the file name and read the input
    let mut filename = String::new();
    println!("Enter the file name you want to create:");
    std::io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();

    // Prompt the user for the file content and read the input
    let mut file_content = String::new();
    println!("Enter the content you want to write to the file:");
    std::io::stdin().read_line(&mut file_content)?;

    // Convert the file content string to a byte slice so it can be passed as an argument to write_all
    let file_content = file_content.as_bytes();

    // Create a new file with the given file name. If the file already exists, it will be overwritten.
    // If the file cannot be created, an error is returned.
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return Err(e);
        }
    };

    // Write the file content to the file. If the write fails, an error is returned.
    match file.write_all(file_content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error writing to file: {}", e);
            return Err(e);
        }
    };

    println!("Successfully created and wrote to file: {}", filename);

    Ok(())
}

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

    // Delete the file.
    let result = std::fs::remove_file(test_filename);
    if let Err(e) = result {
        eprintln!("Error deleting file: {}", e);
    }
}
