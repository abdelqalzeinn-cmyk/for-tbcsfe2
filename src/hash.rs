use crate::country_code::PatchingCode;

fn split_at_last_32_bytes(data: &[u8]) -> Option<(&[u8], &[u8])> {
    let total = data.len();

    if total <= 32 {
        return None;
    }

    Some(data.split_at(total - 32))
}

fn md5_hash(data: &[u8]) -> [u8; 16] {
    md5::compute(data).0
}

pub fn get_salt(cc: PatchingCode) -> Vec<u8> {
    let code = cc.to_string();

    format!("battlecats{code}").into_bytes()
}

pub fn md5_hash_save(data: &[u8], cc: PatchingCode) -> Option<[u8; 16]> {
    let (save_data, _) = split_at_last_32_bytes(data)?;

    let to_hash = [get_salt(cc).as_slice(), save_data].concat();

    Some(md5_hash(to_hash.as_slice()))
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

pub fn write_hash(data: &[u8], cc: PatchingCode) -> Option<Vec<u8>> {
    let (save_data, _) = split_at_last_32_bytes(data)?;

    let to_hash = [get_salt(cc).as_slice(), save_data].concat();

    let hash = md5_hash(to_hash.as_slice());

    let to_add = encode_hex(&hash).into_bytes();

    Some([save_data, to_add.as_slice()].concat())
}

pub fn add_hash_w<W: std::io::Write + std::io::Seek + std::io::Read>(
    writer: &mut W,
    cc: PatchingCode,
) -> Result<(), std::io::Error> {
    writer.seek(std::io::SeekFrom::End(0))?;
    let end_pos = writer.stream_position()?;

    writer.seek(std::io::SeekFrom::Start(0))?;

    let salt = get_salt(cc);

    let mut data = vec![0; end_pos as usize];

    writer.read_exact(&mut data)?;

    writer.seek(std::io::SeekFrom::End(0))?;

    let to_hash = [salt, data].concat();

    let hash = md5_hash(&to_hash);

    let to_add = encode_hex(&hash).into_bytes();

    writer.write_all(&to_add)?;

    Ok(())
}

pub fn add_hash(data: &[u8], cc: PatchingCode) -> Option<Vec<u8>> {
    let to_hash = [get_salt(cc).as_slice(), data].concat();

    let hash = md5_hash(to_hash.as_slice());

    let to_add = encode_hex(&hash).into_bytes();

    Some([data, to_add.as_slice()].concat())
}

fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    let output: Result<Vec<u8>, _> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();

    output.ok()
}
pub fn verify_md5_hash_save(data: &[u8], cc: PatchingCode) -> bool {
    let output = split_at_last_32_bytes(data);
    if let Some((save_data, current_hash)) = output {
        let string = String::from_utf8_lossy(current_hash);
        let current_hash = decode_hex(&string);

        if let Some(current_hash) = current_hash {
            let to_hash = [get_salt(cc).as_slice(), save_data].concat();
            let actual_hash = md5_hash(to_hash.as_slice());

            current_hash == actual_hash
        } else {
            false
        }
    } else {
        false
    }
}

pub fn detect_cc(data: &[u8]) -> Option<PatchingCode> {
    PatchingCode::ALL
        .into_iter()
        .find(|cc| verify_md5_hash_save(data, *cc))
}
