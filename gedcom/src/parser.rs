use crate::tokenizer::Tokenizer;

#[derive(Debug)]
pub struct Parser<'a> {
    content: &'a str,
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            tokenizer: Tokenizer::new(content),
        }
    }

    pub fn parse(&mut self) {
        // WIP
        while let Some(line) = self.tokenizer.next_line() {
            println!("{:?}", line);
        }
    }
}
