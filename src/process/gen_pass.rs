use rand::seq::SliceRandom;
use zxcvbn::zxcvbn; //注意这里一定要引入zxcvbn库

// use crate::opts::GenPassOpts;
// pub const UPPER =

const UPPER: &[u8; 25] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8; 25] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8; 9] = b"123456789"; //去掉数字0
const SYMBOL: &[u8; 9] = b"!@#$%^&*_"; //选择少量不会引发歧义的symbols
                                       // chars.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;:,.<>?");

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut thread_rng = rand::thread_rng();
    // let mut password = String::new();
    let mut password_vec = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        //去掉大写的I和O，视觉上引发歧义的字母删除
        chars.extend_from_slice(UPPER);
        //注意这里要 *是解引用, 因为UPPER 是应用类型
        password_vec.push(
            *UPPER
                .choose(&mut thread_rng)
                .expect("Ignore Null Exception"),
        );
    }
    if lowercase {
        //去掉小写的l
        chars.extend_from_slice(LOWERCASE);
        password_vec.push(
            *LOWERCASE
                .choose(&mut thread_rng)
                .expect("Ignore Null Exception"),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password_vec.push(
            *NUMBER
                .choose(&mut thread_rng)
                .expect("Ignore Null Exception"),
        );
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password_vec.push(
            *SYMBOL
                .choose(&mut thread_rng)
                .expect("Ignore Null Exception"),
        );
    }

    //注意这里的length 需要减去password已经有的长度
    for _ in 0..(length as usize - password_vec.len()) {
        // let index = thread_rng.gen_range(0..chars.len());
        // password.push(chars[index] as char);
        let c = chars.choose(&mut thread_rng).expect("empty char set");
        // password.push(*c as char); //这里使用 *c 解引用, 又因为chars是u8类型，所以需要转成char类型，
        //    且无需clone， u8类型解引用的时候会自动拷贝
        password_vec.push(*c)
    }

    //最后,因为password有确定的模式, 即前4个字符是固定组合, 这里重新洗牌
    password_vec.shuffle(&mut thread_rng);

    let password = String::from_utf8(password_vec)?;
    println!("Password: {}", password);

    let estimate = zxcvbn(&password, &[]);
    //注意这里使用的是标准错误数据
    // cargo run -- genpass -l 16 > out.txt
    //这里打印到txt文件中只有上面的 标准输出, 这里的标准错误输出只会在终端显示
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}
