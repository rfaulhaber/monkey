#[derive(Debug, Clone)]
pub(super) struct WordIter<'a> {
    input: &'a str,
    pos: usize,
    character: Option<char>,
}

impl<'a> WordIter<'a> {
    pub fn new(input: &'a str) -> WordIter<'a> {
        WordIter {
            input,
            pos: 0,
            character: None,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        if self.pos <= self.input.len() {
            self.character = self.input.chars().nth(self.pos);
            self.pos = self.pos + 1;
            self.character
        } else {
            None
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.pos <= self.input.len() {
            self.input.chars().nth(self.pos)
        } else {
            None
        }
    }
}

impl<'a> Iterator for WordIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.read_char() {
                Some(c) if is_symbol(c) => match c {
                    '=' | '!' => match self.peek_next() {
                        Some('=') => {
                            self.read_char();
                            break Some(vec![c, '='].iter().collect());
                        }
                        _ => break Some(c.to_string()),
                    },
                    _ => break Some(c.to_string()),
                },
                Some(c) if c.is_alphanumeric() => {
                    let mut res = vec![c];

                    // TODO ugly, refactor
                    if let Some(next_c) = self.peek_next() {
                        if next_c.is_whitespace() || is_symbol(next_c) {
                            break Some(res.into_iter().collect());
                        }
                    }

                    let mut ch = self.read_char();

                    while let Some(c) = ch {
                        res.push(c);

                        if let Some(next_c) = self.peek_next() {
                            if next_c.is_whitespace() || is_symbol(next_c) {
                                break;
                            }
                        }

                        ch = self.read_char();
                    }

                    break Some(res.iter().collect());
                }
                Some(c) if c.is_whitespace() => continue,
                _ => break None,
            }
        }
    }
}

fn is_symbol(c: char) -> bool {
    match c {
        '{' | '}' | '(' | ')' | '+' | ',' | ';' | '=' | '-' | '!' | '*' | '/' | '<' | '>' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_iter_breaks_up_symbols() {
        let input = "=+(){},;";

        let expected = vec!["=", "+", "(", ")", "{", "}", ",", ";"];

        let res: Vec<String> = WordIter::new(input).collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn word_iter_recognizes_extended_symbols() {
        let input = "!-/*<5>";

        let expected = vec!["!", "-", "/", "*", "<", "5", ">"];

        let res: Vec<String> = WordIter::new(input).collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn word_iter_recognizes_keywords() {
        let input = "fn let true false if else return";

        let expected = vec!["fn", "let", "true", "false", "if", "else", "return"];

        let res: Vec<String> = WordIter::new(input).collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn word_iter_recognizes_multi_char_operators() {
        let input = "!= ==";

        let expected = vec!["!=", "=="];

        let res: Vec<String> = WordIter::new(input).collect();

        assert_eq!(expected, res);
    }

    #[test]
    fn word_iter_lexes_words() {
        let input = "let five_hundred = 500;";

        let expected = vec!["let", "five_hundred", "=", "500", ";"];

        let res: Vec<String> = WordIter::new(input).collect();

        assert_eq!(expected, res);
    }
}
