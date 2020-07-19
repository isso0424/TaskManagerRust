use std::env::current_exe;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::command::types::task::Tasks;
fn create_file(file_name: &str) -> Result<(), std::io::Error> {
    let path = current_exe()?.parent().unwrap().join(file_name);

    if path.is_file() {
        return Ok(());
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let empty_tasks = Tasks { content: vec![] };

    write!(writer, "{}", serde_json::to_string(&empty_tasks)?)?;

    Ok(())
}

pub fn create_data_json() -> Result<(), String> {
    create_file("tasks.json").map_err(|err| err.to_string())?;
    create_file("labels.json").map_err(|err| err.to_string())?;

    Ok(())
}
