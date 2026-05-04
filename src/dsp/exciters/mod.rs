pub mod scrape;

pub use scrape::ScrapeExciter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Exciter {
    Hit,
    Scrape,
}

impl Exciter {
    pub fn from_int(v: i32) -> Self {
        match v {
            1 => Exciter::Scrape,
            _ => Exciter::Hit,
        }
    }

    pub fn to_int(&self) -> i32 {
        match self {
            Exciter::Hit => 0,
            Exciter::Scrape => 1,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Exciter::Hit => "Hit",
            Exciter::Scrape => "Scrape",
        }
    }
}
