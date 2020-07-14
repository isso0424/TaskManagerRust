use chrono::offset::Utc;
use chrono::TimeZone;

pub fn get_limit(args: &[String]) -> Option<i64> {
    if let Some(index) = args.iter().position(|arg| arg == "--limit") {
        let target_index = index + 1;
        if args.len() == target_index {
            return None;
        }

        let raw_date_time = &args[target_index];
        let date_time = Utc
            .datetime_from_str(format!("{} 00:00", raw_date_time).as_str(), "%F %R")
            .ok();

        return Some(date_time?.timestamp());
    }

    None
}

pub fn get_label<'a>(args: &'a [String]) -> Option<Vec<&'a str>> {
    if let Some(index) = args.iter().position(|arg| arg == "--label") {
        let target_index = index + 1;
        if args.len() == target_index {
            return None;
        }
        let labels = args[target_index].split(',').collect();

        return Some(labels);
    }

    None
}

pub fn get_title(args: &[String]) -> Option<String> {
    if let Some(index) = args.iter().position(|arg| arg == "--title") {
        let target_index = index + 1;
        if args.len() == target_index {
            return None;
        }
        let title = args[target_index].clone();

        return Some(title);
    }

    None
}

pub fn get_search_keyword(args: &[String]) -> Option<String> {
    if let Some(index) = args
        .iter()
        .position(|arg| arg == "-k" || arg == "--keyword")
    {
        let target_index = index + 1;
        if args.len() == target_index {
            return None;
        }
        let keyword = args[target_index].clone();

        return Some(keyword);
    }

    None
}

pub fn get_search_label(args: &[String]) -> Option<String> {
    if let Some(index) = args.iter().position(|arg| arg == "-l" || arg == "--label") {
        let target_index = index + 1;
        if args.len() == target_index {
            return None;
        }
        let label = args[target_index].clone();

        return Some(label);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_limit_success() {
        let args: Vec<String> = vec!["--limit", "1999-12-12"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_limit(&args), Some(944956800));
    }

    #[test]
    fn get_limit_failed() {
        let args: Vec<String> = vec!["--limit", "unchi-12-12"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_limit(&args), None);

        let args: Vec<String> = vec!["--limit"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_limit(&args), None);

        let args: Vec<String> = vec!["1999-12-12"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_limit(&args), None);
    }

    #[test]
    fn get_label_success() {
        let args: Vec<String> = vec!["--label", "label1,label2"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        let result = vec!["label1", "label2"];
        assert_eq!(get_label(&args), Some(result));
    }

    #[test]
    fn get_label_failed() {
        let args: Vec<String> = vec!["--label"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_label(&args), None);

        let args: Vec<String> = vec![""].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_label(&args), None);
    }

    #[test]
    fn get_title_success() {
        let args: Vec<String> = vec!["--title", "title"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_title(&args), Some("title".to_string()));
    }

    #[test]
    fn get_title_failed() {
        let args: Vec<String> = vec!["title"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_title(&args), None);

        let args: Vec<String> = vec!["--title"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_title(&args), None);
    }

    #[test]
    fn get_search_label_success() {
        let args: Vec<String> = vec!["--label", "label1"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_search_label(&args), Some("label1".to_string()));

        let args: Vec<String> = vec!["-l", "label1"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();

        assert_eq!(get_search_label(&args), Some("label1".to_string()));
    }

    #[test]
    fn get_search_label_failed() {
        let args: Vec<String> = vec!["--label"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_search_label(&args), None);

        let args: Vec<String> = vec!["-l"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_search_label(&args), None);

        let args: Vec<String> = vec!["label"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_search_label(&args), None);
    }

    #[test]
    fn get_search_keyword_success() {
        let args: Vec<String> = vec!["--keyword", "title"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_search_keyword(&args), Some("title".to_string()));

        let args: Vec<String> = vec!["-k", "title"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_search_keyword(&args), Some("title".to_string()));
    }

    #[test]
    fn get_search_keyword_failed() {
        let args: Vec<String> = vec!["--keyword"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();
        assert_eq!(get_search_keyword(&args), None);

        let args: Vec<String> = vec!["-k"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_search_keyword(&args), None);

        let args: Vec<String> = vec!["keyword"].iter().map(|arg| arg.to_string()).collect();
        assert_eq!(get_search_keyword(&args), None);
    }
}
