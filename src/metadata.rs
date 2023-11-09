use std::{collections::HashMap, fmt};

use serde::Serialize;
use zbus::zvariant::{self, Type, Value};

use crate::{Time, TrackId, Uri};

/// Combined date and time.
///
/// This should be sent as strings in ISO 8601 extended
/// format (eg: 2007-04-29T14:35:51). If the timezone is known (eg: for
/// xesam:lastPlayed), the internet profile format of ISO 8601, as specified in
/// RFC 3339, should be used (eg: 2007-04-29T14:35:51+02:00).
///
/// For example: "2007-04-29T13:56+01:00" for 29th April 2007, four
/// minutes to 2pm, in a time zone 1 hour ahead of UTC.
pub type DateTime = String;

/// A mapping from metadata attribute names to values.
///
/// The [`mpris:trackid`] attribute must always be present.
///
/// If the length of the track is known, it should be provided in the metadata
/// property with the [`mpris:length`] key.
///
/// If there is an image associated with the track, a URL for it may be provided
/// using the [`mpris:artUrl`] key.
///
/// [`mpris:trackid`]: Metadata::set_trackid
/// [`mpris:length`]: Metadata::set_length
/// [`mpris:artUrl`]: Metadata::set_art_url
#[derive(Clone, PartialEq, Serialize, Type)]
#[serde(transparent)]
#[zvariant(signature = "a{sv}")]
#[doc(alias = "Metadata_Map")]
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
    /// Creates an empty [`Metadata`].
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates a new builder-pattern struct instance to construct [`Metadata`].
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder { m: Metadata::new() }
    }

    /// Returns the value corresponding to the key and convert it to `V`.
    pub fn get<'v, V>(&'v self, key: &str) -> Option<zvariant::Result<&'v V>>
    where
        &'v V: TryFrom<&'v Value<'v>>,
    {
        self.get_value(key)
            .map(|v| v.downcast_ref().ok_or(zvariant::Error::IncorrectType))
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get_value(&self, key: &str) -> Option<&Value<'_>> {
        self.0.get(key)
    }

    /// Sets the value for the given key, overwriting any existing value.
    ///
    /// If the given value is `None`, the key is removed from the metadata,
    /// if any.
    pub fn set(&mut self, key: &str, value: Option<impl Into<Value<'static>>>) {
        self.set_value(key, value.map(|value| value.into()))
    }

    /// Like [`Metadata::set`], but takes a [`Value`] instead of a generic type.
    pub fn set_value(&mut self, key: &str, value: Option<Value<'static>>) {
        if let Some(value) = value {
            self.0.insert(key.into(), value);
        } else {
            self.0.remove(key);
        }
    }

    /// A unique identity for this track within the context of an
    /// MPRIS object (eg: tracklist).
    ///
    /// This contains a D-Bus path that uniquely identifies the track
    /// within the scope of the playlist. There may or may not be an actual
    /// D-Bus object at that path; this specification says nothing about
    /// what interfaces such an object may implement.
    pub fn trackid(&self) -> Option<TrackId> {
        self.get_value("mpris:trackid")?.clone().downcast()
    }

    /// Sets a unique identity for this track within the context of an
    /// MPRIS object (eg: tracklist).
    ///
    /// This contains a D-Bus path that uniquely identifies the track
    /// within the scope of the playlist. There may or may not be an actual
    /// D-Bus object at that path; this specification says nothing about
    /// what interfaces such an object may implement.
    pub fn set_trackid(&mut self, trackid: Option<impl Into<TrackId>>) {
        self.set("mpris:trackid", trackid.map(|trackid| trackid.into()))
    }

    /// The duration of the track.
    pub fn length(&self) -> Option<Time> {
        self.get_value("mpris:length")?.clone().downcast()
    }

    /// Sets the duration of the track.
    pub fn set_length(&mut self, length: Option<Time>) {
        self.set("mpris:length", length)
    }

    /// The location of an image representing the track or album.
    ///
    /// Clients should not assume this will continue to exist when
    /// the media player stops giving out the URL.
    pub fn art_url(&self) -> Option<Uri> {
        self.get_value("mpris:artUrl")?.clone().downcast()
    }

    /// Sets the location of an image representing the track or album.
    ///
    /// Clients should not assume this will continue to exist when
    /// the media player stops giving out the URL.
    pub fn set_art_url(&mut self, art_url: Option<impl Into<Uri>>) {
        self.set("mpris:artUrl", art_url.map(|art_url| art_url.into()));
    }

    /// The album name.
    pub fn album(&self) -> Option<&str> {
        self.get_value("xesam:album")?.downcast_ref()
    }

    /// Sets the album name.
    pub fn set_album(&mut self, album: Option<impl Into<String>>) {
        self.set("xesam:album", album.map(|album| album.into()));
    }

    /// The album artist(s).
    pub fn album_artist(&self) -> Option<Vec<String>> {
        self.get_value("xesam:albumArtist")?.clone().downcast()
    }

    /// Sets the album artist(s).
    pub fn set_album_artist(
        &mut self,
        album_artist: Option<impl IntoIterator<Item = impl Into<String>>>,
    ) {
        self.set(
            "xesam:albumArtist",
            album_artist.map(|album_artist| {
                album_artist
                    .into_iter()
                    .map(|i| i.into())
                    .collect::<Vec<_>>()
            }),
        );
    }

    /// The track artist(s).
    pub fn artist(&self) -> Option<Vec<String>> {
        self.get_value("xesam:artist")?.clone().downcast()
    }

    /// Sets the track artist(s).
    pub fn set_artist(&mut self, artist: Option<impl IntoIterator<Item = impl Into<String>>>) {
        self.set(
            "xesam:artist",
            artist.map(|artist| artist.into_iter().map(|i| i.into()).collect::<Vec<_>>()),
        );
    }

    /// The track lyrics.
    pub fn lyrics(&self) -> Option<&str> {
        self.get_value("xesam:asText")?.downcast_ref()
    }

    /// Sets the track lyrics.
    pub fn set_lyrics(&mut self, lyrics: Option<impl Into<String>>) {
        self.set("xesam:asText", lyrics.map(|lyrics| lyrics.into()));
    }

    /// The speed of the music, in beats per minute.
    pub fn audio_bpm(&self) -> Option<i32> {
        self.get_value("xesam:audioBPM")?.downcast_ref().copied()
    }

    /// Sets the speed of the music, in beats per minute.
    pub fn set_audio_bpm(&mut self, audio_bpm: Option<i32>) {
        self.set("xesam:audioBPM", audio_bpm);
    }

    /// An automatically-generated rating, based on things such
    /// as how often it has been played. This should be in the
    /// range 0.0 to 1.0.
    pub fn auto_rating(&self) -> Option<f64> {
        self.get_value("xesam:autoRating")?.downcast_ref().copied()
    }

    /// Sets an automatically-generated rating, based on things such
    /// as how often it has been played. This should be in the
    /// range 0.0 to 1.0.
    pub fn set_auto_rating(&mut self, auto_rating: Option<f64>) {
        self.set("xesam:autoRating", auto_rating);
    }

    /// A (list of) freeform comment(s).
    pub fn comment(&self) -> Option<Vec<String>> {
        self.get_value("xesam:comment")?.clone().downcast()
    }

    /// Sets a (list of) freeform comment(s).
    pub fn set_comment(&mut self, comment: Option<impl IntoIterator<Item = impl Into<String>>>) {
        self.set(
            "xesam:comment",
            comment.map(|comment| comment.into_iter().map(|i| i.into()).collect::<Vec<_>>()),
        );
    }

    /// The composer(s) of the track.
    pub fn composer(&self) -> Option<Vec<String>> {
        self.get_value("xesam:composer")?.clone().downcast()
    }

    /// Sets the composer(s) of the track.
    pub fn set_composer(&mut self, composer: Option<impl IntoIterator<Item = impl Into<String>>>) {
        self.set(
            "xesam:composer",
            composer.map(|composer| composer.into_iter().map(|i| i.into()).collect::<Vec<_>>()),
        );
    }

    /// When the track was created. Usually only the year component
    /// will be useful.
    pub fn content_created(&self) -> Option<DateTime> {
        self.get_value("xesam:contentCreated")?.clone().downcast()
    }

    /// Sets when the track was created. Usually only the year component
    /// will be useful.
    pub fn set_content_created(&mut self, content_created: Option<impl Into<DateTime>>) {
        self.set(
            "xesam:contentCreated",
            content_created.map(|content_created| content_created.into()),
        );
    }

    /// The disc number on the album that this track is from.
    pub fn disc_number(&self) -> Option<i32> {
        self.get_value("xesam:discNumber")?.downcast_ref().copied()
    }

    /// Sets the disc number on the album that this track is from.
    pub fn set_disc_number(&mut self, disc_number: Option<i32>) {
        self.set("xesam:discNumber", disc_number);
    }

    /// When the track was first played.
    pub fn first_used(&self) -> Option<DateTime> {
        self.get_value("xesam:firstUsed")?.clone().downcast()
    }

    /// Sets when the track was first played.
    pub fn set_first_used(&mut self, first_used: Option<impl Into<DateTime>>) {
        self.set(
            "xesam:firstUsed",
            first_used.map(|first_used| first_used.into()),
        );
    }

    /// The genre(s) of the track.
    pub fn genre(&self) -> Option<Vec<String>> {
        self.get_value("xesam:genre")?.clone().downcast()
    }

    /// Sets the genre(s) of the track.
    pub fn set_genre(&mut self, genre: Option<impl IntoIterator<Item = impl Into<String>>>) {
        self.set(
            "xesam:genre",
            genre.map(|genre| genre.into_iter().map(|i| i.into()).collect::<Vec<_>>()),
        );
    }

    /// When the track was last played.
    pub fn last_used(&self) -> Option<DateTime> {
        self.get_value("xesam:lastUsed")?.clone().downcast()
    }

    /// Sets when the track was last played.
    pub fn set_last_used(&mut self, last_used: Option<impl Into<DateTime>>) {
        self.set(
            "xesam:lastUsed",
            last_used.map(|last_used| last_used.into()),
        );
    }

    /// The lyricist(s) of the track.
    pub fn lyricist(&self) -> Option<Vec<String>> {
        self.get_value("xesam:lyricist")?.clone().downcast()
    }

    /// Sets the lyricist(s) of the track.
    pub fn set_lyricist(&mut self, lyricist: Option<impl IntoIterator<Item = impl Into<String>>>) {
        self.set(
            "xesam:lyricist",
            lyricist.map(|lyricist| lyricist.into_iter().map(|i| i.into()).collect::<Vec<_>>()),
        );
    }

    /// The track title.
    pub fn title(&self) -> Option<&str> {
        self.get_value("xesam:title")?.downcast_ref()
    }

    /// Sets the track title.
    pub fn set_title(&mut self, title: Option<impl Into<String>>) {
        self.set("xesam:title", title.map(|title| title.into()));
    }

    /// The track number on the album disc.
    pub fn track_number(&self) -> Option<i32> {
        self.get_value("xesam:trackNumber")?.downcast_ref().copied()
    }

    /// Sets the track number on the album disc.
    pub fn set_track_number(&mut self, track_number: Option<i32>) {
        self.set("xesam:trackNumber", track_number);
    }

    /// The location of the media file.
    pub fn url(&self) -> Option<Uri> {
        self.get_value("xesam:url")?.clone().downcast()
    }

    /// Sets the location of the media file.
    pub fn set_url(&mut self, url: Option<impl Into<Uri>>) {
        self.set("xesam:url", url.map(|url| url.into()));
    }

    /// The number of times the track has been played.
    pub fn use_count(&self) -> Option<i32> {
        self.get_value("xesam:useCount")?.downcast_ref().copied()
    }

    /// Sets the number of times the track has been played.
    pub fn set_use_count(&mut self, use_count: Option<i32>) {
        self.set("xesam:useCount", use_count);
    }

    /// A user-specified rating. This should be in the range 0.0 to 1.0.
    pub fn user_rating(&self) -> Option<f64> {
        self.get_value("xesam:userRating")?.downcast_ref().copied()
    }

    /// Sets a user-specified rating. This should be in the range 0.0 to 1.0.
    pub fn set_user_rating(&mut self, user_rating: Option<f64>) {
        self.set("xesam:userRating", user_rating);
    }
}

