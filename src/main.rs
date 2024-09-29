use std::env;

pub mod domain;

use chrono::Datelike;

const SKIP_REPORT_FLAG: &str = "--skip-report-flag";
const TEST_FLAG: &str = "--test";

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
    let mut test_flag: bool = false;

    let args_iter = args.iter().peekable();
    for i in args_iter {
        match i.as_str() {
            SKIP_REPORT_FLAG => skip_report = true,
            TEST_FLAG => test_flag = true,
            _ => {}
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
}
