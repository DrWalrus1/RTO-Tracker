use crate::in_office_states::*;
use chrono::Datelike;
use core::fmt;
use std::{
    env,
    error::Error,
    fmt::{Display, Formatter},
    io::{stdin, stdout, Write},
};

pub mod in_office_states;
const SKIP_REPORT_FLAG: &str = "--skip-report-flag";

fn get_day_suffix(day: u32) -> &'static str {
    match day {
        1 | 21 | 31 => "st",
        2 | 22 => "nd",
        3 | 23 => "rd",
        _ => "th",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut skip_report: bool = false;

    let args_iter = args.iter().peekable();
    for i in args_iter {
        if let SKIP_REPORT_FLAG = i.as_str() {
            skip_report = true
        }
    }
    // E.g. Today's date: Monday 5th August 2024
    let date = chrono::offset::Local::now();
    let day = format!("{}{}", date.day(), get_day_suffix(date.day()));
    let weekday = date.weekday().to_string() + "day";
    let month = chrono::Month::try_from(u8::try_from(date.month()).unwrap()).unwrap();
    println!(
        "Today's date: {}, {} {} {}",
        weekday,
        day,
        month.name(),
        date.year()
    );
    let mut line = String::new();
    let stdin = stdin();
    println!("Are you in the office today?");
    print!("Avaliable options: (Working from [H]ome/In [O]ffice/On [L]eave/Using [E]xemption) ");
    stdout().flush().unwrap();
    stdin.read_line(&mut line).unwrap();
    let parsed_status = parse_input_into(&line);
    if let Ok(parsed_status) = &parsed_status {
        println!("{}", parsed_status);
    }
    if parsed_status.is_err() {
        match parsed_status.err().unwrap() {
            Some(parse_error) => println!("{}", parse_error),
            None => println!("Nothing inputed. Exiting..."),
        };
    }
}

#[derive(Debug)]
struct ParseError {
    pub error_message: String,
}
impl Error for ParseError {}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.error_message)
    }
}

fn parse_input_into(input: &str) -> Result<InOfficeStates, Option<ParseError>> {
    match input.trim() {
        "H" => Ok(InOfficeStates::Wfh),
        "O" => Ok(InOfficeStates::InOffice),
        "L" => Ok(InOfficeStates::OnLeave),
        "E" => Ok(InOfficeStates::UsingExemption),
        "" => Err(None),
        _ => Err(Some(ParseError {
            error_message: String::from("Invalid input, not valid"),
        })),
    }
}

/*
 *
 * 1. Check for crontab job
 * 2. Check for local share database ( sort it into 1mb files... or into maybe months)
 * 3. Check for Today's Date
 * 4. Check for missing entries
 * 5. If there are missing entries. ask to fill out, mark all as in office/wfh/on leave/exemption/ignore
 * 6. Ask what today's entry is. (office/wfh/on leave/exemption/ignore)
 * 7. maybe ask if they want a report? or maybe leave it for a separate menu
 *
 * executable flag to skip report question??
 */
