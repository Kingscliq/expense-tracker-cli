mod expenses;
mod logger_module;
mod ui;

use expenses::{ExpenseTracker, TrackerActions};
use logger_module as logger;
use std::io;
use ui::display_actions;
use ui::start;

fn main() {
    let mut tracker = ExpenseTracker::new();

    loop {
        display_actions();
        println!("Welcome to our program, please enter a valid input");

        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Please enter a valid character");

        let action = TrackerActions::get_power_action(&action);

        if let Some(act) = action {
            start(&act, &mut tracker)
        } else {
            println!("Failed to process command. Please enter a valid input")
        }
    }
}
