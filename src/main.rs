use std::process::Command;
use serde_json::{Result, Value};
use std::collections::HashSet;

fn get_workspace_json() -> String {
    let output = 
        Command::new("swaymsg")
        .arg("-t")
        .arg("get_workspaces")
        .output()
        .expect("failed to execute process");
    return String::from_utf8_lossy(&output.stdout).as_ref().to_string();
}

fn get_value(x : String) -> Result<Value> {

    let v: Value = serde_json::from_str(x.as_ref())?;
    return Ok(v); 
}

fn main() -> Result<()> {
    let output = get_workspace_json();
    let v : Value = get_value(output)?;


    if let Value::Array(arr) = v {
        let b: HashSet<i64>= 
            arr
            .iter()
            .filter_map(|workspace| workspace.get("num").and_then(Value::as_i64))
            .collect();
        for i in 1..20 {
            if !b.contains(&i) {
                Command::new("swaymsg")
                    .arg("workspace")
                    .arg(i.to_string())
                    .output()
                    .expect("failed to execute process");
                break;
            }
        }
    }

    Ok(())
}
