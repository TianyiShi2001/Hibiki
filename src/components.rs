use crate::db::DB;
use glob::glob;
use id3::{Tag, Version};
use rusqlite::params;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Library {}

impl Library {
    pub fn init() {
        Self::from_dirs(&super::get_hibiki_dirs())
    }
    pub fn from_dirs(dirs: &[String]) {
        for ref dir in dirs {
            let pattern = dir.to_owned().clone() + "/**/*.mp3";
            for path in glob(&pattern).expect("fail to read glob pattern") {
                match path {
                    Ok(path) => {
                        let title_fallback = path
                            //.as_ref()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string();
                        let path_string = path.as_path().display().to_string();
                        let mut title: String = title_fallback;
                        let mut artist: Option<String> = None;
                        let mut album_id: Option<u32> = None;
                        if let Ok(tags) = Tag::read_from_path(&path) {
                            if let Some(t) = tags.album().map(String::from) {
                                title = t;
                                artist = tags.album_artist().map(String::from);
                                let db = DB.lock().unwrap();

                                let stmt = if let Some(a) = &artist {
                                    format!(
                                        "SELECT id FROM albums WHERE title = '{}' AND artist = '{}'",
                                        &title, a
                                    )
                                } else {
                                    format!(
                                        "SELECT id FROM albums WHERE title = '{}' AND artist IS NULL",
                                        &title
                                    )
                                };
                                album_id = if let Ok(album_id) = db.query_row(
                                    &stmt,     //  AND artist = '?2'
                                    params![], // params![title, artist]
                                    |row| row.get(0),
                                ) {
                                    println!("1");
                                    println!("{}", &title);
                                    Some(album_id)
                                } else {
                                    println!("2");
                                    println!("{}", &title);
                                    let cover = match &tags.pictures().next() {
                                        Some(ref pic) => Some(pic.data.clone()),
                                        None => None,
                                    };
                                    db.execute(
                                            "INSERT INTO albums (title, artist, cover) VALUES (?, ?, ?)",
                                            params![title, artist, cover],
                                        )
                                        .unwrap();
                                    Some(db.last_insert_rowid() as u32)
                                }
                            }
                        }
                        Library::exec(
                            "INSERT INTO songs (path, title, artist, album_id) VALUES (?, ?, ?, ?)",
                            params![path_string, title, artist, album_id],
                        )
                        .unwrap();
                    }
                    Err(e) => println!("{:?}", e),
                };
            }
        }
    }

    pub fn exec<P>(stmt: &str, params: P) -> rusqlite::Result<usize>
    where
        P: IntoIterator,
        P::Item: rusqlite::ToSql,
    {
        DB.lock().unwrap().execute(stmt, params)
    }
}

#[derive(Debug)]
pub struct Album {
    pub id: u32,
    pub title: String,
    pub artist: Option<String>,
    pub cover: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct Song {
    pub id: u32,
    pub path: String,
    pub title: String,
    pub artist: Option<String>,
    pub album_id: Option<u32>,
}

// impl<T: AsRef<Path>> Song<T> {
//     pub fn from_path(path: T) -> Self {
//         if let Ok(tags) = Tag::read_from_path(&path) {
//             let album = match tags.album() {
//                 Some(name) => {
//                     let cover = match &tags.pictures().next() {
//                         Some(ref pic) => Some(pic.data.clone()),
//                         None => None,
//                     };
//                     Some(Album {
//                         title: name.to_owned(),
//                         artist: tags.album_artist().map(String::from),
//                         cover,
//                     })
//                 }
//                 None => None,
//             };
//             Self {
//                 path,
//                 title: tags.title().map(|x| x.to_owned()).unwrap(),
//                 artist: tags.artist().map(|x| x.to_owned()),
//                 album,
//             }
//         } else {
//             let title = path
//                 .as_ref()
//                 .file_name()
//                 .unwrap()
//                 .to_string_lossy()
//                 .to_string();
//             Self {
//                 path,
//                 title,
//                 artist: None,
//                 album: None,
//             }
//         }
//     }
// }