/// A builder used to create [`Metadata`].
#[derive(Debug, Default, Clone)]
#[must_use = "must call `build()` to finish building the metadata"]
pub struct MetadataBuilder {
    m: Metadata,
}

impl MetadataBuilder {
    /// Sets a value for the given key.
    pub fn other(mut self, key: &str, value: impl Into<Value<'static>>) -> Self {
        self.m.set(key, Some(value));
        self
    }

    /// Sets a unique identity for this track within the context of an
    /// MPRIS object (eg: tracklist).
    ///
    /// This contains a D-Bus path that uniquely identifies the track
    /// within the scope of the playlist. There may or may not be an actual
    /// D-Bus object at that path; this specification says nothing about
    /// what interfaces such an object may implement.
    pub fn trackid(mut self, trackid: impl Into<TrackId>) -> Self {
        self.m.set_trackid(Some(trackid));
        self
    }

    /// Sets the duration of the track.
    pub fn length(mut self, length: Time) -> Self {
        self.m.set_length(Some(length));
        self
    }

    /// Sets the location of an image representing the track or album.
    ///
    /// Clients should not assume this will continue to exist when
    /// the media player stops giving out the URL.
    pub fn art_url(mut self, art_url: impl Into<Uri>) -> Self {
        self.m.set_art_url(Some(art_url));
        self
    }

