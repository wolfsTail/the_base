#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    NotALetter,
    TooManySymbols,
}

pub fn parse_input(inp: &str) -> Result<char, ParseError> {
    let s = inp.trim();
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (None, _) => Err(ParseError::Empty),
        (Some(c), None) if c.is_alphabetic() => Ok(c.to_lowercase().next().unwrap()),
        (Some(_), None) => Err(ParseError::NotALetter),
        (Some(_), Some(_)) => Err(ParseError::TooManySymbols),
    }
}

#[derive(Debug, PartialEq)]
pub enum RoundResult {
    Hit,
    Miss,
    Repeat,
}

pub struct Game {
    word: String,
    chars: Vec<char>,
    attempts: u32,
}

impl Game {
    pub fn new(word: &str, attempts: u32) -> Self {
        Self {
            word: word.to_lowercase(),
            chars: vec![],
            attempts,
        }
    }
    pub fn add(&mut self, c: char) -> RoundResult {
        if self.chars.contains(&c) {
            return RoundResult::Repeat;
        }
        self.chars.push(c);

        if self.word.contains(c) {
            RoundResult::Hit
        } else {
            self.attempts = self.attempts.saturating_sub(1);
            RoundResult::Miss
        }
    }
    pub fn is_won(&self) -> bool {
        self.word.chars().all(|c| self.chars.contains(&c))
    }
    pub fn is_lost(&self) -> bool {
        self.attempts == 0
    }
    pub fn summary(&self) -> String {
        self.word
            .chars()
            .map(|c| if self.chars.contains(&c) { c } else { '*' })
            .collect()
    }
    pub fn attempts(&self) -> u32 {
        self.attempts
    }
    pub fn secret_word(self) -> String {
        self.word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        assert_eq!(parse_input("a\n"), Ok('a'));
        assert_eq!(parse_input("  a "), Ok('a'));
        assert_eq!(parse_input(""), Err(ParseError::Empty));
        assert_eq!(parse_input("123"), Err(ParseError::TooManySymbols));
        assert_eq!(parse_input("1"), Err(ParseError::NotALetter));
    }
    #[test]
    fn game_choices_won_test() {
        let mut g = Game::new("paper", 3);
        assert_eq!(g.add('a'), RoundResult::Hit);
        assert_eq!(g.add('i'), RoundResult::Miss);
        assert_eq!(g.summary(), "*a***".to_owned());
        assert_eq!(g.attempts(), 2);
        assert_eq!(g.is_lost(), false);
        assert_eq!(g.is_won(), false);
        assert_eq!(g.add('p'), RoundResult::Hit);
        assert_eq!(g.add('e'), RoundResult::Hit);
        assert_eq!(g.add('r'), RoundResult::Hit);
        assert_eq!(g.is_won(), true);
        assert_eq!(g.attempts(), 2);
    }
    #[test]
    fn game_choices_lose_test() {
        let mut g = Game::new("paper", 3);
        assert_eq!(g.add('a'), RoundResult::Hit);
        assert_eq!(g.add('i'), RoundResult::Miss);
        assert_eq!(g.summary(), "*a***".to_owned());
        assert_eq!(g.attempts(), 2);
        assert_eq!(g.is_lost(), false);
        assert_eq!(g.is_won(), false);
        assert_eq!(g.add('a'), RoundResult::Repeat);
        assert_eq!(g.add('d'), RoundResult::Miss);
        assert_eq!(g.add('t'), RoundResult::Miss);
        assert_eq!(g.is_won(), false);
        assert_eq!(g.is_lost(), true);
        assert_eq!(g.attempts(), 0);
    }
}
