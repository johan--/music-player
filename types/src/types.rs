use std::time::Duration;

use tantivy::{
    schema::{Schema, SchemaBuilder, STORED, TEXT},
    Document,
};

#[derive(Debug, Clone, Default)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
    pub year: Option<u32>,
    pub track: Option<u32>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub bit_depth: Option<u8>,
    pub channels: Option<u8>,
    pub duration: Duration,
    pub uri: Option<String>,
    pub cover: Option<String>,
    pub album_artist: String,
}

#[derive(Debug, Clone, Default)]
pub struct SimplifiedSong {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
}

#[derive(Debug, Clone, Default)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub artist_id: Option<String>,
    pub year: Option<u32>,
    pub cover: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub picture: Option<String>,
}

impl From<Document> for Album {
    fn from(doc: Document) -> Self {
        let mut schema_builder: SchemaBuilder = Schema::builder();

        let id_field = schema_builder.add_text_field("id", TEXT | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let artist_field = schema_builder.add_text_field("artist", TEXT | STORED);
        let year_field = schema_builder.add_i64_field("year", STORED);
        let cover_field = schema_builder.add_text_field("cover", TEXT);

        let id = doc
            .get_first(id_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();
        let title = doc
            .get_first(title_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();
        let artist = doc
            .get_first(artist_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();
        let year = Some(doc.get_first(year_field).unwrap().as_i64().unwrap() as u32);
        let cover = match doc.get_first(cover_field) {
            Some(cover) => Some(cover.as_text().unwrap().to_string()),
            None => None,
        };
        Self {
            id,
            title,
            artist,
            year,
            cover,
            ..Default::default()
        }
    }
}

impl From<Document> for Artist {
    fn from(doc: Document) -> Self {
        let mut schema_builder: SchemaBuilder = Schema::builder();

        let id_field = schema_builder.add_text_field("id", TEXT | STORED);
        let name_field = schema_builder.add_text_field("name", TEXT | STORED);

        let id = doc
            .get_first(id_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();
        let name = doc
            .get_first(name_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();

        Self {
            id,
            name,
            ..Default::default()
        }
    }
}

impl From<Document> for SimplifiedSong {
    fn from(doc: Document) -> Self {
        let mut schema_builder: SchemaBuilder = Schema::builder();

        let id_field = schema_builder.add_text_field("id", TEXT | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let artist_field = schema_builder.add_text_field("artist", TEXT | STORED);
        let album_field = schema_builder.add_text_field("album", TEXT);
        let genre_field = schema_builder.add_text_field("genre", TEXT);

        let id = doc
            .get_first(id_field)
            .unwrap()
            .as_text()
            .unwrap()
            .to_string();

        let title = match doc.get_first(title_field) {
            Some(title) => title.as_text().unwrap().to_string(),
            None => String::from(""),
        };
        let artist = match doc.get_first(artist_field) {
            Some(artist) => artist.as_text().unwrap().to_string(),
            None => String::from(""),
        };
        let album = match doc.get_first(album_field) {
            Some(album) => album.as_text().unwrap().to_string(),
            None => String::from(""),
        };
        let genre = match doc.get_first(genre_field) {
            Some(genre) => genre.as_text().unwrap().to_string(),
            None => String::from(""),
        };
        Self {
            id,
            title,
            artist,
            album,
            genre,
            ..Default::default()
        }
    }
}
