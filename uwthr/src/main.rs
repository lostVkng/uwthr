use std::env;

// create modules
mod ip;
mod forecast;
mod ui;
mod ascaii;
mod units;
mod config;

use forecast::ForecastReq;

fn main() {

    // parse CLI arguments
    let mut args: Vec<String> = env::args().collect();

    // drop cli app arg
    args.remove(0);

    // print version
    if args.iter().any(|r| r == "--version") {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("{}", VERSION);
        return;
    }

    // units for display: f / c / k
    let mut units: &str = "f";

    // check if -c flag is included (and remove)
    if let Some(i) = args.iter().position(|r| r == "-c") {
        units = "c";
        args.remove(i);
    }

    // check if -k flag is included (and remove)
    if let Some(i) = args.iter().position(|r| r == "-k") {
        units = "k";
        args.remove(i);
    }

    // set/update api key
    if let Some(i) = args.iter().position(|r| r == "-set") {
        args.remove(i);
        if args.len() > 0 {
            let res = config::set(args[0].to_owned());

            match res {
                Ok(_) => println!("api key added!"),
                Err(_) => println!("Error setting API Key"),
            }
        }
        return;
    }

    // construct forecast request based on CLI input: blank / city name / zipcode
    let forecast_req: ForecastReq;

    if args.is_empty(){

        // get client Lat/Lon Coordinates by IP
        let coords = ip::get();

        match coords {
            Ok(coords) => {
                forecast_req = ForecastReq::Coords {lat: coords.lat, lon: coords.lon};
            },
            Err(_) => {
                println!("Error fetching coordinates");
                return;
            }
        }

    } else if args[0].parse::<u32>().is_ok() {

        // zip code
        let zip = args[0].parse::<u32>().unwrap();
        forecast_req = ForecastReq::Zip(zip);
    } else {
        // assume city
        let s = args[0].to_string();
        forecast_req = ForecastReq::CityName(s);
    }

    // get forecast data
    let fdata = forecast::get(forecast_req);

    match fdata {
        Ok(fdata) => {
            // configure UI
            ui::gen_ui(fdata, units);
        },
        Err(_) => {
            println!("Error fetching weather data");
        }
    }
}
