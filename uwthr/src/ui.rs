use crate::forecast::ForecastData;
use crate::ascaii;
use crate::units::{to_temp, to_speed};


pub fn gen_ui(data: ForecastData, units: &str) {

    let mut t_str: String = String::from("Weather: ");

    // city + country
    t_str.push_str(&data.geo.city);
    t_str.push_str(", ");
    t_str.push_str(&data.geo.country);
    t_str.push_str("\n");

    // weather icon?
    let wicon: [&'static str; 5];

    match data.icon.as_str() {
        "sun" =>  wicon = ascaii::SUN,
        "cloudy" =>  wicon = ascaii::CLOUDY,
        "atmosphere" =>  wicon = ascaii::ATMOSPHERE,
        "lrain" =>  wicon = ascaii::LRAIN,
        "hrain" =>  wicon = ascaii::HRAIN,
        "snow" =>  wicon = ascaii::SNOW,
        _ => wicon = ascaii::SUN,
    }

    // line 1
    t_str.push_str(wicon[0]);
    t_str.push_str(&data.description);
    t_str.push_str("\n");

    // line 2
    t_str.push_str(wicon[1]);
    let temp = to_temp(&data.temp, units);
    t_str.push_str(&temp.to_string());
    t_str.push_str("°");
    t_str.push_str(units);
    t_str.push_str(" - feels like ");
    let feel_temp = to_temp(&data.feels_like, units);
    t_str.push_str(&feel_temp.to_string());
    t_str.push_str("°");
    t_str.push_str(units);
    t_str.push_str("\n");

    // line 3
    t_str.push_str(wicon[2]);
    t_str.push_str("H/L - ");
    let h_temp = to_temp(&data.temp_max, units);
    t_str.push_str(&h_temp.to_string());
    t_str.push_str("°");
    t_str.push_str(units);
    t_str.push_str(" / ");
    let l_temp = to_temp(&data.temp_min, units);
    t_str.push_str(&l_temp.to_string());
    t_str.push_str("°");
    t_str.push_str(units);
    t_str.push_str("\n");

    // line 4
    t_str.push_str(wicon[3]);
    t_str.push_str("Wind - ");
    let wind_speed = to_speed(&data.wind_speed, units);
    t_str.push_str(&wind_speed.to_string());

    if units == "f" {
        t_str.push_str("mph");
    } else {
        t_str.push_str("m/s");
    }
    t_str.push_str("\n");

    // line 5
    t_str.push_str(wicon[4]);
    t_str.push_str("Humidity - ");
    let humidity = &data.humidity;
    t_str.push_str(&humidity.to_string());
    t_str.push_str("%");

    // print to terminal
    println!("{}", t_str);
}