use std::fmt::{self, Display, Formatter};

pub enum InOfficeStates {
    Wfh,
    InOffice,
    OnLeave,
    UsingExemption,
}

impl Display for InOfficeStates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InOfficeStates::Wfh => write!(f, "Work from Home"),
            InOfficeStates::InOffice => write!(f, "In Office"),
            InOfficeStates::OnLeave => write!(f, "On Leave"),
            InOfficeStates::UsingExemption => write!(f, "Using Exemption"),
        }
    }
}
