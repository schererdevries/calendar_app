use chrono::{Datelike, Local, NaiveDate};

fn main() {
    // Get the current date
    let now = Local::now();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();
    
    // Get the first day of the month and the number of days in the month
    let first_day = NaiveDate::from_ymd_opt(current_year, current_month, 1)
        .expect("Invalid date");
    let days_in_month = first_day.num_days_in_month();
    
    // Get the day of the week for the first day (0 = Monday, 6 = Sunday)
    let first_day_weekday = first_day.weekday();
    
    // Print the month name
    let month_names = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];
    println!("    {:^21}", month_names[(current_month - 1) as usize]);
    println!("    {:^21}", current_year);
    
    // Print abbreviated weekday names (one character each)
    // Starting with Monday to match ISO week convention
    println!("  M   T   W   T   F   S   S");
    
    // Print leading spaces for the first week based on the first day of the month
    // Each day takes up 3 characters (2 for the number + 1 space), so we multiply by 3
    let spaces = first_day_weekday.num_days_from_monday() * 4;
    for _ in 0..spaces {
        print!(" ");
    }
    
    // Print the days of the month
    for day in 1..=days_in_month as u32 {
        // Highlight the current day with brackets
        if day == current_day as u32 {
            print!("[{:2}]", day);
        } else {
            print!("{:3} ", day);
        }
        
        // Move to the next line after Sunday (Sunday = 6, counting from Monday)
        if (first_day_weekday.num_days_from_monday() as u32 + day - 1) % 7 == 6 {
            println!();
        }
    }
    
    // Add a newline at the end if the month doesn't end on Sunday
    if (first_day_weekday.num_days_from_monday() as u32 + days_in_month as u32 - 1) % 7 != 6 {
        println!();
    }
}
