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
    count: Option<u8>,
}

impl Guess {
    fn new(word: String) -> Guess {
        Guess { word, count: None }
    }

    fn new_with_count(word: String, count: u8) -> Guess {
        Guess {
            word,
            count: Some(count),
        }
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
            let count: u8 = count_str
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
enum Error {
    ParseGuess(ParseGuessError),
    IO(io::Error),
}

impl From<ParseGuessError> for Error {
    fn from(err: ParseGuessError) -> Self {
        Error::ParseGuess(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref err) => write!(f, "IO error: {}", err),
            Error::ParseGuess(ref err) => write!(f, "parsing guess error: {}", err),
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

    Ok(guesses)
}

fn run() -> Result<()> {
    let guesses = parse_guesses_from_stdin()?;

    println!("guesses {:?}", guesses);

    Ok(())
}

fn main() {
    match run() {
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
        _ => {}
    };
}
