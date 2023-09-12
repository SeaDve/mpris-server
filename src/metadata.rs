use std::{collections::HashMap, fmt};

use serde::Serialize;
use zbus::zvariant::{self, OwnedObjectPath, Type, Value};

use crate::TimeInUs;

/// Date/time fields should be sent as strings in ISO 8601 extended
/// format. If the timezone is known (eg: for xesam:lastPlayed), the
/// internet profile format of ISO 8601, as specified in RFC 3339,
///  should be used.
///
/// For example: "2007-04-29T13:56+01:00" for 29th April 2007, four
/// minutes to 2pm, in a time zone 1 hour ahead of UTC.
pub type DateTime = String;

/// URIs should be sent as (UTF-8) strings. Local files should use the
/// "file://" schema.
pub type Uri = String;

/// A mapping from metadata attribute names to values.
#[derive(Clone, PartialEq, Serialize, Type)]
#[zvariant(signature = "a{sv}")]
pub struct Metadata(HashMap<String, Value<'static>>);

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

impl Metadata {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn builder() -> MetadataBuilder {
        MetadataBuilder { m: Metadata::new() }
    }

    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<Value<'static>>,
    ) -> Option<Value<'static>> {
        self.0.insert(key.into(), value.into())
    }

    pub fn get<'v, V>(&'v self, key: &str) -> Option<zvariant::Result<&'v V>>
    where
        &'v V: TryFrom<&'v Value<'v>>,
    {
        self.get_value(key)
            .map(|v| v.downcast_ref().ok_or(zvariant::Error::IncorrectType))
    }

    pub fn get_value(&self, key: &str) -> Option<&Value<'_>> {
        self.0.get(key)
    }

    /// A unique identity for this track within the context of an
    /// MPRIS object (eg: tracklist).
    pub fn set_trackid(&mut self, trackid: impl Into<OwnedObjectPath>) {
        self.insert("mpris:trackid", trackid.into());
    }

    /// The duration of the track in microseconds.
    pub fn set_length(&mut self, length: TimeInUs) {
        self.insert("mpris:length", length);
    }

    /// The location of an image representing the track or album.
    /// Clients should not assume this will continue to exist when
    /// the media player stops giving out the URL.
    pub fn set_art_url(&mut self, art_url: impl Into<Uri>) {
        self.insert("mpris:artUrl", art_url.into());
    }

    /// The album name.
    pub fn set_album(&mut self, album: impl Into<String>) {
        self.insert("xesam:album", album.into());
    }

    /// The album artist(s).
    pub fn set_album_artist(&mut self, album_artist: impl IntoIterator<Item = impl Into<String>>) {
        self.insert(
            "xesam:albumArtist",
            album_artist
                .into_iter()
                .map(|i| i.into())
                .collect::<Vec<_>>(),
        );
    }

    /// The track artist(s).
    pub fn set_artist(&mut self, artist: impl IntoIterator<Item = impl Into<String>>) {
        self.insert(
            "xesam:artist",
            artist.into_iter().map(|i| i.into()).collect::<Vec<_>>(),
        );
    }

    /// The track lyrics.
    pub fn set_lyrics(&mut self, lyrics: impl Into<String>) {
        self.insert("xesam:asText", lyrics.into());
    }

    /// The speed of the music, in beats per minute.
    pub fn set_audio_bpm(&mut self, audio_bpm: i32) {
        self.insert("xesam:audioBPM", audio_bpm);
    }

    /// An automatically-generated rating, based on things such
    /// as how often it has been played. This should be in the
    /// range 0.0 to 1.0.
    pub fn set_auto_rating(&mut self, auto_rating: f64) {
        self.insert("xesam:autoRating", auto_rating);
    }

    /// A (list of) freeform comment(s).
    pub fn set_comment(&mut self, comment: impl IntoIterator<Item = impl Into<String>>) {
        self.insert(
            "xesam:comment",
            comment.into_iter().map(|i| i.into()).collect::<Vec<_>>(),
        );
    }

    /// The composer(s) of the track.
    pub fn set_composer(&mut self, composer: impl IntoIterator<Item = impl Into<String>>) {
        self.insert(
            "xesam:composer",
            composer.into_iter().map(|i| i.into()).collect::<Vec<_>>(),
        );
    }

    /// When the track was created. Usually only the year component
    /// will be useful.
    pub fn set_content_created(&mut self, content_created: impl Into<DateTime>) {
        self.insert("xesam:contentCreated", content_created.into());
    }

    /// The disc number on the album that this track is from.
    pub fn set_disc_number(&mut self, disc_number: i32) {
        self.insert("xesam:discNumber", disc_number);
    }

    /// When the track was first played.
    pub fn set_first_used(&mut self, first_used: impl Into<DateTime>) {
        self.insert("xesam:firstUsed", first_used.into());
    }

    /// The genre(s) of the track.
    pub fn set_genre(&mut self, genre: impl IntoIterator<Item = impl Into<String>>) {
        self.insert(
            "xesam:genre",
            genre.into_iter().map(|i| i.into()).collect::<Vec<_>>(),
        );
    }

    /// When the track was last played.
    pub fn set_last_used(&mut self, last_used: impl Into<DateTime>) {
        self.insert("xesam:lastUsed", last_used.into());
    }

    /// The lyricist(s) of the track.
    pub fn set_lyricist(&mut self, lyricist: impl IntoIterator<Item = impl Into<String>>) {
        self.insert(
            "xesam:lyricist",
            lyricist.into_iter().map(|i| i.into()).collect::<Vec<_>>(),
        );
    }

    /// The track title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.insert("xesam:title", title.into());
    }

    /// The track number on the album disc.
    pub fn set_track_number(&mut self, track_number: i32) {
        self.insert("xesam:trackNumber", track_number);
    }

    /// The location of the media file.
    pub fn set_url(&mut self, url: impl Into<Uri>) {
        self.insert("xesam:url", url.into());
    }

    /// The number of times the track has been played.
    pub fn set_use_count(&mut self, use_count: i32) {
        self.insert("xesam:useCount", use_count);
    }

    /// A user-specified rating. This should be in the range 0.0 to 1.0.
    pub fn set_user_rating(&mut self, user_rating: f64) {
        self.insert("xesam:userRating", user_rating);
    }
}

