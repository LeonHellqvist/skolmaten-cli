use chrono::prelude::*;
use colored::*;
use reqwest::{Client, Error};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::io::prelude::*;
use std::fs;

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
    pub week_of_year: i64,
    pub year: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    pub reason: Option<String>,
    pub month: i64,
    pub day: i64,
    pub year: i64,
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
const HELP_MESSAGE: &str = "Du kan använda funktionerna:\nsök <matsal> - söker efter en matsal\nid <matsals-id> - sätter din matsal från id";

fn main() {
    let args: Vec<String> = std::env::args().collect();


    if args.len() == 1 {

        if fs::metadata(ID_PATH).is_ok() == true {
            let _print_menu = print_menu();
        } 

        if fs::metadata(ID_PATH).is_ok() == false {
            println!("{}", HELP_MESSAGE);
        }

    }

    if args.len() > 1 {

        let query: &String = &args[1];

        if query == "sök" {
            let _search = search(&args);
        }
        if query == "id" {
            let _id = set_id(&args);
        }

        println!("{}", HELP_MESSAGE);

    }
}

#[tokio::main]
async fn print_menu() -> Result<(), Error> {
    let mut file = std::fs::File::open(ID_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let local: DateTime<Local> = Local::now();

    let id = contents.trim();
    let req_url = format!(
        "https://skolmaten.se/api/4/menu/?station={}&year={}&weekOfYear={}&count=1",
        id, local.year(), local.iso_week().week(),
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

    let day_names = ["Mån", "Tis", "Ons", "Tor", "Fre", "Lör", "Sön"];
    let day_today: usize = (local.weekday().number_from_monday() - 1)
        .try_into()
        .unwrap();

    println!("{}", "------------------------------------------".black());
    for week in resp.menu.weeks {
        let mut day_number = 0;
        for day in week.days {
            if day.meals.is_some() {
                for meal in day.meals.unwrap() {
                    let day_name: ColoredString;
                    if day_number == day_today {
                        day_name = day_names[day_number].bright_blue();
                    } else {
                        day_name = day_names[day_number].blue();
                    }
                    println!("{}: {}", day_name, meal.value);
                }
            } else {
                let reason = day.reason.unwrap();
                println!("{}: {}", day_names[day_number], reason);
            }
            println!("{}", "------------------------------------------".black());
            day_number += 1;
        }
    }

    Ok(())
}

#[tokio::main]
async fn search(args: &Vec<String>) -> Result<(), Error> {
    if args.len() != 3 {
        println!("Du måste söka på en matsal");
        println!("Använd: ./skolmaten sök <matsal>");
    } else {
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
        for municipally in resp {
            for station in municipally.s {
                if (station.n)
                    .to_lowercase()
                    .contains(query.to_lowercase().as_str())
                {
                    println!("{}, ID: {}", station.n, station.i);
                }
            }
        }
    }
    std::process::exit(1);
}

fn set_id(args: &Vec<String>) -> std::io::Result<()> {
    if args.len() != 3 {
        println!("Du måste ange ett matsals-id");
        println!("Använd: ./skolmaten id <matsals-id>");
    } else {
        let query: &String = &args[2];
        println!("Sätter din matsal till \"{}\"", query);

        let mut file = std::fs::File::create(ID_PATH).expect("create failed");
        file.write_all(query.as_bytes()).expect("write failed");
    }
    std::process::exit(1);
}
