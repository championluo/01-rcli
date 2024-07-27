use anyhow::Result;
use std::io::Read;

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::prelude::*;

use crate::{cli::Base64Format, get_reader};

pub fn base64_encode(input: &str, format: Base64Format) -> Result<String> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    Ok(encode)
}

pub fn base64_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
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
    Ok(decode)
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
