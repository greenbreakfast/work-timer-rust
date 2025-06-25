use std::{thread, time::Duration, io::{self, Write}};
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};

fn format_remaining_timer(timer: i32) -> String {
    let mut s = String::new();
    let mut min;
    let mut seconds = timer;
    if timer > 60 {
        seconds = timer % 60;
        min = (timer - seconds)/60;
 
        s = format!("{min} minutes, ");
    } 
    s = format!("{s}{seconds} seconds");
    return s;
}

fn display_remaining_timer(timer: i32) {
    let mut stdout = io::stdout();
    
    // Clear the current line and move cursor to beginning
    stdout
        .execute(terminal::Clear(ClearType::CurrentLine))
        .unwrap()
        .execute(cursor::MoveToColumn(0))
        .unwrap();
    
    // Print the timer without a newline
    print!("\tRemaining: {}", format_remaining_timer(timer));
    
    // Flush to ensure immediate display
    stdout.flush().unwrap();
}

fn main() {
    let mut timer = 25*60;
    println!("Timer for {} seconds", timer);
    
    // Print initial timer display
    display_remaining_timer(timer);
    
    loop {
        thread::sleep(Duration::from_millis(1000));
        timer -= 1;

        // update display
        display_remaining_timer(timer);

        if timer == 0 {
            break;
        }
    }

    // Move to next line before final message
    println!();
    println!("Timer complete!");
}