use chrono::offset::Local;
use chrono::TimeZone;
use std::convert::TryInto;

pub fn get_limit(args: &Vec<String>) -> Option<i64> {
    if let Some(index) = args.iter().position(|arg| arg == "--limit") {
        if args.len() == index + 1 {
            return None;
        }
        let raw_date_time = &args[index + 1];
        print!("{}", raw_date_time);
        let date_time = Local
            .datetime_from_str(format!("{} 00:00", raw_date_time).as_str(), "%F %R")
            .ok();
        print!("{:?}", date_time);
        if date_time.is_none() {
            return None;
        }
        return Some(date_time?.timestamp());
    }
    return None;
}

pub fn get_label<'a>(args: &'a Vec<String>) -> Option<Vec<&'a str>> {
    if let Some(index) = args.iter().position(|arg| arg == "--label") {
        if args.len() == index + 1 {
            return None;
        }
        let labels = args[index + 1].split(",").collect();
        return Some(labels);
    }
    return None;
}
