use regex::Regex;

pub enum CommandType {
    // expected command
    A, C, L,
    // unexpected command
    None
}

pub fn command_type(line: &str) -> CommandType {
    let regex_a_number = Regex::new(r"^@[1-9]+[0-9]*$").unwrap();
    let regex_a_symbol = Regex::new(r"^@[a-zA-Z]+[0-9a-zA-Z.]*$").unwrap();
    if regex_a_number.is_match(line) || regex_a_symbol.is_match(line) {
        return CommandType::A;
    }
    let regex_l = Regex::new(r"^\([a-zA-Z]+[0-9a-zA-Z.]*\)$").unwrap();
    if regex_l.is_match(line) {
        return CommandType::L;
    }
    let regex_c = Regex::new(r"^((A|M|D|MD|AM|AD|AMD)=)?(0|-?1|[DAM]|![DAM]|-[DAM]|[DAM]\+1|[AMD]-1|D[\+\-&|][AM]|[AM]-D)(;(JGT|JEQ|JGE|JLT|JNE|JLE|JMP))?$").unwrap();
    if regex_c.is_match(line) {
        return CommandType::C;
    }
    CommandType::None
}

pub fn symbol(line: &str) -> Option<&str> {
    match command_type(line) {
        CommandType::A => line.get(1..),
        CommandType::L => {
            let len = line.len();
            line.get(1..(len-1))
        },
        _ => None
    }
}

pub fn dest(line: &str) -> Option<&str> {
    match command_type(line) {
        CommandType::C => {
            if !line.contains('=') {
                None
            } else {
                line.split('=').collect::<Vec<&str>>().first().map(|v| *v)
            }
        },
        _ => Option::None
    }
}

pub fn comp(line: &str) -> Option<&str> {
    match command_type(line) {
        CommandType::C => {
            let line_without_jump: Option<&str> = line.split(';').collect::<Vec<&str>>().first().map(|v| *v);
            line_without_jump.map(|v| v.split('=').last()).unwrap()
        },
        _ => None
    }
}

pub fn jump(line: &str) -> Option<&str> {
    match command_type(line) {
        CommandType::C => {
            if !line.contains(';') {
                None
            } else {
                line.split(';').last()
            }
        },
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_type_should_return_type_A() {
        // number type
        assert!(matches!(command_type("@1000"), CommandType::A));
        assert!(matches!(command_type("@0002"), CommandType::None));
        // symbol type
        assert!(matches!(command_type("@abCD"), CommandType::A));
        assert!(matches!(command_type("@aa00"), CommandType::A));
        assert!(matches!(command_type("@Hoge.hoge"), CommandType::A));
        assert!(matches!(command_type("@1a00"), CommandType::None));
    }
    #[test]
    fn command_type_should_return_type_L() {
        assert!(matches!(command_type("(ABC)"), CommandType::L));
        assert!(matches!(command_type("(ABC000)"), CommandType::L));
        assert!(matches!(command_type("(ABC.abc)"), CommandType::L));
        assert!(matches!(command_type("(ABC.abc0)"), CommandType::L));
        assert!(matches!(command_type("(ABC_abc0)"), CommandType::None));
    }
    #[test]
    fn command_type_should_return_type_C() {
        assert!(matches!(command_type("M=1"), CommandType::C));
        assert!(matches!(command_type("M=0"), CommandType::C));
        assert!(matches!(command_type("D=M"), CommandType::C));
        assert!(matches!(command_type("D=D-A"), CommandType::C));
        assert!(matches!(command_type("D;JGT"), CommandType::C));
        assert!(matches!(command_type("M=D+M"), CommandType::C));
        assert!(matches!(command_type("M=M+1"), CommandType::C));
        assert!(matches!(command_type("0;JMP"), CommandType::C));

        assert!(matches!(command_type("B=A+M"), CommandType::None));
        assert!(matches!(command_type("A=A+M;JJJ"), CommandType::None));
    }
    #[test]
    fn symbol_should_return_symbol_for_type_A() {
        assert_eq!(symbol("@1000"), Some("1000"));
        assert_eq!(symbol("@ABC"), Some("ABC"));
        assert_eq!(symbol("@ABC.abc"), Some("ABC.abc"));
    }
    #[test]
    fn symbol_should_return_symbol_for_type_L() {
        assert_eq!(symbol("(LABEL)"), Some("LABEL"));
        assert_eq!(symbol("(LABEL.label)"), Some("LABEL.label"));
    }
    #[test]
    fn symbol_should_return_None_for_other_type() {
        assert_eq!(symbol("A=A+1"), None);
        assert_eq!(symbol("random character"), None);
    }
    #[test]
    fn dest_should_return_dest() {
        assert_eq!(dest("D=D+1"), Some("D"));
        assert_eq!(dest("AMD=D+A"), Some("AMD"));
        assert_eq!(dest("M=D+M"), Some("M"));
        assert_eq!(dest("M=D;JGT"), Some("M"));
        assert_eq!(dest("D=A+1;JLT"), Some("D"));

        assert_eq!(dest("D;JLE"), None);
        assert_eq!(dest("0;JMP"), None);
        assert_eq!(dest("@i"), None);
        assert_eq!(dest("random character"), None);
    }
    #[test]
    fn comp_should_return_comp() {
        assert_eq!(comp("D=D+M"), Some("D+M"));
        assert_eq!(comp("D=D+M;JGT"), Some("D+M"));
        assert_eq!(comp("D;JEQ"), Some("D"));
        assert_eq!(comp("M=M+1"), Some("M+1"));
        assert_eq!(comp("A=-1"), Some("-1"));
        
        assert_eq!(dest("@i"), None);
        assert_eq!(dest("random character"), None);
    }
    #[test]
    fn jump_should_return_jump() {
        assert_eq!(jump("0;JMP"), Some("JMP"));
        assert_eq!(jump("D;JGE"), Some("JGE"));
        assert_eq!(jump("D=-1;JEQ"), Some("JEQ"));

        assert_eq!(jump("D=A+1"), None);
        assert_eq!(jump("@i"), None);
        assert_eq!(dest("random character"), None);
    }
}