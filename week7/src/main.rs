use hangman::{Game, ParseError, RoundResult, parse_input};
use std::io::{self, Write};

const SECRET: &str = "привет";
const ATTEMPTS: u32 = 3;

fn main() {
    let mut game = Game::new(SECRET, ATTEMPTS);
    let mut line = String::new();

    println!("Отгадай слово по буквам. Права на ошибку: {ATTEMPTS}.");

    loop {
        print!(
            "\nСлово: {}   попыток: {}\nБуква> ",
            game.summary(),
            game.attempts()
        );
        io::stdout().flush().expect("не удалось вывести текст");

        line.clear();
        if io::stdin().read_line(&mut line).expect("ошибка чтения") == 0 {
            println!("\nВвод закрыт, игра прервана.");
            return;
        }

        let c = match parse_input(&line) {
            Ok(c) => c,
            Err(e) => {
                println!("{}", error_text(&e));
                continue;
            }
        };

        match game.add(c) {
            RoundResult::Hit => println!("Угадал букву «{c}»!"),
            RoundResult::Miss => println!("Буквы «{c}» нет."),
            RoundResult::Repeat => println!("Увы, «{c}» уже была."),
        }

        if game.is_won() || game.is_lost() {
            break;
        }
    }

    let won = game.is_won();
    let word = game.secret_word();
    if won {
        println!("\nПобеда! Слово: {word}");
    } else {
        println!("\nПопытки закончились. Загаданное слово: {word}");
    }
}

fn error_text(e: &ParseError) -> &'static str {
    match e {
        ParseError::Empty => "Пустой ввод!",
        ParseError::NotALetter => "Это не буква!!!",
        ParseError::TooManySymbols => "Введи ровно одну (1) букву",
    }
}
