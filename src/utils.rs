use anyhow::Result;
use std::{fs::File, io::Read, path::Path};

/**
 * 校验； 如果输入是文件路径，则校验路径是否存在
 */
pub fn valid_path(filename: &str) -> Result<String, &'static str> {
    //改造，以适配 base64 的标准输入“-”
    //字符串直接用等比较
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

/**
 * 通用获取输入的函数
 */
pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