    /// Sets the album name.
    pub fn album(mut self, album: impl Into<String>) -> Self {
        self.m.set_album(Some(album));
        self
    }

    /// Sets the album artist(s).
    pub fn album_artist(
        mut self,
        album_artist: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.m.set_album_artist(Some(album_artist));
        self
    }

    /// Sets the track artist(s).
    pub fn artist(mut self, artist: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_artist(Some(artist));
        self
    }

    /// Sets the track lyrics.
    pub fn lyrics(mut self, lyrics: impl Into<String>) -> Self {
        self.m.set_lyrics(Some(lyrics));
        self
    }

    /// Sets the speed of the music, in beats per minute.
    pub fn audio_bpm(mut self, audio_bpm: i32) -> Self {
        self.m.set_audio_bpm(Some(audio_bpm));
        self
    }

    /// Sets an automatically-generated rating, based on things such
    /// as how often it has been played. This should be in the
    /// range 0.0 to 1.0.
    pub fn auto_rating(mut self, auto_rating: f64) -> Self {
        self.m.set_auto_rating(Some(auto_rating));
        self
    }

    /// Sets a (list of) freeform comment(s).
    pub fn comment(mut self, comment: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_comment(Some(comment));
        self
    }

    /// Sets the composer(s) of the track.
    pub fn composer(mut self, composer: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_composer(Some(composer));
        self
    }

