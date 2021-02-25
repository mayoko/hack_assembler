use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
mod OneLineParserMod;
use OneLineParserMod::{CommandType, OneLineParser};

struct Parser {
    line_iter: Lines<BufReader<File>>,
    current_line: String
}

impl Parser {
    fn new(path: &str) -> Self {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        Parser { 
            line_iter: reader.lines(),
            current_line: String::new()
        }
    }

    // move to the next line.
    // if parser reached to the last line or something goes wrong, 
    // it returns false. Other wise, it returns true.
    fn advance(&mut self) -> bool {
        match self.line_iter.next() {
            None => return false,
            Some(result) => match result {
                Ok(line) => {
                    self.current_line = line
                }
                _ => return false
            }
        }
        true
    }

    fn commandType(&self) -> CommandType {
        let one_line_parser = OneLineParser { line: &self.current_line };
        one_line_parser.commandType()
    }
    
    fn symbol(&self) -> Option<&str> {
        let one_line_parser = OneLineParser { line: &self.current_line };
        one_line_parser.symbol()
    }
    
    fn dest(&self) -> Option<&str> {
        let one_line_parser = OneLineParser { line: &self.current_line };
        one_line_parser.dest()
    }
    
    fn comp(&self) -> Option<&str> {
        let one_line_parser = OneLineParser { line: &self.current_line };
        one_line_parser.comp()
    }

    fn jump(&self) -> Option<&str> {
        let one_line_parser = OneLineParser { line: &self.current_line };
        one_line_parser.jump()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}