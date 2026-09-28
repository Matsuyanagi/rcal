use chrono::{Datelike, Days, NaiveDate, Weekday};
use std::collections::HashSet;
use std::io;
use std::path::Path;

#[derive(Default)]
pub struct HolidayCalendar {
    events: Vec<Event>,
}

struct Event {
    start: NaiveDate,
    duration: u64,
    rule: Option<YearlyRule>,
    extra_dates: HashSet<NaiveDate>,
    excluded_dates: HashSet<NaiveDate>,
}

#[derive(Default)]
struct EventBuilder {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    rule: Option<YearlyRule>,
    extra_dates: HashSet<NaiveDate>,
    excluded_dates: HashSet<NaiveDate>,
}

struct YearlyRule {
    until: Option<NaiveDate>,
    months: Vec<u32>,
    month_days: Vec<i32>,
    week_days: Vec<(Option<i8>, Weekday)>,
}

impl HolidayCalendar {
    pub fn load(executable: &Path, explicit: Option<&Path>) -> io::Result<Self> {
        let default = executable
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join("rcal.ics");
        let path = explicit.unwrap_or(&default);
        let contents = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if explicit.is_none() && error.kind() == io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(error) => return Err(error),
        };
        Self::from_ics(&contents).map_err(|message| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("{}: {message}", path.display()),
            )
        })
    }

    pub fn from_ics(contents: &str) -> Result<Self, String> {
        let mut calendar = Self::default();
        let mut event: Option<EventBuilder> = None;
        let mut unfolded: Vec<String> = Vec::new();
        for line in contents.lines() {
            let line = line.trim_end_matches('\r');
            if line.starts_with([' ', '\t']) {
                let previous = unfolded.last_mut().ok_or("orphaned folded line")?;
                previous.push_str(&line[1..]);
            } else {
                unfolded.push(line.to_string());
            }
        }
        for line in unfolded {
            if line.eq_ignore_ascii_case("BEGIN:VEVENT") {
                if event.is_some() {
                    return Err("nested VEVENT".into());
                }
                event = Some(EventBuilder::default());
                continue;
            }
            if line.eq_ignore_ascii_case("END:VEVENT") {
                let item = event.take().ok_or("END:VEVENT without BEGIN:VEVENT")?;
                let start = item.start.ok_or("VEVENT has no DTSTART")?;
                let duration = match item.end {
                    Some(end) if end <= start => return Err("DTEND must follow DTSTART".into()),
                    Some(end) => (end - start).num_days() as u64,
                    None => 1,
                };
                calendar.events.push(Event {
                    start,
                    duration,
                    rule: item.rule,
                    extra_dates: item.extra_dates,
                    excluded_dates: item.excluded_dates,
                });
                continue;
            }
            let Some(item) = event.as_mut() else { continue };
            let Some((property, value)) = line.split_once(':') else {
                continue;
            };
            let name = property.split(';').next().unwrap_or("");
            match name.to_ascii_uppercase().as_str() {
                "DTSTART" => item.start = Some(parse_date(value)?),
                "DTEND" => item.end = Some(parse_date(value)?),
                "RRULE" => item.rule = Some(YearlyRule::parse(value)?),
                "RDATE" => item.extra_dates.extend(parse_dates(value)?),
                "EXDATE" => item.excluded_dates.extend(parse_dates(value)?),
                _ => {}
            }
        }
        if event.is_some() {
            return Err("unterminated VEVENT".into());
        }
        Ok(calendar)
    }

    pub fn contains(&self, date: NaiveDate) -> bool {
        self.events.iter().any(|event| event.contains(date))
    }
}

impl Event {
    fn contains(&self, date: NaiveDate) -> bool {
        (0..self.duration).any(|offset| {
            let Some(start) = date.checked_sub_days(Days::new(offset)) else {
                return false;
            };
            !self.excluded_dates.contains(&start)
                && (self.extra_dates.contains(&start)
                    || start == self.start
                    || self
                        .rule
                        .as_ref()
                        .is_some_and(|rule| rule.matches(self.start, start)))
        })
    }
}