    /// Sets when the track was created. Usually only the year component
    /// will be useful.
    pub fn content_created(mut self, content_created: impl Into<DateTime>) -> Self {
        self.m.set_content_created(Some(content_created));
        self
    }

    /// Sets the disc number on the album that this track is from.
    pub fn disc_number(mut self, disc_number: i32) -> Self {
        self.m.set_disc_number(Some(disc_number));
        self
    }

    /// Sets when the track was first played.
    pub fn first_used(mut self, first_used: impl Into<DateTime>) -> Self {
        self.m.set_first_used(Some(first_used));
        self
    }

    /// Sets the genre(s) of the track.
    pub fn genre(mut self, genre: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_genre(Some(genre));
        self
    }

    /// Sets when the track was last played.
    pub fn last_used(mut self, last_used: impl Into<DateTime>) -> Self {
        self.m.set_last_used(Some(last_used));
        self
    }

    /// Sets the lyricist(s) of the track.
    pub fn lyricist(mut self, lyricist: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.m.set_lyricist(Some(lyricist));
        self
    }

    /// Sets the track title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.m.set_title(Some(title));
        self
    }

    /// Sets the track number on the album disc.
    pub fn track_number(mut self, track_number: i32) -> Self {
        self.m.set_track_number(Some(track_number));
        self
    }

    /// Sets the location of the media file.
    pub fn url(mut self, url: impl Into<Uri>) -> Self {
        self.m.set_url(Some(url));
        self
    }

    /// Sets the number of times the track has been played.
    pub fn use_count(mut self, use_count: i32) -> Self {
        self.m.set_use_count(Some(use_count));
        self
    }

    /// Sets a user-specified rating. This should be in the range 0.0 to 1.0.
    pub fn user_rating(mut self, user_rating: f64) -> Self {
        self.m.set_user_rating(Some(user_rating));
        self
    }

    /// Creates [`Metadata`] from the builder.
    #[must_use = "building metadata is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Metadata {
        self.m
    }
}

impl<'a> From<Metadata> for Value<'a> {
    fn from(metainfo: Metadata) -> Self {
        Value::new(metainfo.0)
    }
}
