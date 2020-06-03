use crate::command::types::{label::Labels, task::Tasks};
use chrono::{Local, TimeZone};

fn check_label() -> Result<(), String> {
    let labels = Labels::load().map_err(|err| return err.to_string())?;

    let all_label_notifies = "".to_string();

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

fn check_task() -> Result<(), String> {
    let all_task_notifies = "".to_string();
    let tasks = Tasks::load().map_err(|err| return err.to_string())?;

    let all_task_notifies_vec: Vec<String> = tasks
        .content
        .iter()
        .map(|task| {
            let task_title = task.title;
            let task_limit: String = match task.limit {
                Some(limit) => Local.timestamp(limit, 0).to_string(),
                None => "なし".to_string(),
            };
            let task_labels = match task.label {
                Some(labels) => labels,
                None => vec![],
            };
            let task_label = "".to_string();
            for label in task_labels {
                task_label = task_label + label.title.as_str() + "  ";
            }
            format!(
                "タスク名:{}\n期限:{}\nラベル:{}\n\n\n\n",
                task_title, task_limit, task_label
            )
        })
        .collect();

    for task in all_task_notifies_vec {
        all_task_notifies = all_task_notifies + task.as_str();
    }

    let notification_message = format!(
        "現在残っているタスクは以下のとおりです\n\n\n\n{}",
        all_task_notifies
    );

    println!("{}", notification_message);

    Ok(())
}

fn check(args: Vec<String>) -> Result<(), String> {
    let target = &args[2];

    match target.as_str() {
        "label" => {
            check_label()?;
        }
        "task" => {
            check_task()?;
        }
    }
    Ok(())
}
