use chrono::prelude::*;
use colored::*;
use reqwest::{Client, Error};
use std::io::prelude::*;
use std::{fs, env, process, io};

mod structs;

const ID_PATH: &str = "skolmaten-cli-id.txt";
const HELP_MESSAGE: &str = "Du kan använda funktionerna:\nsök <matsal> - söker efter en matsal\nid <matsals-id> - sätter din matsal från id\nvecka <veckonummer> - visar matsedel för specifik vecka";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {

        if fs::metadata(ID_PATH).is_ok() == true {
            let _print_menu = print_menu(Local::now().iso_week().week() as u8);
        } 

    }

    if args.len() == 3 {

	    match args[1].as_str() {

	        "sök" => { let _search = search(&args); },
	        "id" => { let _id = set_id(&args[2]); },
	        "vecka" => { let _vecka = print_menu(args[2].parse::<u8>().unwrap()); },
	        _ => println!("{}", HELP_MESSAGE),

	    }
    }

    println!("{}", HELP_MESSAGE);
    process::exit(0);

}

#[tokio::main]
async fn print_menu(week: u8) -> Result<(), Error> {

    let mut file = fs::File::open(ID_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let local: DateTime<Local> = Local::now();

    let id = contents.trim();
    let req_url = format!(
        "https://skolmaten.se/api/4/menu/?station={}&year={}&weekOfYear={}&count=1",
        id, local.year(), week,
    );

    let resp = Client::new()
        .get(&req_url)
        .header("USER_AGENT", "skolmaten-cli")
        .header("api-version", "4.0")
        .header("client-token", "web")
        .header("client-version-token", "web")
        .header("locale", "sv-SE")
        .send()
        .await?
        .json::<structs::Root>()
        .await?;

    const DAY_NAMES: [&str; 7] = ["Mån", "Tis", "Ons", "Tor", "Fre", "Lör", "Sön"];
    let day_today: usize = (local.weekday().number_from_monday() - 1) as usize;

    println!("\n{}\n", format!("Matsedel v.{}", week).bold());
    
    for week in resp.menu.weeks {

        let mut day_number = 0;

        for day in week.days {
            let mut day_name: ColoredString = DAY_NAMES[day_number].blue();
            if day_number == day_today && week.week_of_year == local.iso_week().week() as u8 {
                day_name = DAY_NAMES[day_number].bright_blue();
            }
            
            if day.meals.is_some() {
                for (i, meal) in day.meals.unwrap().into_iter().enumerate() {
                    if i == 0 {
                        println!("{}: {}", day_name, meal.value);
                    }

                    else {
                        println!("     {}", meal.value);
                    }
                }
            }

            else {
                let reason = day.reason.unwrap();
                println!("{}: {}", day_name, reason);
            }

            println!("");

            day_number += 1;

        }
    }

    process::exit(0);

}

#[tokio::main]
async fn search(args: &Vec<String>) -> Result<(), Error> {

    let query: &String = &args[2];
    println!("Söker efter \"{}\"", query);
    let resp = Client::new()
        .get("https://skolmaten.se/api/4/stations/index/")
        .header("USER_AGENT", "skolmaten-cli")
        .header("api-version", "4.0")
        .header("client-token", "web")
        .header("client-version-token", "web")
        .header("locale", "sv-SE")
        .send()
        .await?
        .json::<structs::RootStation>()
        .await?;

    let mut result_amount: u32 = 0;
    let mut result_id: Vec<i64> = Vec::new();

    for municipality in resp {
        for station in municipality.s {
            if (station.n).to_lowercase().contains(query.to_lowercase().as_str()) {

                result_amount = result_amount + 1;
                result_id.push(station.i);
                println!("{}. {}, ID: {}", result_amount, station.n, station.i);

            }
        }
    }

    print!("Skriv in ditt matsalsnummer, eller tryck [Enter] för att lämna: ");

    io::stdout().flush().expect("Could not flush stdout");

    let mut selected_station = String::new();

    io::stdin().read_line(&mut selected_station).ok().expect("Couldn't read line");
        
    if selected_station.as_bytes().len() == 1 { 
        process::exit(0);
    }

    selected_station = selected_station.chars().filter(|c| c.is_digit(10)).collect();

    let mut selected_station_int: u32 = 0;

    match selected_station.parse::<u32>() {
        Err(_) => exit_program("Numret du angav var ogiltigt"),
        _ => selected_station_int = selected_station.trim().parse::<u32>().unwrap()
    }

    match result_id.get(selected_station_int as usize - 1) {
        None => exit_program("Numret finns inte! Skriv inte in ID, utan numret till vänster."),
        _ => set_id(&result_id[selected_station_int as usize - 1].to_string()),
    }

	process::exit(0);

}

fn set_id(id: &String) {

    println!("Sätter din matsal till \"{}\"", id);

    let mut file = fs::File::create(ID_PATH).expect("create failed");
    file.write_all(id.as_bytes()).expect("write failed");

    process::exit(0);

}

fn exit_program(message: &str) {

    println!("{}: {}", "Error".red(), message);

    process::exit(1);

}