use serde::Deserialize;
use reqwest::Error;


#[derive(Deserialize, Debug)]
pub struct IPGeo {
    pub lat: f64,
    pub lon: f64,
}

// get lat & lon from api
pub fn get() -> Result<IPGeo, Error> {

    let response: IPGeo = reqwest::blocking::get("http://ip-api.com/json")?.json()?;

    Ok(response)
}