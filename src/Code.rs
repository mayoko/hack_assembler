struct Code {
}

impl Code {
    fn dest(&self, mnemonic: &str) -> u32 {
        let mut ret = 0;
        if mnemonic.contains('A') {
            ret |= 4;
        }
        if mnemonic.contains('D') {
            ret |= 2;
        }
        if mnemonic.contains('M') {
            ret |= 1;
        }
        return ret;
    }
    fn comp(&self, mnemonic: &str) -> u32 {
        let use_m = mnemonic.contains('M');
        let mnemonic_a = mnemonic.replace('M', "A");
        let comp_string: &str = if mnemonic_a == "0" {
            "101010"
        } else if mnemonic_a == "1" {
            "111111"
        } else if mnemonic_a == "-1" {
            "111010"
        } else if mnemonic_a == "D" {
            "001100"
        } else if mnemonic_a == "A" {
            "110000"
        } else if mnemonic_a == "!D" {
            "001101"
        } else if mnemonic_a == "!A" {
            "110001"
        } else if mnemonic_a == "-D" {
            "001111"
        } else if mnemonic_a == "-A" {
            "110011"
        } else if mnemonic_a == "D+1" {
            "011111"
        } else if mnemonic_a == "A+1" {
            "110111"
        } else if mnemonic_a == "D-1" {
            "001110"
        } else if mnemonic_a == "A-1" {
            "110010"
        } else if mnemonic_a == "D+A" {
            "000010"
        } else if mnemonic_a == "D-A" {
            "010011"
        } else if mnemonic_a == "A-D" {
            "000111"
        } else if mnemonic_a == "D&A" {
            "000000"
        } else if mnemonic_a == "D|A" {
            "010101"
        } else {
            "000000"
        };
        let mut ans = if use_m { 64 } else { 0 };
        let mut plus = 32;
        for c in comp_string.chars() {
            if c == '1' {
                ans += plus;
            }
            plus /= 2;
        }
        return ans;
    }
    fn jump(&self, mnemonic: &str) -> u32 {
        if mnemonic == "null" {
            return 0;
        }
        if mnemonic == "JNE" {
            return 5;
        }
        if mnemonic == "JMP" {
            return 7;
        }
        let mut ret = 0;
        if mnemonic.contains('G') {
            ret |= 1;
        }
        if mnemonic.contains('E') {
            ret |= 2;
        }
        if mnemonic.contains('L') {
            ret |= 4;
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_should_return_correct_binary() {
        let code = Code {};
        assert!(code.jump("") == 0);
        assert!(code.jump("JGT") == 1);
        assert!(code.jump("JEQ") == 2);
        assert!(code.jump("JGE") == 3);
        assert!(code.jump("JLT") == 4);
        assert!(code.jump("JNE") == 5);
        assert!(code.jump("JLE") == 6);
        assert!(code.jump("JMP") == 7);
    }

    #[test]
    fn comp_should_return_correct_binary() {
        let code = Code {};
        assert_eq!(code.comp("D"), 12);
        assert_eq!(code.comp("A"), 48);
        assert_eq!(code.comp("M"), 112);
        assert_eq!(code.comp("A+1"), 55);
        assert_eq!(code.comp("D|M"), 85);
    }

    #[test]
    fn dest_should_return_correct_binary() {
        let code = Code {};
        assert!(code.dest("") == 0);
        assert!(code.dest("A") == 4);
        assert!(code.dest("D") == 2);
        assert!(code.dest("M") == 1);
        assert!(code.dest("AD") == 6);
        assert!(code.dest("AM") == 5);
        assert!(code.dest("DM") == 3);
        assert!(code.dest("ADM") == 7);
    }
}
