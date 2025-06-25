use std::{thread, time::Duration, io::{self, Write}};
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};

fn format_remaining_time(timer: i32) -> String {
    let seconds = timer % 60;
    let min = (timer - seconds)/60;

    let s = format!("{min}m{seconds}s");
    return s;
}

fn percent_complete(timer: i32, timer_start_val: i32) -> String {
    return format!("{:.0}%", ((timer_start_val - timer) as f32/timer_start_val as f32)*100.0);
}

fn display_remaining_timer(timer: i32, timer_start_val: i32) {
    let mut stdout = io::stdout();
    
    // Clear the current line and move cursor to beginning
    stdout
        .execute(terminal::Clear(ClearType::CurrentLine))
        .unwrap()
        .execute(cursor::MoveToColumn(0))
        .unwrap();
    
    // Print the timer without a newline
    print!("\tRemaining: {}\t{}", format_remaining_time(timer), percent_complete(timer, timer_start_val));
    
    // Flush to ensure immediate display
    stdout.flush().unwrap();
}

fn main() {
    let timer_start_val = 25*60;
    let mut timer = timer_start_val;
    println!("Timer for {}", format_remaining_time(timer));
    
    // Print initial timer display
    display_remaining_timer(timer, timer_start_val);
    
    loop {
        thread::sleep(Duration::from_millis(1000));
        timer -= 1;

        // update display
        display_remaining_timer(timer, timer_start_val);

        if timer == 0 {
            break;
        }
    }

    // Move to next line before final message
    println!();
    println!("Timer complete!");
}