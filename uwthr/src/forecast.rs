use serde::{Deserialize, Deserializer};
use reqwest::Error;
use crate::config;

#[derive(Debug)]
pub enum ForecastReq {
    Coords {lat: f64, lon: f64},
    CityName(String),
    Zip(u32)
}

#[derive(Debug)]
pub struct ForecastData {
    pub geo: GeoData,
    pub main: String,
    pub description: String,
    pub icon: String,
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_deg: f64,
}

#[derive(Debug)]
pub struct GeoData {
    pub city: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub timezone: f64,
}

// Need to implement custom deserialization bc api JSON structure
impl<'de> Deserialize<'de> for ForecastData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {

        #[derive(Deserialize, Debug)]
        struct Outer {
            coord: Coord,
            weather: Vec<Weather>,
            main: Main,
            wind: Wind,
            name: String,
            timezone: f64,
            sys: Sys,
        }

        #[derive(Deserialize, Debug)]
        struct Coord {
            pub lat: f64,
            pub lon: f64,
        }

        #[derive(Deserialize, Debug)]
        struct Weather {
            pub id: i64,
            pub main: String,
            pub description: String,
        }

        #[derive(Deserialize, Debug)]
        struct Main {
            pub temp: f64,
            pub feels_like: f64,
            pub temp_min: f64,
            pub temp_max: f64,
            pub pressure: f64,
            pub humidity: f64,
        }

        #[derive(Deserialize, Debug)]
        struct Wind {
            pub speed: f64,
            pub deg: f64,
        }

        #[derive(Deserialize, Debug)]
        struct Sys {
            pub country: String,
        }

        let data: Outer = Outer::deserialize(deserializer)?;      

        let geo: GeoData = GeoData {
            city: data.name,
            country: data.sys.country,
            lat: data.coord.lat,
            lon: data.coord.lon,
            timezone: data.timezone,
        };

        // select correct weather icon
        let wicon: String;

        // map weather codes to ascaii icons
        let code = data.weather[0].id;
        match code {
            200..=232 => wicon = String::from("hrain"),
            300..=321 => wicon = String::from("lrain"),
            500..=531 => wicon = String::from("lrain"),
            600..=622 => wicon = String::from("snow"),
            701..=781 => wicon = String::from("atmosphere"),
            800 => wicon = String::from("sun"),
            801..=804 => wicon = String::from("cloudy"),
            _ => wicon = String::from("sun"),
        }

        // Return data struct
        Ok(ForecastData {
            geo,
            main: data.weather[0].main.to_owned(),
            description: data.weather[0].description.to_owned(),
            icon: wicon,
            temp: data.main.temp,
            feels_like: data.main.feels_like,
            temp_min: data.main.temp_min,
            temp_max: data.main.temp_max,
            pressure: data.main.pressure,
            humidity: data.main.humidity,
            wind_speed: data.wind.speed,
            wind_deg: data.wind.deg,
        })
    }
}

pub fn get(req: ForecastReq) -> Result<ForecastData, Error> {
    
    let mut url:String = "http://api.openweathermap.org/data/2.5/weather?".to_owned();

    // build URL based on request type
    match req {
        ForecastReq::Coords { lat, lon } => {

            url.push_str("lat=");
            url.push_str(&lat.to_string());
            url.push_str("&lon=");
            url.push_str(&lon.to_string());
        },
        ForecastReq::CityName(x) => {

            url.push_str("q=");
            url.push_str(&x.to_string());
        },
        ForecastReq::Zip(x) => {

            url.push_str("zip=");
            url.push_str(&x.to_string());
            url.push_str(",us");
        },
    }

    // get the API key
    let myconfig = config::read();
    let mut key = String::from("");

    match myconfig {
        Ok(c) => key = c.api_key,
        Err(e) => {
            println!("{:?}", e);
            println!("Cannot read API Key")
        },
    }

    // add the api key
    url.push_str("&appid=");
    url.push_str(&key);

    // send API request
    let resp: ForecastData = reqwest::blocking::get(&url)?.json()?;

    Ok(resp)
}