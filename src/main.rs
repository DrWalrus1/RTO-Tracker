use std::env;

const SKIP_REPORT_FLAG: &str = "--skip-report-flag";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut skip_report: bool = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            SKIP_REPORT_FLAG => skip_report = true,
            _ => {}
        }
    }
    println!("Skip report flag is detected: {}", skip_report);
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