impl YearlyRule {
    fn parse(value: &str) -> Result<Self, String> {
        let mut rule = Self {
            until: None,
            months: Vec::new(),
            month_days: Vec::new(),
            week_days: Vec::new(),
        };
        let mut frequency = false;
        for part in value.split(';') {
            let (name, data) = part
                .split_once('=')
                .ok_or_else(|| format!("invalid RRULE part: {part}"))?;
            match name {
                "FREQ" if data == "YEARLY" => frequency = true,
                "UNTIL" => rule.until = Some(parse_date(data)?),
                "BYMONTH" => {
                    rule.months = data
                        .split(',')
                        .map(|s| {
                            s.parse::<u32>()
                                .map_err(|_| format!("invalid BYMONTH: {s}"))
                        })
                        .collect::<Result<_, _>>()?
                }
                "BYMONTHDAY" => {
                    rule.month_days = data
                        .split(',')
                        .map(|s| {
                            s.parse::<i32>()
                                .map_err(|_| format!("invalid BYMONTHDAY: {s}"))
                        })
                        .collect::<Result<_, _>>()?
                }
                "BYDAY" => {
                    rule.week_days = data
                        .split(',')
                        .map(parse_week_day)
                        .collect::<Result<_, _>>()?
                }
                _ => return Err(format!("unsupported RRULE part: {part}")),
            }
        }
        if !frequency {
            return Err("only FREQ=YEARLY is supported".into());
        }
        if rule.months.iter().any(|month| !(1..=12).contains(month))
            || rule
                .month_days
                .iter()
                .any(|day| *day == 0 || !(-31..=31).contains(day))
            || (rule.months.is_empty()
                && rule.week_days.iter().any(|(ordinal, _)| ordinal.is_some()))
        {
            return Err("unsupported BYMONTH, BYMONTHDAY or BYDAY value".into());
        }
        Ok(rule)
    }

    fn matches(&self, first: NaiveDate, date: NaiveDate) -> bool {
        if date < first || self.until.is_some_and(|until| date > until) {
            return false;
        }
        if !self.months.is_empty() {
            if !self.months.contains(&date.month()) {
                return false;
            }
        } else if self.month_days.is_empty()
            && self.week_days.is_empty()
            && date.month() != first.month()
        {
            return false;
        }
        if !self.month_days.is_empty() {
            let days_in_month = date
                .with_day(1)
                .unwrap()
                .checked_add_months(chrono::Months::new(1))
                .unwrap()
                .pred_opt()
                .unwrap()
                .day() as i32;
            if !self.month_days.iter().any(|day| {
                if *day > 0 {
                    *day == date.day() as i32
                } else {
                    days_in_month + *day + 1 == date.day() as i32
                }
            }) {
                return false;
            }
        } else if self.week_days.is_empty() && date.day() != first.day() {
            return false;
        }
        self.week_days.is_empty()
            || self.week_days.iter().any(|(ordinal, weekday)| {
                if date.weekday() != *weekday {
                    return false;
                }
                match ordinal {
                    None => true,
                    Some(n) if *n > 0 => (date.day() as i8 - 1) / 7 + 1 == *n,
                    Some(n) => {
                        let last = date
                            .with_day(1)
                            .unwrap()
                            .checked_add_months(chrono::Months::new(1))
                            .unwrap()
                            .pred_opt()
                            .unwrap()
                            .day();
                        -(((last - date.day()) / 7 + 1) as i8) == *n
                    }
                }
            })
    }
}

fn parse_week_day(value: &str) -> Result<(Option<i8>, Weekday), String> {
    if value.len() < 2 {
        return Err(format!("invalid BYDAY: {value}"));
    }
    let (prefix, day) = value.split_at(value.len() - 2);
    let weekday = match day {
        "MO" => Weekday::Mon,
        "TU" => Weekday::Tue,
        "WE" => Weekday::Wed,
        "TH" => Weekday::Thu,
        "FR" => Weekday::Fri,
        "SA" => Weekday::Sat,
        "SU" => Weekday::Sun,
        _ => return Err(format!("invalid BYDAY: {value}")),
    };
    let ordinal = if prefix.is_empty() {
        None
    } else {
        let n = prefix
            .parse::<i8>()
            .map_err(|_| format!("invalid BYDAY: {value}"))?;
        if n == 0 || !(-5..=5).contains(&n) {
            return Err(format!("invalid BYDAY: {value}"));
        }
        Some(n)
    };
    Ok((ordinal, weekday))
}

fn parse_dates(value: &str) -> Result<Vec<NaiveDate>, String> {
    value.split(',').map(parse_date).collect()
}

fn parse_date(value: &str) -> Result<NaiveDate, String> {
    if value.len() != 8 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("unsupported date value: {value}"));
    }
    NaiveDate::parse_from_str(value, "%Y%m%d").map_err(|_| format!("invalid date: {value}"))
}
