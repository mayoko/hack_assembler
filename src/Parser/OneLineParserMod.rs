use regex::Regex;

pub struct OneLineParser<'a> {
    line: &'a str
}

pub enum CommandType {
    // expected command
    A, C, L,
    // unexpected command
    None
}

impl OneLineParser<'_> {
    pub fn commandType(&self) -> CommandType {
        let regex_a = Regex::new(r"@[a-zA-Z]+[0-9a-zA-Z]*").unwrap();
        // if self.line.starts_with('@') {
        if regex_a.is_match(self.line) {
            return CommandType::A;
        }
        let regex_l = Regex::new(r"\(.+\)").unwrap();
        // if self.line.starts_with('(') && self.line.ends_with(')') {
        if regex_l.is_match(self.line) {
            return CommandType::L;
        }
        let regex_c = Regex::new(r"([ADM]=)?(.*)(;(JGT|JEQ|JGE|JLT|JNE|JLE|JMP))?").unwrap();
        if regex_c.is_match(self.line) {
            return CommandType::C;
        }
        CommandType::None
    }

    pub fn symbol(&self) -> Option<&str> {
        match self.commandType() {
            CommandType::A => self.line.get(1..),
            CommandType::L => {
                let len = self.line.len();
                self.line.get(1..(len-1))
            },
            _ => Option::None
        }
    }

    pub fn dest(&self) -> Option<&str> {
        match self.commandType() {
            CommandType::C => self.line.split('=').collect()[0]
            _ => Option::None
        }
    }

    pub fn comp(&self) -> Option<&str> {
        match self.commandType() {
            CommandType::C => self.line.split(';').rsplit('=').collect()[0]
            _ => Option::None
        }
    }

    pub fn jump(&self) -> Option<&str> {
        match self.commandType() {
            CommandType::C => self.line.rsplit(';').collect()[0]
            _ => Option::None
        }
    }
}