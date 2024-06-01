use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    // #[serde(rename = "Name")]
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

// #[command(name="csv_proc")]
pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret_vec = Vec::with_capacity(128); // let records = reader.deserialize()
                                               // .map(|record| record?)>
                                               // .collect::<Vec<Player>>();
                                               //这里borrow了reader 2个mutable reference
                                               //通过使用clone() 来消除重复可变引用的影响
    let headers = reader.headers()?.clone();
    // for result in reader.deserialize() {
    for result in reader.records() {
        let record = result?;
        // println!("{:?}", record);
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret_vec.push(json_value);
    }
    // for result in reader.deserialize() {
    //     // Notice that we need to provide a type hint for automatic
    //     // deserialization.
    //     let record: Player = result?;
    //     println!("{:?}", record);
    // }
    let to_string_pretty = serde_json::to_string_pretty(&ret_vec)?;
    fs::write(output, to_string_pretty)?;
    Ok(())
}
