use std::fmt;

#[derive(Debug)]
pub enum HuffmanError {
    EncodingError(String),
}

impl fmt::Display for HuffmanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HuffmanError::EncodingError(msg) => write!(f, "There was an encoding error: {}", msg),
        }
    }
}

impl std::error::Error for HuffmanError {}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct HuffmanTree {
    root: Option<Box<HuffmanNode>>,
    code_map: HashMap<char, String>,
}

// Ordering for BinaryHeap
#[derive(Eq, PartialEq)]
enum HuffmanNode {
    Leaf {
        freq: usize,
        ch: char,
    },
    Internal {
        freq: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
}

impl HuffmanNode {
    fn freq(&self) -> usize {
        match self {
            HuffmanNode::Leaf { freq, .. } => *freq,
            HuffmanNode::Internal { freq, .. } => *freq,
        }
    }
}

// Custom ordering based on freq
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_frequency_map(contents: &str) -> BinaryHeap<Reverse<HuffmanNode>> {
    let mut frequency_map: HashMap<char, usize> = HashMap::new();
    for c in contents.chars() {
        frequency_map
            .entry(c)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    let mut heap = BinaryHeap::new();
    for (ch, freq) in frequency_map {
        let node = HuffmanNode::Leaf { ch, freq };
        heap.push(Reverse(node));
    }
    heap
}

fn build_code_map(head: &HuffmanNode, path: String, code_map: &mut HashMap<char, String>) {
    match head {
        HuffmanNode::Leaf { ch, .. } => {
            code_map.insert(*ch, path);
        }
        HuffmanNode::Internal { left, right, .. } => {
            build_code_map(left, format!("{path}0"), code_map);
            build_code_map(right, format!("{path}1"), code_map);
        }
    }
}

impl HuffmanTree {
    pub fn new(contents: &str) -> Self {
        let mut frequency_map = get_frequency_map(contents);
        while frequency_map.len() > 1 {
            // 2 or more in the frequency_map
            let Reverse(left) = frequency_map.pop().unwrap();
            let Reverse(right) = frequency_map.pop().unwrap();

            let new_freq = left.freq() + right.freq();
            let parent = HuffmanNode::Internal {
                freq: new_freq,
                left: Box::new(left),
                right: Box::new(right),
            };

            frequency_map.push(Reverse(parent));
        }

        let mut code_map = HashMap::new();
        if let Some(Reverse(head)) = frequency_map.peek() {
            build_code_map(head, String::new(), &mut code_map);
        }
        Self {
            root: frequency_map.pop().map(|Reverse(head)| Box::new(head)),
            code_map,
        }
    }

    pub fn serialize_map(&self) -> HashMap<char, String> {}

    pub fn get_encoded(&self, to_be_encoded: &str) -> Result<String, HuffmanError> {
        if self.root.is_none() && to_be_encoded.is_empty() {
            return Ok(String::new());
        }

        let mut encode_string = String::new();
        for c in to_be_encoded.chars() {
            match self.code_map.get(&c) {
                Some(path) => encode_string += path,
                None => {
                    return Err(HuffmanError::EncodingError(format!(
                        "There was no {} found in encoding map",
                        c
                    )));
                }
            }
        }
        Ok(encode_string)
    }
}
