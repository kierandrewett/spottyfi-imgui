use bitflags::bitflags;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;

bitflags! {
    pub struct SpotifyAPISearchType: u8 {
        const Album = 0x01;
        const Artist = 0x02;
        const Playlist = 0x03;
        const Track = 0x04;
    }
}

impl SpotifyAPISearchType {
    pub fn from(value: String) -> Self {
        let mut flags = SpotifyAPISearchType::empty();
        for part in value.split(',') {
            match part {
                "album" => flags |= SpotifyAPISearchType::Album,
                "artist" => flags |= SpotifyAPISearchType::Artist,
                "playlist" => flags |= SpotifyAPISearchType::Playlist,
                "track" => flags |= SpotifyAPISearchType::Track,
                _ => {},
            }
        }

        flags
    }
}

impl fmt::Display for SpotifyAPISearchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = vec![];

        if self.contains(SpotifyAPISearchType::Album) {
            flags.push("album");
        }
        if self.contains(SpotifyAPISearchType::Artist) {
            flags.push("artist");
        }
        if self.contains(SpotifyAPISearchType::Playlist) {
            flags.push("playlist");
        }
        if self.contains(SpotifyAPISearchType::Track) {
            flags.push("track");
        }

        write!(f, "{}", flags.join(","))
    }
}