mod app;
use app::{App, START_CMD};

fn main() {
    let mut app = App::new();
    loop {
        let input = app.get_input();

        if input.trim() == format!("{}q", START_CMD) 
        || input.trim() == format!("{}quit", START_CMD) {
            break;
        }

        if input.is_empty() {
            continue;
        }

        if app.is_command(&input) {
            if let Err(e) = app.execute(&input) {
                println!("{}", e);
            };
        } else {
            match app.convert(&input) {
                Ok(output) => {
                    app.print(&output);
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}