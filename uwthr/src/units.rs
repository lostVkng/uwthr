
// converts temperature
pub fn to_temp(v: &f64, units: &str) -> i32 {

    // convert to temp in different unit
    let mut x = v.to_owned();

    match units {
        "c" => {
            x = x - 273.15;
        },
        "f" => {
            x = (x * 1.8) - 459.67;
        },
        _ => {},
    }

    // round to integer
    return x.round() as i32;
}

// converts speed
pub fn to_speed(v: &f64, units: &str) -> i32 {

    // convert to temp in different unit
    let mut x = v.to_owned();

    match units {
        "f" => {
            x = x * 2.23694;
        },
        _ => {},
    }

    // round to integer
    return x.round() as i32;
}