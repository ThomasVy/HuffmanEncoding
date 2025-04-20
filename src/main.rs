mod huffman;
use core::panic;
use std::fs;
use std::io;

fn read_file(file_name: &str) -> Result<String, io::Error> {
    //Read the whole file to a string
    fs::read_to_string(file_name)
}

fn main() {
    //const FILE_NAME: &str = "bee20script.txt";
    //let contents = match read_file(FILE_NAME) {
    //    Ok(contents) => contents,
    //    Err(error) => panic!("Could not read {}, error:{}", FILE_NAME, error),
    //};

    let contents = "EEEEAABBCCEEEEEEEEECD1234sadfthomasaE";

    let huffman_tree = huffman::HuffmanTree::new(contents);
    let encoded_content = huffman_tree.get_encoded();
    match encoded_content {
        Some(text) => println!("{:?}", text),
        None => panic!("What happend"),
    }

    println!("huffman");
}
