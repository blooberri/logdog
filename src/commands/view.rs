pub fn execute_view() {
    match std::fs::read_to_string("report.md") {
        //take the massive string of text and print it.
        Ok(content) => {
            println!("\n==================================");
            println!("REPORT PREVIEW");
            println!("==================================\n");
            println!("{}", content);
            println!("==================================\n");
        }
        Err(_) => {
            eprintln!("❌ Error: 'report.md' not found. Run 'logdog init' first!");
        }
    }
}
