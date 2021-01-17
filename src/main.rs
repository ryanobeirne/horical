use holiday::*;
use std::fmt;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut std_out = std::io::stdout();
    writeln!(&mut std_out, "{}", HoriCal::today())?;

    Ok(())
}

struct HoriCal {
    weekdays: Vec<Weekday>,
    days: Vec<u32>,
    today: u32,
}

impl fmt::Display for HoriCal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let weekdays = self
            .weekdays
            .iter()
            .enumerate()
            .map(|(i, wkd)| {
                let wkd = TwoLetterWkday::from(wkd);
                if i == self.today as usize {
                    format!("${{color0}}{}${{color}}", wkd)
                } else if wkd.is_weekend() {
                    format!("${{color1}}{}${{color}}", wkd)
                } else {
                    wkd.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("  ");

        let days = self
            .days
            .iter()
            .enumerate()
            .map(|(i, day)| {
                if i == self.today as usize {
                    format!("${{color0}}{:02}${{color}}", day)
                } else if self.weekdays[i].is_weekend() {
                    format!("${{color1}}{:02}${{color}}", day)
                } else {
                    format!("{:02}", day)
                }
            })
            .collect::<Vec<_>>()
            .join("  ");

        write!(f, "{}\n{}", weekdays, days)
    }
}

impl HoriCal {
    fn today() -> Self {
        let mut weekdays = Vec::new();
        let mut days = Vec::new();

        let today = Local::today();
        let mut i = today.with_day0(0).unwrap().naive_local();
        let last = today.last_day_of_month();

        while i <= last {
            weekdays.push(i.weekday());
            days.push(i.day());
            i = i.succ();
        }

        HoriCal {
            weekdays,
            days,
            today: today.day0(),
        }
    }
}

use TwoLetterWkday::*;
#[derive(Debug, PartialEq, Eq)]
enum TwoLetterWkday {
    SU,
    MO,
    TU,
    WE,
    TH,
    FR,
    SA,
}

impl IsWeekend for TwoLetterWkday {
    fn is_weekend(&self) -> bool {
        self == &SA || self == &SU
    }
}

impl IsWeekend for Weekday {
    fn is_weekend(&self) -> bool {
        self == &Weekday::Sat || self == &Weekday::Sun
    }
}

impl fmt::Display for TwoLetterWkday {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&Weekday> for TwoLetterWkday {
    fn from(weekday: &Weekday) -> Self {
        use Weekday::*;
        match weekday {
            Sun => SU,
            Mon => MO,
            Tue => TU,
            Wed => WE,
            Thu => TH,
            Fri => FR,
            Sat => SA,
        }
    }
}

trait IsWeekend {
    fn is_weekend(&self) -> bool;
}
