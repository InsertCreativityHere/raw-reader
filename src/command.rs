
use crate::pattern::{BytePattern, StringPattern};
use std::convert::TryFrom;
use std::str::FromStr;
use std::num::{IntErrorKind, ParseIntError};

/// TODO
#[derive(Debug)]
pub enum Command {
    Seek(Seek),
    Find(Find),
    Print(Print),
    Config(Config),
    Help(Help),
    Exit,
    None,
}

impl FromStr for Command {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get the first token in the string; this token specifies the command to run.
        let Some((command, remainder)) = split_at_first_token(s) else {
            return Ok(Command::None);
        };

        // Compare the token against a list of commands, then parse the rest of the string accordingly.
        match command.to_lowercase().as_str() {
            "seek"   => remainder.parse::<Seek>().map(Command::Seek),
            "find"   => remainder.parse::<Find>().map(Command::Find),
            "print"  => remainder.parse::<Print>().map(Command::Print),
            "config" => remainder.parse::<Config>().map(Command::Config),
            "help"   => remainder.parse::<Help>().map(Command::Help),
            "exit"   => {
                reject_additional_tokens(remainder, "help")?;
                Ok(Command::Exit)
            }
            unknown  => Err(format!("Unknown command: '{unknown}'. Enter 'help' for a list of commands.")),
        }
    }
}

/// TODO
#[derive(Debug)]
pub enum Seek {
    Absolute(i64),
    Relative(i64),
}

impl FromStr for Seek {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The next token in the string describes the seek mode. Return an error if it's missing.
        let Some((mode, arguments)) = split_at_first_token(s) else {
            return Err("Missing seek mode: 'absolute' or 'relative'. Enter 'help seek' for an example.".to_owned());
        };

        // The last token in the string should be the offset/position to seek to. We check for the token,
        // and parse it as an integer if it's present. If it's missing, return an error.
        let Some((raw_integer, extra)) = split_at_first_token(arguments) else {
            return Err("Missing offset/position to seek to. Enter 'help seek' for an explanation".to_owned());
        };
        let integer = raw_integer.parse::<i64>().map_err(|err| {
            format!("invalid offset/position: '{raw_integer}' {}.", get_explanation_for(err))
        })?;

        // Return an error if there's any tokens left in the string.
        reject_additional_tokens(extra, "help seek")?;

        // Construct a `Seek` with the specified mode and offset/position.
        // Or report an error if an invalid seek mode was specified.
        match mode.to_lowercase().as_str() {
            "absolute" => Ok(Seek::Absolute(integer)),
            "relative" => Ok(Seek::Relative(integer)),
            unknown => Err(format!("Unknown seek mode: '{unknown}'. Enter 'help seek' for a list of seek modes.")),
        }
    }
}

/// TODO
#[derive(Debug)]
pub enum Find {
    NonZero,
    Byte(BytePattern),
    String(StringPattern),
}

impl FromStr for Find {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get the next token in the string; this token specifies the find mode. Return an error if it's missing.
        let Some((mode, remainder)) = split_at_first_token(s) else {
            return Err("Missing find mode: 'nonzero', 'bytes', or 'string'. Enter 'help find' for an example.".to_owned());
        };

        // Compare the token against a list of find modes, then parse the rest of the string accordingly.
        match mode.to_lowercase().as_str() {
            "nonzero" => {
                reject_additional_tokens(remainder, "help find nonzero")?;
                Ok(Find::NonZero)
            }
            "bytes" => remainder.parse::<BytePattern>().map(Find::Byte),
            "string" => remainder.parse::<StringPattern>().map(Find::String),
            unknown => Err(format!("unknown find mode: '{unknown}'. Enter 'help find' for a list of find modes.'"))
        }
    }
}

/// TODO
#[derive(Debug)]
pub struct Print(u64);

impl FromStr for Print {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The next (and last) token in the string should be the number of bytes to print. We check
        // for the token and parse it as an integer if it's present. If it's missing, return an error.
        let Some((raw_integer, extra)) = split_at_first_token(s) else {
            return Err("Missing number of bytes to print. Enter 'help print' for an example.".to_owned());
        };
        let integer = raw_integer.parse::<i64>().map_err(|err| {
            format!("Invalid number of bytes: '{raw_integer}' {}.", get_explanation_for(err))
        })?;
        let positive_integer = u64::try_from(integer).map_err(|_| {
            "The number of bytes to print must be non-negative.".to_owned()
        })?;

        // Return an error if there's any tokens left in the string.
        reject_additional_tokens(extra, "help print")?;

        Ok(Print(positive_integer))
    }
}

/// TODO
#[derive(Debug)]
// TODO ADD CONFIG OPTIONS.
pub enum Config {}

impl FromStr for Config {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO ADD CONFIG OPTIONS.
        Err("no options".to_owned())
    }
}

/// TODO
#[derive(Debug)]
pub enum Help {
    None,
    Seek,
    SeekAbsolute,
    SeekRelative,
    Find,
    FindNonZero,
    FindByte,
    FindString,
    Print,
    Config,
}

impl FromStr for Help {
    type Err = String;

    /// TODO
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO COMMENT THIS FUNCTION!
        todo!()
    }
}

/// TODO
fn split_at_first_token(raw_input: &str) -> Option<(&str, &str)> {
    // Remove any whitespace from the start of the input.
    let input = raw_input.trim_start();

    // If the input was empty (except whitespace), return `None` since there aren't any tokens.
    if input.is_empty() {
        return None;
    }

    // If the string contains whitespace (after removing any leading whitespace), split the string
    // into 2 pieces: any characters before that whitespace, and everything after that whitespace.
    //
    // If the string doesn't contain whitespace (after removing any leading whitespace), return the
    // string as-is for the first token, and the empty string as the remainder.
    Some(if input.contains(char::is_whitespace) {
        // Safe to unwrap because we know the string contains non-whitespace characters
        // (because we trimmed it and it wasn't empty), followed by whitespace characters
        // (because we just checked if it contains whitespace). So this will always succeed.
        input.split_once(char::is_whitespace).unwrap()
    } else {
        (input, "")
    })
}

/// TODO
fn get_explanation_for(error: ParseIntError) -> &'static str {
    match error.kind() {
        IntErrorKind::Empty => unreachable!("Attempted to parse an empty string!"),
        IntErrorKind::InvalidDigit => "is not a valid number",
        IntErrorKind::PosOverflow => "is too large and overflowed",
        IntErrorKind::NegOverflow => "is too small and overflowed",
        IntErrorKind::Zero => "cannot be 0",
        _ => unreachable!("I need to update these match arms because Rust added a new variant!"),
    }
}

/// TODO
fn reject_additional_tokens(remainder: &str, help: &str) -> Result<(), String> {
    let extra = remainder.trim();
    if !extra.is_empty() {
        Err(format!("Unexpected extra parameter: '{extra}'. Enter '{help}' for an explanation."))
    } else {
        Ok(())
    }
}

// TODO ADD UNIT TESTS!