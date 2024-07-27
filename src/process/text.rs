use crate::{get_reader, process_genpass, TextSignFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};
/**
 * 生成签名接口
 */
trait TextSign {
    // fn sign(&self, data: &str) -> Result<Vec<u8>>;
    // 最开始的接口是这样的, data是文件路径, 接口设计上无关自己的部分最好不要暴露给用户
    //这里的data是从reader中读取的内容,所以直接接口定义成直接使用reader,而不需要关心如何获取reader
    //Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
    //搞不懂, 这里动态引用需要加个mut, 下面的verify却不用?
}

trait TextVerify {
    //这里reader使用的静态分派的写法, 与上面的TextSign接口的 动态分派 的写法有性能和代码体积的差别
    // 和java的 静态代理和动态代理 类似
    // 真实业务代码中 无需考虑 这里的区别, 直接最大的性能开销还是 在io里面
    // 除了可以使用 impl Read 的做法外, 还可以写成泛型方式
    // fn verify<R: Read>(&self, reader: R, sig: &[u8]) -> Result<bool>;

    // Verify the data from the reader with the signature
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
    //搞不懂, 这里为啥不需要加个mut, 和上面的sign方法不一样?
}

trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized; //这个where中的Sized 是个marker trait， 表示一种约束，返回的self一定要是定长的
                     //大部分的数据结构都是定长的, 处理 [u8] ,str 这种是不定长的
}

pub trait KeyGenerator {
    //非对称加密生成的是一对key， 对称加密生成的是1个key
    //如何兼容？ 再在外面包一层Vec， 也就是 Vec<Vec<u8>> 这样
    // 对于blake3， 只要生成1个key
    // 对于ed25， 则要生成一对key
    fn generate() -> Result<Vec<Vec<u8>>>;
}

//定义结构体, 实现这2个impl接口
struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    //如果别人提供了u8；32 则直接构造，如果别人不是u832，则尝试转成u832，再new一个
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32]; //直接使用32会报错
                              // let key = &key[..key.len()]; //这里使用key的长度
        let key: [u8; 32] = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Blake3::try_new(&key)
    }
}

//ed25519 需要拆成 signer 和 verifier
struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Ed25519Signer::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Ed25519Verifier::try_new(&key)
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    // let mut buf = Vec::new();
    // reader.read_to_end(&mut buf)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            // let key = fs::read(key)?;
            // //这里从文件读取有换行付的缘故,进行处理下
            // let key = &key[..32]; //取32位,去掉后面的换行符 与通过genpass生成的key位数相同, 切片类型是 &[u8]
            // let key = key.try_into()?;
            // //unwrap()用于将Result结果转化OK的泛型类型
            // // let key = key.try_into().unwrap();
            // //新建Blake3实例
            // let signer = Blake3 { key };
            let signer = Blake3::load(key)?; //这里可以精简代码
                                             //调用sign方法
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            // let key = fs::read(key)?;
            // let key = &key[..32];
            // let key = key.try_into()?;

            // let key = SigningKey::from_bytes(&key);
            // let ed25519_signer = Ed25519Signer{ key };
            // ed25519_signer.sign(&mut reader)?
            //上面的代码注释, 封装成下面这句代码
            Ed25519Signer::load(key)?.sign(&mut reader)?
        }
    };
    //对输入的数据通过blake3 hash之后,再统一使用base64编码, 最终做成签名
    let encode = URL_SAFE_NO_PAD.encode(signed);
    Ok(encode)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> Result<bool> {
    let reader = get_reader(input)?;
    let sig: Vec<u8> = URL_SAFE_NO_PAD.decode(sig)?;
    let verify_result: bool = match format {
        TextSignFormat::Blake3 => {
            let verify = Blake3::load(key)?;
            verify.verify(reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verify = Ed25519Verifier::load(key)?;
            verify.verify(reader, &sig)?
        }
    };
    Ok(verify_result)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let keyed_hash = &blake3::keyed_hash(&self.key, &buf);
        let bytes = &keyed_hash.as_bytes(); //as_bytes 是转成 u832 数组的方法
        let result = bytes.to_vec(); //to_vec() 转出vec对象
        Ok(result)
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // println!("buf: {:?}",String::from_utf8_lossy(&buf)); //强转成字符串

        //这样写有问题, 原因是 blake3::hash(&buf)生成了实例hash, as_bytes() 只去了这个hash的引用
        // as_bytes() 的源码就是只获取引用 &self.0
        // let hash = 真正hash实例的引用, 当引用获取或, 实例hash会被释放, 导致后面的通过引用获取hash值与 sign比较会失败
        // let hash = blake3::hash(&buf).as_bytes();
        //正确的做法如下:

        // let binding = blake3::hash(&buf);   //增加一个binding指向实例hash //这个代码的问题是在hash的时候没有加入key来一起做hash
        let binding = blake3::keyed_hash(&self.key, &buf); //增加一个binding指向实例hash
        let hash = binding.as_bytes(); //获取这个binding的引用
                                       // println!("new sign: {:?}", binding.clone());
        Ok(hash == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok((self.key.sign(&buf)).to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        //注意这里类型的转换
        let signature = Signature::from_bytes(sig.try_into()?);
        let is_ok = self.key.verify(&buf, &signature).is_ok();
        Ok(is_ok)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        // let key = process_genpass()
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        //注意这里还要调用下verifying_key()方法, 才能拿到私钥
        let pk = sk.verifying_key().as_bytes().to_vec();
        //公钥
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_blake3_sign_and_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;

        let data = b"hello";
        //注意这里 &mut &data[..] 是字符数组转成了切片, 此切片变量匿名, 直接取这个切片的可变应用
        let sign = blake3.sign(&mut &data[..])?;
        println!("{}", URL_SAFE_NO_PAD.encode(sign.clone()));
        let res = blake3.verify(&mut &data[..], &sign)?;
        assert!(res);
        Ok(())
    }
}
