use std::{fs, io::{self, BufReader, BufRead}};

use crate::{image::Image, matrix::EdgeMatrix, transform::Transformer};

#[derive(Clone, Debug)]
pub struct Parser {
    image: Box<Image<500, 500>>,
    e: EdgeMatrix,
    t: Transformer
}

impl Parser {
    pub fn parse(file: fs::File) -> io::Result<()> {
        let reader = BufReader::new(file);

        let mut curr_command: Option<String> = None; 
        reader
            .lines()
            .map(|line| line.split('#')[0].split_whitespace()) // Strip off comments and split whitespace
            .flatten()
        Ok(())
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self { 
            image: Box::new(Image::new_flip("result".to_string(), true)), 
            e: Default::default(), 
            t: Default::default() 
        }
    }
}