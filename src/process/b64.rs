use anyhow::Result;
use std::fs::File;
use std::io::Read;

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;

use crate::cli::Base64Format;

pub fn base64_encode(input: &str, format: Base64Format) -> Result<()> {
    // println!("input:{},format:{}", input,format);
    //if-else 返回需要相同类型
    //使用box消除2个返回不同类型的差异
    // let mut reader: Box<dyn Read> = if input == "-" {
    //     //结束输入： window下 按 ctrl + z
    //     Box::new(std::io::stdin())
    // } else {
    //     Box::new(File::open(input)?)
    // };

    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    //解码命令 cargo run -- base64 decode -o temp64.txt --format urlsafe
    //注意，这么写decode会报错 Error: Invalid symbol 10, offset 736.
    //cargo run -- base64 encode -i Cargo.toml --format urlsafe > temp64.txt
    // 使用上面这个命令， 会把 Encoded: 也输出到 文件中， 导致解码失败
    // println!("Encoded: {}", encode);
    println!("{}", encode);
    Ok(())
}

pub fn base64_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    //因为解码的时候，读取到的是一行一行的字符串，需要改成read_to_string,并去掉换行符
    // let mut buf = Vec::new();
    // reader.read_to_end(&mut buf)?;
    let mut buf: String = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    // let decode = URL_SAFE.decode(input)?;
    //TODO: decoded data might not be string (but for this example, we assume it is)
    let decode = String::from_utf8(decode)?;
    print!("{}", decode);
    Ok(())
}

/**
 * 通用获取输入的函数
 */
fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::UrlSafe;
        assert!(base64_encode(input, format).is_ok());
    }

    #[test]
    fn test_base64_decode() {
        let input = "fixtures/temp64.txt";
        let format: Base64Format = Base64Format::UrlSafe;
        assert!(base64_decode(input, format).is_ok());
    }
}
