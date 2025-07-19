use std::io::Cursor;

use crate::{
    hash::{add_hash, detect_cc},
    save::SaveFile,
    stream::{Readable, WritableNoOptions},
};

pub mod blocks;
pub mod country_code;
pub mod game;
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

    let mut save_file = SaveFile::read(&mut reader, cc.into()).unwrap();

    println!("{}", save_file.save.catfood);

    save_file
        .save
        .story_chapters
        .clear_chapter(game::main_story::ClearChapterOptions {
            chapter: game::main_story::StoryChapterType::Eoc(
                game::main_story::InnerChapterType::First,
            ),
            clear_amount: 1,
            add_to_clears: false,
        });
    save_file
        .save
        .story_chapters
        .clear_chapter(game::main_story::ClearChapterOptions {
            chapter: game::main_story::StoryChapterType::Eoc(
                game::main_story::InnerChapterType::Second,
            ),
            clear_amount: 2,
            add_to_clears: false,
        });
    save_file
        .save
        .story_chapters
        .clear_chapter(game::main_story::ClearChapterOptions {
            chapter: game::main_story::StoryChapterType::Eoc(
                game::main_story::InnerChapterType::Third,
            ),
            clear_amount: 3,
            add_to_clears: false,
        });
    save_file
        .save
        .story_chapters
        .clear_chapter(game::main_story::ClearChapterOptions {
            chapter: game::main_story::StoryChapterType::Itf(
                game::main_story::InnerChapterType::First,
            ),
            clear_amount: 4,
            add_to_clears: false,
        });
    save_file
        .save
        .story_chapters
        .clear_chapter(game::main_story::ClearChapterOptions {
            chapter: game::main_story::StoryChapterType::Itf(
                game::main_story::InnerChapterType::Second,
            ),
            clear_amount: 5,
            add_to_clears: false,
        });

    // save_file.save.story_chapters.clear_all(
    //     ClearAllChaptersOptions::default()
    //         .with_clear_amount(10)
    //         .with_add_to_clears(false),
    // );

    let mut writer = Cursor::new(Vec::new());

    save_file.write_no_opts(&mut writer).unwrap();

    let new_data = add_hash(&writer.into_inner(), cc).unwrap();

    std::fs::write("./SAVE_DATA", &new_data).unwrap();
}
