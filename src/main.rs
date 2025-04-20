mod huffman;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

fn read_file(file_name: &str) -> Result<String, io::Error> {
    //Read the whole file to a string
    fs::read_to_string(file_name)
}
fn write_encoded_to_file(file_name: &str, encoded_content: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_name)?;
    file.write_all(encoded_content.as_bytes())?;
    Ok(())
}

fn main() {
    //const FILE_NAME: &str = "bee20script.txt";
    //let contents = read_file(FILE_NAME).expect("File wasn't read");
    let contents = "EEEEAABBCCEEEEEEEEECD1234sadfthomasaE";
    const OUTPUT_FILENAME: &str = "output.txt";

    let huffman_tree = huffman::HuffmanTree::new(&contents);
    let encoded_content = huffman_tree.get_encoded().expect("Error encoding failed");
    println!("Success encode: {}", encoded_content);
    match write_encoded_to_file(OUTPUT_FILENAME, &encoded_content) {
        Ok(_) => println!("Success writing to {OUTPUT_FILENAME}"),
        Err(_) => println!("Failed to write to {OUTPUT_FILENAME}"),
    }
}
