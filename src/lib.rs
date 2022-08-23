#[cfg(test)]
mod tests {
	use crate::smiley_line;

    #[test]
    fn simple() {
        assert_eq!(smiley_line(0), "");
        assert_eq!(smiley_line(1), ")");
        assert_eq!(smiley_line(2), ":)");
        assert_eq!(smiley_line(3), ":-)");
    }
}

pub fn smiley_line(length: u64) -> String {
    let mut sline: String = "".to_owned();
    if length == 1 {
        return ")".to_owned();
    }
    let mut l = length;
    while l > 1 {
        if l % 9 == 0 {
            sline += ":-):-):-)";
            l -= 9;
        } else if l % 5 == 0 {
            sline += ":-):)";
            l -= 5;
        } else if l % 4 == 0 {
            sline += ":):)";
            l -= 4;
        } else if l % 3 == 0 {
            sline += ":-)";
            l -= 3;
        } else if l % 2 == 0 {
            sline += ":)";
            l -= 2;
        } else if l % 2 != 0 {
            sline += ":-)";
            l -= 3;
        }
    }
    sline
}
