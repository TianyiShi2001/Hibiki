use glob::glob;
use id3::{Tag, Version};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Library {
    pub songs: Vec<Song>,
    pub albums: Vec<Album>,
}

impl Library {
    pub fn init() -> Self {
        Self::from_dirs(&super::get_hibiki_dirs())
    }
    pub fn from_dirs(dirs: &[String]) -> Self {
        // let res: Self = Default::default();
        let mut songs: Vec<Song> = Vec::new();
        let mut albums: Vec<Album> = Vec::new();
        for ref dir in dirs {
            // let dir = dir.to_owned();
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
                        let song = if let Ok(tags) = Tag::read_from_path(&path) {
                            let album = match tags.album().map(String::from) {
                                Some(title) => {
                                    let artist = tags.album_artist().map(String::from);
                                    let mut i = 0;
                                    while i < albums.len() {
                                        let album = &albums[i];
                                        if album.title == title && album.artist == artist {
                                            break;
                                        }
                                        i += 1;
                                    }
                                    if i == albums.len() {
                                        let cover = match &tags.pictures().next() {
                                            Some(ref pic) => Some(pic.data.clone()),
                                            None => None,
                                        };
                                        albums.push(Album {
                                            title,
                                            artist,
                                            cover,
                                        })
                                    }
                                    Some(i)
                                }
                                None => None,
                            };
                            Song {
                                path: path_string,
                                title: match tags.title().map(|x| x.to_owned()) {
                                    Some(x) => x,
                                    None => title_fallback,
                                },
                                artist: tags.artist().map(|x| x.to_owned()),
                                album,
                            }
                        } else {
                            Song {
                                path: path_string,
                                title: title_fallback,
                                artist: None,
                                album: None,
                            }
                        };
                        songs.push(song);
                    }
                    Err(e) => println!("{:?}", e),
                };
            }
        }
        Self { songs, albums }
    }
}

#[derive(Debug)]
pub struct Album {
    pub title: String,
    pub artist: Option<String>,
    pub cover: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct Song {
    path: String,
    title: String,
    artist: Option<String>,
    album: Option<usize>,
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
