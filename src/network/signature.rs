use hmac::{Hmac, Mac};

pub fn sign<const BLOCK_SIZE: usize, T: Mac + hmac::digest::KeyInit>(
    inquiry_code: &str,
    data: &[u8],
) -> Result<String, std::io::Error> {
    let mut random_data = vec![0; BLOCK_SIZE];
    getrandom::fill(&mut random_data).map_err(|e| std::io::Error::other(e.to_string()))?;

    let random_data = hex::encode(random_data);

    let key = [inquiry_code.as_bytes(), random_data.as_bytes()].concat();

    let mut mac = <T as Mac>::new_from_slice(&key).map_err(std::io::Error::other)?;

    mac.update(data);

    let mac_result = mac.finalize().into_bytes();

    let mut signature = random_data;

    signature.push_str(&hex::encode(mac_result));

    Ok(signature)
}

pub fn sign_v2(inquiry_code: &str, data: &[u8]) -> Result<String, std::io::Error> {
    sign::<32, Hmac<sha2::Sha256>>(inquiry_code, data)
}

pub fn sign_v1(inquiry_code: &str, data: &[u8]) -> Result<String, std::io::Error> {
    let data = [data, data].concat(); // duplicated for some reason
    sign::<20, Hmac<sha1::Sha1>>(inquiry_code, &data)
}
