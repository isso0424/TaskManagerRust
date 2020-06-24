use crate::command::types::{label::Labels, task::Tasks};
use chrono::{Local, TimeZone};

fn check_label() -> Result<(), String> {
    let labels = Labels::load().map_err(|err| return err.to_string())?;

    let mut all_label_notifies = "".to_string();

    for label in labels.content {
        all_label_notifies = all_label_notifies + label.title.as_str() + "\n";
    }

    let notification_message = format!(
        "現在のラベルの一覧は以下のとおりです\n\n\n\n{}",
        all_label_notifies
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
            let task_title = &task.title;
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
            let task_title = &task.title;
            let task_limit: String = match task.limit {
                Some(limit) => Local.timestamp(limit, 0).to_string(),
                None => "なし".to_string(),
            };
            let empty_vector = vec![];
            let task_labels = match &task.label {
                Some(labels) => labels,
                None => &empty_vector,
            };
            let mut task_label = "".to_string();
            for label in task_labels {
                task_label = task_label + label.title.as_str() + "  ";
            }
            format!(
                "タスク名:{}\n期限:{}\nラベル:{}\n",
                task_title, task_limit, task_label
            )
        })
        .collect()
}

fn create_task_notify(tasks: Vec<String>) -> String {
    let mut task_notifies = "".to_string();

    if tasks.len() == 0 {
        return "現在残っているタスクはありません".to_string();
    }

    for task in tasks {
        task_notifies = task_notifies + task.as_str();
    }

    task_notifies
}

fn create_done_task_notify(tasks: Vec<String>) -> String {
    let mut done_task_notifies = "".to_string();

    if tasks.len() == 0 {
        return "現在完了済みのタスクはありません".to_string();
    }

    for task in tasks {
        done_task_notifies = done_task_notifies + task.as_str();
    }

    done_task_notifies
}

fn check_task() -> Result<(), String> {
    let tasks = Tasks::load().map_err(|err| return err.to_string())?;

    let task_notifies_vec = get_notifies(&tasks);

    let done_task_notifies_vec = get_done_notifies(&tasks);

    let task_notifies = create_task_notify(task_notifies_vec);

    let done_task_notifies = create_done_task_notify(done_task_notifies_vec);

    let notification_message = format!(
        "現在残っているタスクは以下のとおりです\n\n{}\n\n完了済みのタスクは以下の通りです\n\n{}",
        task_notifies, done_task_notifies
    );

    println!("{}", notification_message);

    Ok(())
}

pub fn check(args: Vec<String>) -> Result<(), String> {
    let target = &args[2];

    match target.as_str() {
        "label" => {
            check_label()?;
        }
        "task" => {
            check_task()?;
        }
        _ => return Err("Target not found.".to_string()),
    }
    Ok(())
}
