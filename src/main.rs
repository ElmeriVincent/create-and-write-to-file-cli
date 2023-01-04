use file_and_content::create_and_write_to_file;

pub mod file_and_content;

fn main() -> std::io::Result<()> {
    create_and_write_to_file()
}
