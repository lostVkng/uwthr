use std::fs;
use serde_json;
use serde::{Serialize, Deserialize};
use std::io;

extern crate dirs;


#[derive(Serialize, Deserialize, Debug)]
pub struct MyConfig {
    pub api_key: String,
}

// Read existing config file
pub fn read() -> Result<MyConfig, Box<dyn std::error::Error + 'static>> {

    let home = dirs::home_dir().unwrap();
    let _path = home.join(".uwthr");

    let config = fs::read_to_string(_path)?;

    let res: MyConfig = serde_json::from_str(&config)?;

    Ok(res)
}

// set / create config file
pub fn set(key: String) -> Result<(), io::Error> {

    let json: MyConfig = MyConfig {
        api_key: key,
    };

    let s = serde_json::to_string(&json)?;

    let home = dirs::home_dir().unwrap();
    let _path = home.join(".uwthr");

    let _res = std::fs::write(_path, &s);

    Ok(())
}