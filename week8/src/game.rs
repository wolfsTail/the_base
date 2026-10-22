use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    WaitingForInput,
    CheckingGuess(char),
    Won,
    Lost,
}

impl GameState {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Won | Self::Lost)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundResult {
    Hit,
    Miss,
    Repeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WrongState(pub GameState);

impl fmt::Display for WrongState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "переход недопустим в состоянии {:?}", self.0)
    }
}

impl Error for WrongState {}

#[derive(Debug, Clone)]
pub struct Game {
    word: String,
    guessed: Vec<char>,
    attempts: u32,
    state: GameState,
}

impl Game {
    pub fn new(word: &str, attempts: u32) -> Self {
        let mut game = Self {
            word: word.to_lowercase(),
            guessed: Vec::new(),
            attempts,
            state: GameState::WaitingForInput,
        };
        game.state = game.settle();
        game
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn guess(&mut self, c: char) -> Result<(), WrongState> {
        match self.state {
            GameState::WaitingForInput => {
                self.state = GameState::CheckingGuess(c);
                Ok(())
            }
            other => Err(WrongState(other)),
        }
    }

    pub fn check(&mut self) -> Result<RoundResult, WrongState> {
        let GameState::CheckingGuess(c) = self.state else {
            return Err(WrongState(self.state));
        };

        let result = if self.guessed.contains(&c) {
            RoundResult::Repeat
        } else {
            self.guessed.push(c);
            if self.word.contains(c) {
                RoundResult::Hit
            } else {
                self.attempts -= 1;
                RoundResult::Miss
            }
        };

        self.state = self.settle();
        Ok(result)
    }

    fn settle(&self) -> GameState {
        if self.letters().all(|c| self.guessed.contains(&c)) {
            GameState::Won
        } else if self.attempts == 0 {
            GameState::Lost
        } else {
            GameState::WaitingForInput
        }
    }

    fn letters(&self) -> impl Iterator<Item = char> + '_ {
        self.word.chars().filter(|c| c.is_alphabetic())
    }

