
use crossterm::{
    cursor::MoveTo, event::{Event, KeyCode, read}, execute, style::Print, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;

fn main() {
    enable_raw_mode().expect("raw-mode-did-not-work");
    let mut stdout = stdout();
    
    loop {
        if let Event::Key(event ) = read().expect("Did-not-register-Key") {
            if event.code == KeyCode::Char('q') {
                break;
            }
            execute!(
                stdout,
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print("Welcome to Kaizen")
            ).expect("Unable to run the execute command");
        }
    }

    disable_raw_mode().expect("couldn-not-turn-off-raw_mode");
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).expect("Couldn't clear the screen");
}