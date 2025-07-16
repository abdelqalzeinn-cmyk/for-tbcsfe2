use std::io::Cursor;

use crate::{
    game_version::GameVersion,
    hash::{add_hash, detect_cc},
    save::{GVCC, Save},
    stream::{Readable, ReadableNoOptions, Writable, WritableNoOptions},
};

pub mod blocks;
pub mod country_code;
pub mod game_version;
pub mod hash;
pub mod save;
pub mod stream;

fn main() {
    let data = include_bytes!("/home/henry/Documents/bcsfe/saves/SAVE_DATA");
    // let data = include_bytes!("/home/henry/Documents/bcsfe/saves/transfer_backup");

    let cc = detect_cc(data).unwrap();

    dbg!(cc);
    let mut reader = Cursor::new(data);

    let gv = GameVersion::read_no_opts(&mut reader).expect("gv");

    let gvcc = GVCC { cc: cc.into(), gv };

    let save = Save::read(&mut reader, gvcc).expect("aaa");

    // let dbg_str = format!("{save:#?}");

    // println!("{}", dbg_str);

    println!("{}", save.catfood);

    let mut writer = Cursor::new(Vec::new());

    gv.write_no_opts(&mut writer).unwrap();

    save.write(&mut writer, gvcc).unwrap();

    dbg!(&save.remaing_data);

    let new_data = add_hash(&writer.into_inner(), cc).unwrap();

    std::fs::write("./SAVE_DATA", &new_data).unwrap();

    let mut new_reader = Cursor::new(new_data);

    let gv2 = GameVersion::read_no_opts(&mut new_reader).unwrap();

    let save_2 = Save::read(&mut new_reader, gvcc).unwrap();

    println!("{}", save_2.catfood);

    // dbg!(save_2);

    // let save_file = SaveFile::from_data(CountryCode::En, data).expect("aaa");
}