#[derive(Debug, Default, Clone)]
pub struct MetadataBuilder {
    m: Metadata,
}

impl MetadataBuilder {
    pub fn trackid(mut self, trackid: impl Into<OwnedObjectPath>) -> Self {
        self.m.set_trackid(trackid);
        self
    }

    pub fn length(mut self, length: TimeInUs) -> Self {
        self.m.set_length(length);
        self
    }

    pub fn art_url(mut self, art_url: impl Into<Uri>) -> Self {
        self.m.set_art_url(art_url);
        self
    }

    pub fn album(mut self, album: impl Into<String>) -> Self {
        self.m.set_album(album);
        self
    }

    pub fn album_artist(
        mut self,
        album_artist: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.m.set_album_artist(album_artist);
        self
    }

    pub fn artist(mut self, artist: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_artist(artist);
        self
    }

    pub fn lyrics(mut self, lyrics: impl Into<String>) -> Self {
        self.m.set_lyrics(lyrics);
        self
    }

    pub fn audio_bpm(mut self, audio_bpm: i32) -> Self {
        self.m.set_audio_bpm(audio_bpm);
        self
    }

    pub fn auto_rating(mut self, auto_rating: f64) -> Self {
        self.m.set_auto_rating(auto_rating);
        self
    }

    pub fn comment(mut self, comment: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_comment(comment);
        self
    }

    pub fn composer(mut self, composer: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_composer(composer);
        self
    }

    pub fn content_created(mut self, content_created: impl Into<DateTime>) -> Self {
        self.m.set_content_created(content_created);
        self
    }

    pub fn disc_number(mut self, disc_number: i32) -> Self {
        self.m.set_disc_number(disc_number);
        self
    }

    pub fn first_used(mut self, first_used: impl Into<DateTime>) -> Self {
        self.m.set_first_used(first_used);
        self
    }

    pub fn genre(mut self, genre: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_genre(genre);
        self
    }

    pub fn last_used(mut self, last_used: impl Into<DateTime>) -> Self {
        self.m.set_last_used(last_used);
        self
    }

    pub fn lyricist(mut self, lyricist: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_lyricist(lyricist);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.m.set_title(title.into());
        self
    }

    pub fn track_number(mut self, track_number: i32) -> Self {
        self.m.set_track_number(track_number);
        self
    }

    pub fn url(mut self, url: impl Into<Uri>) -> Self {
        self.m.set_url(url.into());
        self
    }

    pub fn use_count(mut self, use_count: i32) -> Self {
        self.m.set_use_count(use_count);
        self
    }

    pub fn user_rating(mut self, user_rating: f64) -> Self {
        self.m.set_user_rating(user_rating);
        self
    }

    pub fn build(self) -> Metadata {
        self.m
    }
}

impl<'a> From<Metadata> for Value<'a> {
    fn from(metainfo: Metadata) -> Self {
        Value::new(metainfo.0)
    }
}
