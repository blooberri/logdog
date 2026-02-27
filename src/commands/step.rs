use std::io::Write;
use std::fs::{OpenOptions};
pub fn execute_step(description: String){
    let mut file = match OpenOptions::new().append(true).open("report.md") {
                Ok(f)=> f,
                Err(_)=>{
                    eprint!("❌ Error: 'report.md' not found. Run 'logdog init' first!");
                    return;
                }
            };
            let formatted_step = format!("- {}\n",description);
            file.write_all(formatted_step.as_bytes()).expect("Failed to write step");
            println!("✅ Step logged: {}", description);
}