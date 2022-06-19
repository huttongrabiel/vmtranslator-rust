pub fn lex(line_contents: &str) -> Vec<&str> {
    trim(line_contents)
        .split_whitespace()
        .filter(|line| {
            is_lexable_line(line)
        })
        .collect()
}

fn is_lexable_line(line_contents: &str) -> bool {
    match line_contents.get(0..2) {
        Some(contents) => {
            if contents == "//" {
                return false;
            }
        },
        None => (),
    }

    if !line_contents.is_ascii() {
        return false;
    }

    let blank_line: String = line_contents
                    .chars()
                    .filter(|ch| ch.is_ascii() && !ch.is_ascii_whitespace())
                    .collect();

    if blank_line.is_empty() {
        return false;
    }

    true
}

fn trim(line: &str) -> &str {
    match line.find("//") {
        Some(index) => {
            &line[..index]
        },
        None => {
            line
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexable_line() {
        let l1 = "// This is a comment line";
        assert!(!is_lexable_line(l1));

        let l2 = "push constant 2";
        assert!(is_lexable_line(l2));

        let l3 = "push constant 2 // This is a comment";
        assert!(is_lexable_line(l3));

        let l4 = "";
        assert!(!is_lexable_line(l4));

        let l5 = "\n";
        assert!(!is_lexable_line(l5));
    }

    #[test]
    fn lex_line() {
        let l1 = "push constant 2";
        assert_eq!(lex(l1), vec!["push", "constant", "2"]);

        let l2 = "pop static 8\n\n";
        assert_eq!(lex(l2), vec!["pop", "static", "8"]);

        let l3 = "push local 12 // This is a comment";
        assert_eq!(lex(l3), vec!["push", "local", "12"]);
    }

    #[test]
    fn trim_line() {
        let l1 = "push local 12 // This is a comment";
        assert_eq!(trim(l1), "push local 12 ");
    }
}
