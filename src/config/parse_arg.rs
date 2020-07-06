use chrono::offset::Local;
use chrono::TimeZone;

pub fn get_limit(args: &Vec<String>) -> Option<i64> {
    if let Some(index) = args.iter().position(|arg| arg == "--limit") {
        let target_index = index + 1;
        if args.len() == target_index {
            return None;
        }

        let raw_date_time = &args[target_index];
        let date_time = Local
            .datetime_from_str(format!("{} 00:00", raw_date_time).as_str(), "%F %R")
            .ok();

        if date_time.is_none() {
            return None;
        }
        return Some(date_time?.timestamp());
    }

    None
}

pub fn get_label<'a>(args: &'a Vec<String>) -> Option<Vec<&'a str>> {
    if let Some(index) = args.iter().position(|arg| arg == "--label") {
        let target_index = index + 1;
        if args.len() == target_index {
            return None;
        }
        let labels = args[target_index].split(",").collect();

        return Some(labels);
    }

    None
}

pub fn get_title(args: &Vec<String>) -> Option<String> {
    if let Some(index) = args.iter().position(|arg| arg == "--title") {
        if args.len() == index + 1 {
            return None;
        }
        let title = args[index + 1].clone();

        return Some(title);
    }

    return None;
}
