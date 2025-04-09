#[derive(Debug, Clone)]
pub struct Rule {
    pub birth: Vec<u8>,
    pub survive: Vec<u8>,
}

impl Rule {
    pub fn from_str(rule_str: &str) -> Option<Self> {
        todo!()
    }
}
