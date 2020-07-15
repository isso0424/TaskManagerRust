use crate::command::types::{label::Labels, task::Tasks};
use crate::config::parse_arg;

fn check_label(args: Vec<String>) -> Result<(), String> {
    let title = parse_arg::get_search_keyword(&args);
    let labels = Labels::load()
        .map_err(|err| err.to_string())?
        .search_with_title(title);

    let mut label_notifies = "".to_string();

    labels.content.iter().for_each(|label| {
        label_notifies = label_notifies.clone() + label.title.as_str() + "\n";
    });

    let notification_message = format!(
        "現在のラベルの一覧は以下のとおりです\n\n\n\n{}",
        label_notifies
    );

    print!("{}", notification_message);

    Ok(())
}

fn get_done_notifies(tasks: &Tasks) -> Vec<String> {
    tasks
        .content
        .iter()
        .map(|task| {
            if !task.done {
                return "".to_string();
            }
            let task_title = task.get_title();
            format!("タスク名:{}\n\n", task_title)
        })
        .collect()
}

fn get_notifies(tasks: &Tasks) -> Vec<String> {
    tasks
        .content
        .iter()
        .map(|task| {
            if task.done {
                return "".to_string();
            }
            let task_title = task.get_title();
            let task_limit: String = task.limit_to_string();
            let task_labels = task.get_label();

            let mut label_string = "".to_string();

            for label in task_labels {
                label_string = label_string + label.title.as_str() + "  ";
            }
            format!(
                "タスク名:{}\n期限:{}\nラベル:{}\n",
                task_title, task_limit, label_string
            )
        })
        .collect()
}

fn create_task_notify(tasks: &Tasks) -> String {
    let task_notifies = get_notifies(tasks);
    let mut notifies = "".to_string();

    if task_notifies.iter().all(|notify| notify == "") {
        return "現在残っているタスクはありません".to_string();
    }

    for task in task_notifies {
        if task == "" {
            continue;
        }
        notifies += task.as_str();
    }

    notifies
}

fn create_done_task_notify(tasks: &Tasks) -> String {
    let task_notifies = get_done_notifies(tasks);
    let mut notifies = "".to_string();

    if task_notifies.iter().all(|notify| notify == "") {
        return "現在完了済みのタスクはありません".to_string();
    }

    for task in task_notifies {
        notifies += task.as_str();
    }

    notifies
}

fn create_notification_message(tasks: Tasks) -> String {
    let task_notifies = create_task_notify(&tasks);

    let done_task_notifies = create_done_task_notify(&tasks);

    format!(
        "現在残っているタスクは以下のとおりです\n\n{}\n\n完了済みのタスクは以下の通りです\n\n{}",
        task_notifies, done_task_notifies
    )
}

fn check_task(args: Vec<String>) -> Result<(), String> {
    let title = parse_arg::get_search_keyword(&args);
    let label = parse_arg::get_search_label(&args);

    let tasks = Tasks::load()
        .map_err(|err| err.to_string())?
        .search_with_title(title)
        .search_with_label(label);

    let notification_message = create_notification_message(tasks);

    println!("{}", notification_message);

    Ok(())
}

pub fn check(args: Vec<String>) -> Result<(), String> {
    let target = &args[2];

    match target.as_str() {
        "label" => {
            check_label(args)?;
        }
        "task" => {
            check_task(args)?;
        }
        _ => return Err("Target not found.".to_string()),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::types::{label::Label, task::Task};

    #[test]
    fn create_notification_message_success() {
        let task = Task {
            title: "title".to_string(),
            label: Some(vec![Label {
                title: "label".to_string(),
            }]),
            limit: Some(944956800),
            done: false,
        };

        let task_done = Task {
            title: "title".to_string(),
            label: Some(vec![Label {
                title: "label".to_string(),
            }]),
            limit: Some(944956800),
            done: false,
        };

        let tasks = Tasks {
            content: vec![task, task_done],
        };

        assert_eq!(
            create_notification_message(tasks.clone()),
            format!(
                "現在残っているタスクは以下のとおりです\n\n{}\n\n完了済みのタスクは以下の通りです\n\n{}",
                create_task_notify(&tasks),
                create_done_task_notify(&tasks)
            )
        );
    }

    #[test]
    fn create_notification_message_empty() {
        let task = Task {
            title: "title".to_string(),
            label: Some(vec![Label {
                title: "label".to_string(),
            }]),
            limit: Some(944956800),
            done: false,
        };

        let task_done = Task {
            title: "title".to_string(),
            label: Some(vec![Label {
                title: "label".to_string(),
            }]),
            limit: Some(944956800),
            done: true,
        };

        let tasks = Tasks {
            content: vec![task_done],
        };

        assert_eq!(
            create_notification_message(tasks.clone()),
            format!(
                "現在残っているタスクは以下のとおりです\n\n{}\n\n完了済みのタスクは以下の通りです\n\n{}",
                "現在残っているタスクはありません".to_string(),
                create_done_task_notify(&tasks)
            )
        );

        let tasks = Tasks {
            content: vec![task],
        };

        assert_eq!(
            create_notification_message(tasks.clone()),
            format!(
                "現在残っているタスクは以下のとおりです\n\n{}\n\n完了済みのタスクは以下の通りです\n\n{}",
                create_task_notify(&tasks),
                "現在完了済みのタスクはありません".to_string()
            )
        );
    }
}
