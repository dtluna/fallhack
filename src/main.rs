#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    io::{self, prelude::*, stdin},
    process::exit,
    result,
    vec::Vec,
};

#[derive(Debug)]
struct Guess {
    word: String,
    count: Option<usize>,
}

impl Guess {
    fn new(word: String) -> Guess {
        Guess { word, count: None }
    }

    fn new_with_count(word: String, count: usize) -> Guess {
        Guess {
            word,
            count: Some(count),
        }
    }

    fn num_of_common_letters(&self, other: &Guess) -> usize {
        let mut num: usize = 0;

        for (index, letter) in self.word.char_indices() {
            let other_letter: char = other.word.as_bytes()[index].into();
            if letter == other_letter {
                // we've checked that words have equal lengths already
                num += 1;
            }
        }

        num
    }
}

impl TryFrom<&str> for Guess {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self> {
        lazy_static! {
            static ref GUESS_REGEX: Regex =
                Regex::new(r"(?P<word>[[:alpha:]]+)[[:space:]]*(?P<count>[[:digit:]]*)")
                    .expect("could not compile regexp");
        }

        let captures = match GUESS_REGEX.captures(&line) {
            None => {
                return Err(ParseGuessError {
                    line: line.into(),
                    detail: "wrong guess format".into(),
                }
                .into())
            }
            Some(captures) => captures,
        };

        let word = captures
            .name("word")
            .expect("word should have been successfully captured by regex")
            .as_str();

        let count_str = captures
            .name("count")
            .expect("count should have been successfully captured by regex")
            .as_str();

        if count_str.len() > 0 {
            let count: usize = count_str
                .parse()
                .expect("the regex should not allow this to fail");

            if usize::from(count) > word.len() {
                return Err(ParseGuessError {
                    line: line.into(),
                    detail: "count is longer than the word".into(),
                }
                .into());
            }

            Ok(Guess::new_with_count(word.into(), count))
        } else {
            Ok(Guess::new(word.into()))
        }
    }
}

#[derive(Debug)]
struct ParseGuessError {
    line: String,
    detail: String,
}

impl fmt::Display for ParseGuessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cannot parse line \"{}\" into Guess: {}",
            self.line, self.detail
        )
    }
}

#[derive(Debug)]
struct NoGuessesError {}

impl fmt::Display for NoGuessesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "no guesses in input",)
    }
}

#[derive(Debug)]
struct UnequalLengthsError {}

impl fmt::Display for UnequalLengthsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "guess words have unequal lengths",)
    }
}

#[derive(Debug)]
enum Error {
    ParseGuess(ParseGuessError),
    IO(io::Error),
    NoGuesses(NoGuessesError),
    UnequalLengths(UnequalLengthsError),
}

impl From<ParseGuessError> for Error {
    fn from(err: ParseGuessError) -> Self {
        Self::ParseGuess(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<NoGuessesError> for Error {
    fn from(err: NoGuessesError) -> Self {
        Self::NoGuesses(err)
    }
}

impl From<UnequalLengthsError> for Error {
    fn from(err: UnequalLengthsError) -> Self {
        Self::UnequalLengths(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref err) => write!(f, "IO error: {}", err),
            Error::ParseGuess(ref err) => write!(f, "parsing guess error: {}", err),
            Error::NoGuesses(ref err) => write!(f, "{}", err),
            Error::UnequalLengths(ref err) => write!(f, "{}", err),
        }
    }
}

type Result<T> = result::Result<T, Error>;

fn parse_guesses_from_stdin() -> Result<Vec<Guess>> {
    let mut guesses: Vec<Guess> = Vec::new();

    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    for line in buffer.lines() {
        let guess: Guess = line.try_into()?;
        guesses.push(guess);
    }

    if guesses.len() == 0 {
        return Err(NoGuessesError {}.into());
    }

    let len = guesses
        .get(0)
        .expect("we checked for length above")
        .word
        .len();

    for guess in guesses.iter() {
        if guess.word.len() != len {
            return Err(UnequalLengthsError {}.into());
        }
    }

    Ok(guesses)
}

fn run() -> Result<()> {
    let guesses = parse_guesses_from_stdin()?;

    let guesses_with_count: Vec<&Guess> = guesses
        .iter()
        .filter(|guess| guess.count.is_some())
        .collect();
    let guesses_without_count = guesses.iter().filter(|guess| guess.count.is_none());

    let filtered_guesses = guesses_without_count.filter(|guess_without_count| {
        guesses_with_count.iter().all(|guess_with_count| {
            guess_with_count.num_of_common_letters(guess_without_count)
                == guess_with_count.count.unwrap()
        })
    });

    for filtered_guess in filtered_guesses {
        println!("{}", filtered_guess.word);
    }

    Ok(())
}

fn main() {
    match run() {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1)
        }
        _ => {}
    };
}
