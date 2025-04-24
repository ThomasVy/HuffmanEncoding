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
    //const OUTPUT_FILENAME: &str = "output.txt";
    let contents = "EEEEAABBCCEEEEEEEEECD1234sadfthomasaE";

    let huffman_tree = huffman::HuffmanTree::new(contents);
    let encoded_content = huffman_tree
        .get_encoded(contents)
        .expect("Error encoding failed");
    println!("Success encode: {}", encoded_content);
}
