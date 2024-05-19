use std::env;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

use chrono::{Datelike, Days, Weekday};

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
        Some(x) if x == "employees" => print_employees(),
        Some(x) if x == "next" => print_next_name(args.get(2)),
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
    let (file_content, _) = safely_read_file("db/colleagues.csv", false);
    COLLEAGUES_RAW.push_str(file_content.trim());
    COLLEAGUES = COLLEAGUES_RAW.split('\n').collect();
    if let Some((first, _)) = COLLEAGUES.split_first() {
        // remove the 1st row if it's recognized as a special "header row"
        // by containing the literal text "employee_id"
        if first == &"employee_id" {
            COLLEAGUES.remove(0);
        }
    }
}

fn print_employees() {
    println!("┌────────────────────────────────┐");
    unsafe {
        for c in &COLLEAGUES {
            println!("│ {}", *c);
        }
    }
    println!("└────────────────────────────────┘");
}

fn get_current_year() -> i32 {
    chrono::Utc::now().year()
}

fn get_current_cw() -> u32 {
    chrono::Utc::now().iso_week().week()
}

fn safely_read_file(filepath: &str, append_mode: bool) -> (String, Option<File>) {
    let file = OpenOptions::new()
        .write(true)
        .append(append_mode)
        .create(append_mode)
        .open(filepath);
    let ret_val = match file {
        Ok(f) => (
            read_to_string(filepath).unwrap_or(String::from("")),
            Some(f),
        ),
        Err(_) => (String::from(""), None),
    };
    ret_val
}

fn get_volunteer_for_cw(cw: String) -> String {
    let cur_year = get_current_year();
    let mut target_row = cur_year.to_string() + "," + &cw;
    let (roster_raw, _) = safely_read_file("db/roster.csv", false);
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

fn populate_roster_from_cw(cur_week: u32) {
    let weeks = 12u8; // populate for how many weeks ahead
    let (roster_raw, file) = safely_read_file("db/roster.csv", true);
    let def_cur_name = get_volunteer_for_cw(cur_week.to_string());

    let def_time = chrono::Utc::now();
    let def_year = def_time.year();
    let def_year_str = def_year.to_string();
    let def_week = def_time.iso_week().week();
    let def_week_str = def_week.to_string();

    let mut target_line = get_current_year().to_string() + ",";
    target_line.push_str(&cur_week.to_string());
    let cur_line = roster_raw
        .split('\n')
        .find(|row| row.starts_with(&target_line))
        .unwrap_or(&"");
    let cur_line_cmp = cur_line.split(',').collect::<Vec<&str>>();

    let year_str = cur_line_cmp
        .get(0)
        .unwrap_or(&&def_year_str.as_str())
        .to_owned();
    let week_str = cur_line_cmp
        .get(1)
        .unwrap_or(&&def_week_str.as_str())
        .to_owned();

    // value of `cur_name`, `cur_year` and `cur_week` will be mutable when the for..loop below runs
    let cur_name = &mut cur_line_cmp
        .get(2)
        .unwrap_or(&def_cur_name.as_str())
        .to_owned();
    let cur_year = &mut year_str.parse().unwrap_or(def_year);
    let cur_week = &mut week_str.parse().unwrap_or(def_week);

    let mut buf = BufWriter::new(file.unwrap());
    for _ in 1..weeks {
        let next_name = unsafe { get_next_name(*cur_name) };

        // get year & week value for next line, without exceeding the highest week for that year
        let cur = chrono::NaiveDate::from_isoywd_opt(*cur_year, *cur_week, Weekday::Mon).unwrap();
        let date_of_next_week = cur.checked_add_days(Days::new(7)).unwrap();
        let (_, mut year_of_next_week) = date_of_next_week.year_ce();
        // `checked_add_days` doesn't currently handle "leap year" correctly (a bug in `chrono` crate?)
        // so we need a manual workaround
        let should_manually_advance_year =
            if cur.leap_year() && (*cur_week == 52 || *cur_week == 53) {
                true
            } else {
                false
            };
        if should_manually_advance_year {
            year_of_next_week += 1;
        }
        let next_week = date_of_next_week.iso_week().week();

        let mut next_line =
            year_of_next_week.to_string() + "," + next_week.to_string().as_str() + ",";

        next_line.push_str(next_name);
        writeln!(buf, "{}", next_line).unwrap();

        // move up the values to prepare for the next loop
        *cur_name = next_name;
        *cur_week = next_week;
        *cur_year = year_of_next_week as i32;
    }
}

unsafe fn get_next_name(cur_name: &str) -> &str {
    let first = COLLEAGUES.first().unwrap_or(&"");
    let mut it = COLLEAGUES.iter();
    let _ = it.position(|&q| q == cur_name).unwrap_or(0);
    // `.position()` seems to move the cursor as well, so
    // right now calling `.next()` would actually get us
    // the next one after `cur_name`
    it.next().unwrap_or(first)
}

fn print_next_name(given_name: Option<&String>) {
    let def_name = &String::from("");
    let next_name = unsafe { get_next_name(given_name.unwrap_or(def_name)) };
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
