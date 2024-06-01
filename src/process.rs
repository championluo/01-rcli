use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
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
    let mut ret_vec: Vec<Player> = Vec::with_capacity(128); // let records = reader.deserialize()
                                                            // .map(|record| record?)>
                                                            // .collect::<Vec<Player>>();
    for result in reader.deserialize() {
        let record: Player = result?;
        // println!("{:?}", record);
        ret_vec.push(record);
    }
    // println!("{:?}",records);
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
