use std::env;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;

use chrono::Datelike;

static mut COLLEAGUES_RAW: String = String::new();
static mut COLLEAGUES: Vec<&'static str> = vec![];

fn main() -> std::io::Result<()> {
    create_dir_all("db")?;
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => print_volunteer_for_cw(),
        Some(x) if x == "populate" => populate_roster(),
        Some(x) if x == "employees" => unsafe { print_employees() },
        _ => print_volunteer_for_cw(),
    }
    Ok(())
}

fn print_volunteer_for_cw() {
    let cw = get_current_cw();
    let volunteer = get_volunteer_for_cw(cw.to_string());
    // @TODO if voluteer is 'unknown', print a reminder to populate the roster
    println!("┌────────────────────────────────┐");
    println!("│ Current week: {}", cw);
    println!("│ Our volunteer: {}", volunteer);
    println!("└────────────────────────────────┘");
}

unsafe fn fetch_employees() {
    COLLEAGUES_RAW.clear();
    COLLEAGUES_RAW.push_str(safely_read_file("db/colleagues.csv").trim());
    COLLEAGUES = COLLEAGUES_RAW.split('\n').collect();
}

unsafe fn print_employees() {
    fetch_employees();
    println!("┌────────────────────────────────┐");
    for c in &COLLEAGUES {
        println!("│ {}", *c);
    }
    println!("└────────────────────────────────┘");
}

fn get_current_year() -> i32 {
    chrono::Utc::now().year()
}

fn get_current_cw() -> u32 {
    chrono::Utc::now().iso_week().week()
}

fn safely_read_file(filepath: &str) -> String {
    let file = OpenOptions::new().write(true).create(true).open(filepath);
    let ret_val = match file {
        Ok(_) => read_to_string(filepath).unwrap(),
        Err(_) => String::from(""),
    };
    ret_val
}

fn get_volunteer_for_cw(cw: String) -> String {
    let cur_year = get_current_year();
    let mut target_row = cur_year.to_string() + "," + &cw;
    let roster_raw = safely_read_file("db/roster.csv");
    let roster_str = roster_raw.trim();
    let targeted_row = roster_str
        .split('\n')
        .find(|row| row.starts_with(&target_row))
        .unwrap_or_else(|| {
            target_row.push_str(",unknown");
            &target_row
        });
    // yyyy,ww,name
    let volunteer = targeted_row.split(',').collect::<Vec<&str>>()[2];
    volunteer.to_string()
}

fn populate_roster() {
    populate_roster_from_cw(get_current_cw());
}

fn populate_roster_from_cw(_cw: u32) {
    println!("\nSorry, this feature is not yet implemented\n");
}
