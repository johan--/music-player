use music_player_types::types::Song;
use sea_orm::{entity::prelude::*, ActiveValue};

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "artist_track")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub artist_id: String,
    pub track_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::artist::Entity",
        from = "Column::ArtistId",
        to = "super::artist::Column::Id"
    )]
    Artist,
    #[sea_orm(
        belongs_to = "super::track::Entity",
        from = "Column::TrackId",
        to = "super::track::Column::Id"
    )]
    Track,
}

impl ActiveModelBehavior for ActiveModel {}

impl From<&Song> for ActiveModel {
    fn from(song: &Song) -> Self {
        Self {
            id: ActiveValue::set(format!(
                "{:x}",
                md5::compute(format!("{}{}", song.artist, song.uri.as_ref().unwrap()))
            )),
            artist_id: ActiveValue::Set(format!(
                "{:x}",
                md5::compute(song.album_artist.to_owned())
            )),
            track_id: ActiveValue::Set(format!("{:x}", md5::compute(song.uri.as_ref().unwrap()))),
        }
    }
}
