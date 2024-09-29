use std::fmt::Display;

pub enum Status {
    InOffice,
    WorkingFromHome,
}

pub impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "Working From Home" | "WFH" | _ => return Status::WorkingFromHome,
        }
    }
}

pub impl Display for Status {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::WorkingFromHome => formatter.write_str("Working from home"),
            Status::InOffice => formatter.write_str("In office"),
        }
    }
}
