use std::env;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::OpenOptions;

use chrono::Datelike;

static mut COLLEAGUES_RAW: String = String::new();
static mut COLLEAGUES: Vec<&'static str> = vec![];

fn main() -> std::io::Result<()> {
    create_dir_all("db")?;
    unsafe {
        fetch_employees();
    }

    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => print_volunteer_for_cw(),
        Some(x) if x == "seed" => generate_sample_db(),
        Some(x) if x == "populate" => populate_roster(),
        Some(x) if x == "employees" => unsafe { print_employees() },
        Some(x) if x == "next" => unsafe { print_next_name(args.get(2)) },
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
    if let Some((first, _)) = COLLEAGUES.split_first() {
        // remove the 1st row if it's recognized as a special "header row"
        // by containing the literal text "employee_id"
        if first == &"employee_id" {
            COLLEAGUES.remove(0);
        }
    }
}

unsafe fn print_employees() {
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
    // @TODO
    // - find 'cw' from roster db e.g. with `get_volunteer_for_cw()`
    // - consider the value of 'cw':
    //     - if it's 'unknown', go back by -1 until we get a name or we reach the 1st row & go with the 1st name
    //     - if it's anything else, get next name and write next line, repeating the process for x rows
    // - write the result into file
}

unsafe fn get_next_name(cur_name: &String) -> &str {
    let first = COLLEAGUES.first().unwrap_or(&"");
    let mut it = COLLEAGUES.iter();
    let _ = it.position(|&q| q == cur_name).unwrap_or(0);
    // `.position()` seems to move the cursor as well, so
    // right now calling `.next()` would actually get us
    // the next one after `cur_name`
    it.next().unwrap_or(first)
}

unsafe fn print_next_name(given_name: Option<&String>) {
    let def_name = &String::from("");
    let next_name = get_next_name(given_name.unwrap_or(def_name));
    if next_name != "" {
        println!("{}", next_name);
    } else {
        println!("empty db");
    }
}

fn generate_sample_db() {
    println!("\nSorry, this feature is not yet implemented\n");
    // @TODO write the following lines into `db/colleagues.csv` if that file is empty
    // ```
    // employee_id
    // tom
    // harry
    // hermione
    // lucious
    // ron
    // ```
}
