use rand::seq::SliceRandom;

// use crate::opts::GenPassOpts;

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut thread_rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    if uppercase {
        //去掉大写的I和O，视觉上引发歧义的字母删除
        chars.extend_from_slice(b"ABCDEFGHJKLMNOPQRSTUVWXYZ");
    }
    if lowercase {
        //去掉小写的l
        chars.extend_from_slice(b"abcdefghijkmnopqrstuvwxyz");
    }
    if number {
        //去掉数字0
        chars.extend_from_slice(b"123456789");
    }
    if symbol {
        //选择少量不会引发歧义的symbols
        // chars.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;:,.<>?");
        chars.extend_from_slice(b"!@#$%^&*_");
    }

    for _ in 0..length {
        // let index = thread_rng.gen_range(0..chars.len());
        // password.push(chars[index] as char);
        let c = chars.choose(&mut thread_rng).expect("empty char set");
        password.push(*c as char); //这里使用 *c 解引用, 又因为chars是u8类型，所以需要转成char类型，
                                   //且无需clone， u8类型解引用的时候会自动拷贝
    }

    println!("Password: {}", password);

    Ok(())
}