    pub fn summary(&self) -> String {
        self.word
            .chars()
            .map(|c| {
                if !c.is_alphabetic() || self.guessed.contains(&c) {
                    c
                } else {
                    '*'
                }
            })
            .collect()
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn guessed(&self) -> &[char] {
        &self.guessed
    }

    pub fn secret_word(&self) -> &str {
        &self.word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(g: &mut Game, c: char) -> RoundResult {
        g.guess(c).expect("автомат ждёт ввода");
        g.check().expect("буква ждёт проверки")
    }

    #[test]
    fn starts_waiting_for_input() {
        let g = Game::new("Paper", 3);
        assert_eq!(g.state(), GameState::WaitingForInput);
        assert_eq!(g.summary(), "*****");
        assert_eq!(g.attempts(), 3);
        assert!(g.guessed().is_empty());
    }

    #[test]
    fn guess_only_remembers_letter_until_check() {
        let mut g = Game::new("paper", 3);

        g.guess('a').unwrap();
        assert_eq!(g.state(), GameState::CheckingGuess('a'));
        assert_eq!(g.summary(), "*****", "до проверки данные не меняются");
        assert!(g.guessed().is_empty());

        assert_eq!(g.check(), Ok(RoundResult::Hit));
        assert_eq!(g.state(), GameState::WaitingForInput);
        assert_eq!(g.summary(), "*a***");
        assert_eq!(g.guessed(), ['a']);
    }

    #[test]
    fn game_choices_won_test() {
        let mut g = Game::new("paper", 3);
        assert_eq!(turn(&mut g, 'a'), RoundResult::Hit);
        assert_eq!(turn(&mut g, 'i'), RoundResult::Miss);
        assert_eq!(g.summary(), "*a***");
        assert_eq!(g.attempts(), 2);
        assert_eq!(g.state(), GameState::WaitingForInput);

        assert_eq!(turn(&mut g, 'p'), RoundResult::Hit);
        assert_eq!(turn(&mut g, 'e'), RoundResult::Hit);
        assert_eq!(
            g.state(),
            GameState::WaitingForInput,
            "одна буква ещё закрыта"
        );

        assert_eq!(turn(&mut g, 'r'), RoundResult::Hit);
        assert_eq!(g.state(), GameState::Won);
        assert_eq!(g.summary(), "paper");
        assert_eq!(g.attempts(), 2, "победа не тратит попытки");
    }

    #[test]
    fn game_choices_lose_test() {
        let mut g = Game::new("paper", 3);
        assert_eq!(turn(&mut g, 'a'), RoundResult::Hit);
        assert_eq!(turn(&mut g, 'i'), RoundResult::Miss);
        assert_eq!(g.summary(), "*a***");
        assert_eq!(g.attempts(), 2);

        assert_eq!(turn(&mut g, 'a'), RoundResult::Repeat);
        assert_eq!(g.attempts(), 2, "повтор не тратит попытку");
        assert_eq!(g.guessed(), ['a', 'i'], "повтор не записывается дважды");

        assert_eq!(turn(&mut g, 'd'), RoundResult::Miss);
        assert_eq!(
            g.state(),
            GameState::WaitingForInput,
            "последняя попытка ещё есть"
        );

        assert_eq!(turn(&mut g, 't'), RoundResult::Miss);
        assert_eq!(g.state(), GameState::Lost);
        assert_eq!(g.attempts(), 0);
        assert_eq!(g.secret_word(), "paper");
    }

    #[test]
    fn guess_is_rejected_unless_waiting() {
        let mut g = Game::new("paper", 3);
        g.guess('a').unwrap();

        assert_eq!(g.guess('b'), Err(WrongState(GameState::CheckingGuess('a'))));
        assert_eq!(
            g.state(),
            GameState::CheckingGuess('a'),
            "отвергнутый переход ничего не меняет"
        );
    }

    #[test]
    fn check_is_rejected_unless_checking() {
        let mut g = Game::new("paper", 3);
        assert_eq!(g.check(), Err(WrongState(GameState::WaitingForInput)));
        assert_eq!(g.state(), GameState::WaitingForInput);
        assert_eq!(g.attempts(), 3);
    }

    #[test]
    fn terminal_states_accept_no_transitions() {
        let mut won = Game::new("a", 1);
        turn(&mut won, 'a');
        assert_eq!(won.state(), GameState::Won);
        assert!(won.state().is_terminal());
        assert_eq!(won.guess('b'), Err(WrongState(GameState::Won)));
        assert_eq!(won.check(), Err(WrongState(GameState::Won)));
        assert_eq!(won.state(), GameState::Won);

        let mut lost = Game::new("a", 1);
        turn(&mut lost, 'z');
        assert_eq!(lost.state(), GameState::Lost);
        assert!(lost.state().is_terminal());
        assert_eq!(lost.guess('a'), Err(WrongState(GameState::Lost)));
        assert_eq!(lost.check(), Err(WrongState(GameState::Lost)));
    }

    #[test]
    fn zero_attempts_is_lost_from_the_start() {
        let g = Game::new("paper", 0);
        assert_eq!(g.state(), GameState::Lost);
    }

    #[test]
    fn word_is_case_insensitive() {
        let mut g = Game::new("PaPeR", 3);
        assert_eq!(turn(&mut g, 'p'), RoundResult::Hit);
        assert_eq!(g.summary(), "p*p**");
    }

    #[test]
    fn non_letters_are_open_and_not_required() {
        let mut g = Game::new("ice-cream", 1);
        assert_eq!(g.summary(), "***-*****");

        for c in "iceram".chars() {
            assert_eq!(turn(&mut g, c), RoundResult::Hit);
        }
        assert_eq!(g.state(), GameState::Won, "дефис угадывать не нужно");
    }

    #[test]
    fn wrong_state_is_a_proper_error() {
        let err: Box<dyn Error> = Box::new(WrongState(GameState::Won));
        assert_eq!(err.to_string(), "переход недопустим в состоянии Won");
    }
}
