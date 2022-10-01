use chrono::prelude::*;
use colored::*;
use reqwest::{Client, Error};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::io::prelude::*;
use std::{fs, env, process, io};

pub type RootStation = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub s: Vec<GeneratedType>,
    pub n: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedType {
    pub i: i64,
    pub n: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub menu: Menu,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub is_feedback_allowed: bool,
    pub weeks: Vec<Week>,
    pub station: Station,
    pub id: i64,
    pub bulletins: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub days: Vec<Day>,
    pub week_of_year: u8,
    pub year: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub reason: Option<String>,
    pub month: u8,
    pub day: u8,
    pub year: i16,
    pub meals: Option<Vec<Meal>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meal {
    pub attributes: Vec<Value>,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub url_name: String,
    pub id: i64,
    pub district: District,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct District {
    pub province: Province,
    pub url_name: String,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Province {
    pub url_name: String,
    pub id: i64,
    pub name: String,
}

const ID_PATH: &str = "skolmaten-cli-id.txt";
const HELP_MESSAGE: &str = "Du kan använda funktionerna:\nsök <matsal> - söker efter en matsal\nid <matsals-id> - sätter din matsal från id\nvecka <veckonummer> - visar matsedel för specifik vecka";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {

        if fs::metadata(ID_PATH).is_ok() == true {
            let _print_menu = print_menu(Local::now().iso_week().week().try_into().unwrap());
        } 

        println!("{}", HELP_MESSAGE);

    }

    if args.len() > 1 {

	match args[1].as_str() {

	    "sök" => { let _search = search(&args); },
	    "id" => { let _id = set_id(&args); },
	    "vecka" => { let _vecka = print_menu(args[2].parse::<u8>().unwrap()); },
	    _ => println!("{}", HELP_MESSAGE),

	}
    }
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

    let client = Client::new();
    let resp = client
        .get(&req_url)
        .header("USER_AGENT", "skolmaten-cli")
        .header("api-version", "4.0")
        .header("client-token", "web")
        .header("client-version-token", "web")
        .header("locale", "sv-SE")
        .send()
        .await?
        .json::<Root>()
        .await?;

    const DAY_NAMES: [&str; 7] = ["Mån", "Tis", "Ons", "Tor", "Fre", "Lör", "Sön"];
    let day_today: usize = (local.weekday().number_from_monday() - 1).try_into().unwrap();

    println!("{}", "------------------------------------------".black());
    for week in resp.menu.weeks {
        let mut day_number = 0;
        for day in week.days {
            if day.meals.is_some() {
                for (i, meal) in day.meals.unwrap().into_iter().enumerate() {
                    if i == 0 {
                        let mut day_name: ColoredString = DAY_NAMES[day_number].blue();
                        if day_number == day_today && week.week_of_year == local.iso_week().week() as u8 {
                            day_name = DAY_NAMES[day_number].bright_blue();
                        }
                        println!("{}: {}", day_name, meal.value);
                    } else {
                        println!("     {}", meal.value);
                    }
                }
            } else {
                let reason = day.reason.unwrap();
                println!("{}: {}", DAY_NAMES[day_number], reason);
            }
            println!("{}", "------------------------------------------".black());
            day_number += 1;
        }
    }

    process::exit(0);
}

#[tokio::main]
async fn search(args: &Vec<String>) -> Result<(), Error> {

    if args.len() == 3 {
        let query: &String = &args[2];
        println!("Söker efter \"{}\"", query);
        let client = Client::new();
        let resp = client
            .get("https://skolmaten.se/api/4/stations/index/")
            .header("USER_AGENT", "skolmaten-cli")
            .header("api-version", "4.0")
            .header("client-token", "web")
            .header("client-version-token", "web")
            .header("locale", "sv-SE")
            .send()
            .await?
            .json::<RootStation>()
            .await?;

        let mut result_amount: u32 = 0;
        let mut result_id: Vec<i64> = Vec::new();

        for municipality in resp {
            for station in municipality.s {
                if (station.n)
                    .to_lowercase()
                    .contains(query.to_lowercase().as_str())
                {
                    result_amount = result_amount + 1;
                    result_id.push(station.i);
                    println!("{}. {}, ID: {}", result_amount, station.n, station.i);
                }
            }
        }

        print!("Skriv in ditt matsalsnummer, eller tryck [Enter] för att lämna: ");

        io::stdout().flush().expect("Could not flush stdout");

        let mut selected_station = String::new();

        io::stdin().read_line(&mut selected_station)
            .ok()
            .expect("Couldn't read line");

        
        if selected_station.as_bytes().len() == 1 { 
            process::exit(0);
        }

        selected_station = selected_station.chars().filter(|c| c.is_digit(10)).collect();

        let mut selected_station_int: u32 = 0;

        match selected_station.parse::<u32>() {
            Err(_) => exit_program("Nummeret du angav var ogiltigt"),
            _ => selected_station_int = selected_station.trim().parse::<u32>().unwrap().try_into().unwrap()
        }

        match result_id.get(selected_station_int as usize - 1) {
            None => exit_program("Nummeret finns inte! Skriv inte in ID utan numret till vänster."),
            _ => write_id_file(&result_id[selected_station_int as usize - 1].to_string()),
        }

	process::exit(0);

    }

    println!("Du måste söka på en matsal\nAnvänd: ./skolmaten sök <matsal>");
    process::exit(1);

}

fn write_id_file(id: &String) {

    println!("Sätter din matsal till \"{}\"", id);

    let mut file = fs::File::create(ID_PATH).expect("create failed");
    file.write_all(id.as_bytes()).expect("write failed");

    process::exit(0);
}

fn set_id(args: &Vec<String>) {
    if args.len() != 3 {
        println!("Du måste ange ett matsals-id");
        println!("Använd: ./skolmaten id <matsals-id>");
    } else {
        write_id_file(&args[2]);
    }
    process::exit(0);
}

fn exit_program(message: &str) {

    println!("{}: {}", "Error".red(), message);

    process::exit(1);

}
