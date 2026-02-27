use std::fs::{OpenOptions};
use std::io::{self, Read,Write};
pub fn execute_fetch(){
    let mut input_data = String::new();
            io::stdin().read_to_string(&mut input_data).expect("Failed to read terminal output");
            //Open the file to append
            let mut file = match OpenOptions::new().append(true).open("report.md"){
                Ok(f)=>f,
                Err(_)=>{
                    eprintln!("❌ Error: 'report.md' not found. Run 'logdog init' first!");
                    return;
                }
            };
            let formatted_code_block = format!("\n**Terminal Evidence:**\n```bash\n{}\n```\n", input_data.trim());
            //Write to the file
            file.write_all(formatted_code_block.as_bytes()).expect("Failed to write to report");
            println!("LogDog fetched your terminal output.");
}