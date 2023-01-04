# File IO Project

This introductory project contains a function called create_and_write_to_file which prompts the user for a file name and file content,
creates a new file with the given file name, and writes the content to the file.

## Usage
To use the create_and_write_to_file function, import it into your project with the following line of code:

```
use file_io::create_and_write_to_file;
```

Then, call the function with the following syntax:

```
create_and_write_to_file();
```

## Example
Here is an example of how you could use the create_and_write_to_file function in your code:

```
use file_io::create_and_write_to_file;

fn main() {
    let result = create_and_write_to_file();
    if let Err(e) = result {
        eprintln!("Error creating or writing to file: {}", e);
        return;
    }
    println!("Successfully created and wrote to file!");
}
```

## Testing
To run the tests for the create_and_write_to_file function, use the following command:
```
cargo test --test_file_creation_and_writing --ignored
```

You do not need use `#[ignore]` in your test file since we only have one test.
You can comment it out and use `cargo test --test_file_creation_and_writing` instead.

## After setup

1.use `cargo run` to run it and then answer to the questions.

2. Use the `ls` commmand to list directories and files. You should now be able to see the file you created.

3. Type `cat <your_filename>` to see the contents of the file 

![fileio-example](https://user-images.githubusercontent.com/77973084/210615239-ef84e2eb-f045-4d26-995b-49b1301c8450.png)
