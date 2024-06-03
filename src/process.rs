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

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret_vec = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    // for result in reader.deserialize() {
    for result in reader.records() {
        let record = result?;
        // println!("{:?}", record);
        //zip的作用是将两个迭代器的元素组合成一个新的迭代器【header， record】
        //collect::<Value>()-> 将元组的迭代器转换为json对象
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret_vec.push(json_value);
    }
    let to_string_pretty = serde_json::to_string_pretty(&ret_vec)?;
    fs::write(output, to_string_pretty)?;
    Ok(())
}
