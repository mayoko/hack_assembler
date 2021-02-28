use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
mod OneLineParser;
use OneLineParser::{CommandType, command_type, comp, dest, jump, symbol};

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

    fn command_type(&self) -> CommandType {
        command_type(&self.current_line)
    }
    
    fn symbol(&self) -> Option<&str> {
        symbol(&self.current_line)
    }
    
    fn dest(&self) -> Option<&str> {
        dest(&self.current_line)
    }
    
    fn comp(&self) -> Option<&str> {
        comp(&self.current_line)
    }

    fn jump(&self) -> Option<&str> {
        jump(&self.current_line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}