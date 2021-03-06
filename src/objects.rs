use serde::{Deserialize, Serialize};

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-albumobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlbumObject {
    /// The type of the album: `album`, `single`, or `compilation`.
    pub album_type: String,
    /// The artists of the album. Each artist object includes a link in `href` to more detailed information about the artist.
    pub artists: Vec<ArtistObject>,
    /// The markets in which the album is available: [ISO 3166-1 alpha-2 country codes.](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) Note that an album is considered available in a market when at least 1 of its tracks is available in that market.
    pub available_markets: Vec<String>,
    /// The copyright statements of the album.
    pub copyrights: Vec<CopyrightObject>,
    /// Known external IDs for the album.
    pub external_ids: ExternalIdObject,
    /// Known external URLs for this album.
    pub external_urls: ExternalUrlObject,
    /// A list of the genres used to classify the album. For example: “Prog Rock” , “Post-Grunge”. (If not yet classified, the array is empty.)
    pub genres: Vec<String>,
    /// A link to the Web API endpoint providing full details of the album.
    pub href: String,
    /// The Spotify ID for the album.
    pub id: String,
    /// The cover art for the album in various sizes, widest first.
    pub images: Vec<ImageObject>,
    /// The label for the album.
    pub label: String,
    /// The name of the album. In case of an album takedown, the value may be an empty string.
    pub name: String,
    /// The popularity of the album. The value will be between 0 and 100, with 100 being the most popular. The popularity is calculated from the popularity of the album’s individual tracks.
    pub popularity: usize,
    /// The date the album was first released, for example “1981-12-15”. Depending on the precision, it might be shown as “1981” or “1981-12”.
    pub release_date: String,
    /// The precision with which release_date value is known: “year” , “month” , or “day”.
    pub release_date_precision: String,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject) for more details.
    pub restrictions: AlbumRestrictionObject,
    /// The tracks of the album.
    pub tracks: Vec<SimplifiedTrackObject>,
    /// The object type: “album"
    #[serde(rename = "type")]
    pub _type: String,
    /// The Spotify URI for the album.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlbumRestrictionObject {
    /// The reason for the restriction. Supported values:
    /// * `market` - The content item is not available in the given market.
    /// * `product` - The content item is not available for the user’s subscription type.
    /// * `explicit` - The content item is explicit and the user’s account is set to not play explicit content. Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
    pub reason: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-artistobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtistObject {
    /// Known external URLs for this artist.
    pub external_urls: ExternalUrlObject,
    /// Information about the followers of the artist.
    pub followers: Option<FollowersObject>,
    /// A list of the genres the artist is associated with. For example: `"Prog Rock"` , `"Post-Grunge"`. (If not yet classified, the array is empty.)
    pub genres: Option<Vec<String>>,
    /// A link to the Web API endpoint providing full details of the artist.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub id: String,
    /// Images of the artist in various sizes, widest first.
    pub images: Vec<ImageObject>,
    /// The name of the artist.
    pub name: String,
    /// The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular. The artist’s popularity is calculated from the popularity of all the artist’s tracks.
    pub popularity: usize,
    /// The object type: `"artist"`
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub uri: String,
}

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-audiofeaturesobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AudioFeaturesObject {
    /// A confidence measure from 0.0 to 1.0 of whether the track is acoustic. 1.0 represents high confidence the track is acoustic.
    pub acousticness: usize,
    /// An HTTP URL to access the full audio analysis of this track. An access token is required to access this data.
    pub analysis_url: String,
    /// Danceability describes how suitable a track is for dancing based on a combination of musical elements including tempo, rhythm stability, beat strength, and overall regularity. A value of 0.0 is least danceable and 1.0 is most danceable.
    pub danceability: usize,
    /// The duration of the track in milliseconds.
    pub duration_ms: usize,
    /// Energy is a measure from 0.0 to 1.0 and represents a perceptual measure of intensity and activity. Typically, energetic tracks feel fast, loud, and noisy. For example, death metal has high energy, while a Bach prelude scores low on the scale. Perceptual features contributing to this attribute include dynamic range, perceived loudness, timbre, onset rate, and general entropy.
    pub energy: usize,
    /// The Spotify ID for the track.
    pub id: String,
    /// Predicts whether a track contains no vocals. “Ooh” and “aah” sounds are treated as instrumental in this context. Rap or spoken word tracks are clearly “vocal”. The closer the instrumentalness value is to 1.0, the greater likelihood the track contains no vocal content. Values above 0.5 are intended to represent instrumental tracks, but confidence is higher as the value approaches 1.0.
    pub instrumentalness: usize,
    /// The key the track is in. Integers map to pitches using standard [Pitch Class notation](https://en.wikipedia.org/wiki/Pitch_class). E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on.
    pub key: usize,
    /// Detects the presence of an audience in the recording. Higher liveness values represent an increased probability that the track was performed live. A value above 0.8 provides strong likelihood that the track is live.
    pub liveness: usize,
    /// The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typical range between -60 and 0 db.
    pub loudness: usize,
    /// Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0.
    pub mode: usize,
    /// Speechiness detects the presence of spoken words in a track. The more exclusively speech-like the recording (e.g. talk show, audio book, poetry), the closer to 1.0 the attribute value. Values above 0.66 describe tracks that are probably made entirely of spoken words. Values between 0.33 and 0.66 describe tracks that may contain both music and speech, either in sections or layered, including such cases as rap music. Values below 0.33 most likely represent music and other non-speech-like tracks.
    pub speechiness: usize,
    /// The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration.
    pub tempo: usize,
    /// An estimated overall time signature of a track. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure).
    pub time_signature: usize,
    /// A link to the Web API endpoint providing full details of the track.
    pub track_href: String,
    /// The object type: “audio_features”
    #[serde(rename = "type")]
    pub _type: String,
    /// The Spotify URI for the track.
    pub uri: String,
    /// A measure from 0.0 to 1.0 describing the musical positiveness conveyed by a track. Tracks with high valence sound more positive (e.g. happy, cheerful, euphoric), while tracks with low valence sound more negative (e.g. sad, depressed, angry).
    pub valence: usize,
}

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-categoryobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CategoryObject {
    /// A link to the Web API endpoint returning full details of the category.
    pub href: String,
    /// The category icon, in various sizes.
    pub icons: Vec<ImageObject>,
    /// The [Spotify category ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) of the category.
    pub id: String,
    /// The name of the category.
    pub name: String,
}

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-contextobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextObject {
    /// External URLs for this context.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the track.
    pub href: String,
    /// The object type, e.g. “artist”, “playlist”, “album”, “show”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the context.
    pub uri: String,
}

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-copyrightobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CopyrightObject {
    /// The copyright text for this content.
    pub text: String,
    /// The type of copyright: `C` = the copyright, `P` = the sound recording (performance) copyright.
    #[serde(rename = "type")]
    pub _type: String,
}

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-currentlyplayingcontextobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentlyPlayingContextObject {
    /// Allows to update the user interface based on which playback actions are available within the current context.
    pub actions: DisallowsObject,
    /// A Context Object. Can be `null`.
    pub context: ContextObject,
    /// The object type of the currently playing item. Can be one of `track`, `episode`, `ad` or `unknown`.
    pub currently_playing_type: String,
    /// The device that is currently active.
    pub device: DeviceObject,
    /// If something is currently playing, return `true`.
    pub is_playing: bool,
    /// The currently playing track or episode. Can be `null`.
    pub item: PlaylistItemType<TrackObject, EpisodeObject>,
    /// Progress into the currently playing track or episode. Can be `null`.
    pub progress_ms: usize,
    /// off, track, context
    pub repeat_state: String,
    /// If shuffle is on or off.
    pub shuffle_state: String,
    /// Unix Millisecond Timestamp when data was fetched.
    pub timestamp: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-currentlyplayingobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentlyPlayingObject {
    /// A Context Object. Can be `null`.
    pub context: ContextObject,
    /// The object type of the currently playing item. Can be one of `track`, `episode`, `ad` or `unknown`.
    pub currently_playing_type: String,
    /// If something is currently playing, return `true`.
    pub is_playing: bool,
    /// The currently playing track or episode. Can be `null`.
    pub item: PlaylistItemType<TrackObject, EpisodeObject>,
    /// Progress into the currently playing track or episode. Can be `null`.
    pub progress_ms: usize,
    /// If shuffle is on or off.
    pub shuffle_state: String,
    /// Unix Millisecond Timestamp when data was fetched.
    pub timestamp: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-cursorobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CursorObject {
    /// The cursor to use as key to find the next page of items.
    pub after: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-cursorpagingobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CursorPagingObject {
    /// The cursors used to find the next set of items.
    pub cursors: CursorObject,
    /// A link to the Web API endpoint returning the full result of the request.
    pub href: String,
    /// The requested data.
    //items: Vec<Object>
    /// The maximum number of items in the response (as set in the query or by default)
    pub limit: usize,
    /// URL to the next page of items. (`null` if none)
    pub next: String,
    /// The total number of items available to return.
    pub total: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-deviceobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceObject {
    /// The device ID. This may be `null`.
    pub id: String,
    /// If this device is the currently active device.
    pub is_active: bool,
    /// If this device is currently in a private session.
    pub is_private_session: bool,
    /// Whether controlling this device is restricted. At present if this is “true” then no Web API commands will be accepted by this device.
    pub is_restricted: bool,
    /// The name of the device.
    pub name: String,
    /// Device type, such as “computer”, “smartphone” or “speaker”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The current volume in percent. This may be null.
    pub volume_percent: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-devicesobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DevicesObject {
    /// A list of 0..n Device objects
    pub devices: Vec<DeviceObject>,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-disallowsobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DisallowsObject {
    /// Interrupting playback. Optional field.
    pub interrupting_playback: Option<bool>,
    /// Pausing. Optional field.
    pub pausing: Option<bool>,
    /// Resuming. Optional field.
    pub resuming: Option<bool>,
    /// Seeking playback location. Optional field.
    pub seeking: Option<bool>,
    /// Skipping to the next context. Optional field.
    pub skipping_next: Option<bool>,
    /// Skipping to the previous context. Optional field.
    pub skipping_prev: Option<bool>,
    /// Toggling repeat context flag. Optional field.
    pub toggling_repeat_context: Option<bool>,
    /// Toggling repeat track flag. Optional field.
    pub toggling_repeat_track: Option<bool>,
    /// Toggling shuffle flag. Optional field.
    pub toggling_shuffle: Option<bool>,
    /// Transfering playback between devices. Optional field.
    pub transferring_playback: Option<bool>,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-episodeobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpisodeObject {
    /// A URL to a 30 second preview (MP3 format) of the episode. `null` if not available.
    pub audio_preview_url: String,
    /// A description of the episode. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
    pub description: String,
    /// The episode length in milliseconds.
    pub duration_ms: usize,
    /// Whether or not the episode has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,
    /// External URLs for this episode.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the episode.
    pub href: String,
    /// A description of the episode. This field may contain HTML tags.
    pub html_description: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the episode.
    pub id: String,
    /// The cover art for the episode in various sizes, widest first.
    pub images: Vec<ImageObject>,
    /// True if the episode is hosted outside of Spotify’s CDN.
    pub is_externally_hosted: bool,
    /// True if the episode is playable in the given market. Otherwise false.
    pub is_playable: bool,
    /// **Note: This field is deprecated and might be removed in the future. Please use the languages field instead.** The language used in the episode, identified by a [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub language: String,
    /// A list of the languages used in the episode, identified by their [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,
    /// The name of the episode.
    pub name: String,
    /// The date the episode was first released, for example `"1981-12-15"`. Depending on the precision, it might be shown as `"1981"` or `"1981-12"`.
    pub release_date: String,
    /// The precision with which `release_date` value is known: `"year"`, `"month"`, or `"day"`.
    pub release_date_precision: String,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-episoderestrictionobject) for more details.
    pub restrictions: EpisodeRestrictionObject,
    /// The user’s most recent position in the episode. Set if the supplied access token is a user token and has the scope `user-read-playback-position`.
    pub resume_point: ResumePointObject,
    /// The show on which the episode belongs.
    pub show: SimplifiedShowObject,
    /// The object type: “episode”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the episode.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-episoderestrictionobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EpisodeRestrictionObject {
    /// The reason for the restriction. Supported values:
    /// * `market` - The content item is not available in the given market.
    /// * `product` - The content item is not available for the user’s subscription type.
    /// * `explicit` - The content item is explicit and the user’s account is set to not play explicit content. Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorJSON {
    pub error: ErrorObject,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-errorobject)
pub struct ErrorObject {
    /// A short description of the cause of the error.
    pub message: String,
    /// The HTTP status code (also returned in the response header; see [Response Status Codes](https://developer.spotify.com/documentation/web-api/#response-status-codes) for more information).
    pub status: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-explicitcontentsettingsobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitContentSettingsObject {
    /// When `true`, indicates that explicit content should not be played.
    pub filter_enabled: bool,
    /// When `true`, indicates that the explicit content setting is locked and can’t be changed by the user.
    pub filter_locked: bool,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-externalidobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalIdObject {
    /// [International Article Number](https://en.wikipedia.org/wiki/International_Article_Number)
    pub ean: String,
    /// [International Standard Recording Code](https://en.wikipedia.org/wiki/International_Standard_Recording_Code)
    pub isrc: String,
    /// [Universal Product Code](https://en.wikipedia.org/wiki/Universal_Product_Code)
    pub upc: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-externalurlobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalUrlObject {
    /// The [Spotify URL](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the object.
    pub spotify: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-followersobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FollowersObject {
    /// A link to the Web API endpoint providing full details of the followers; `null` if not available. Please note that this will always be set to null, as the Web API does not support it at the moment.
    pub href: String,
    /// The total number of followers.
    pub total: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-imageobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageObject {
    /// The image height in pixels. If unknown: `null` or not returned.
    pub height: Option<u32>,
    /// The source URL of the image.
    pub url: String,
    /// The image width in pixels. If unknown: `null` or not returned.
    pub width: Option<u32>,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-linkedtrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkedTrackObject {
    /// Known external URLs for this track.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the track.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub id: String,
    /// The object type: “track”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-pagingobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PagingObject {
    /// A link to the Web API endpoint returning the full result of the request
    pub href: String,
    /// The requested data.
    //items: Vec<Object>,
    /// The maximum number of items in the response (as set in the query or by default).
    pub limit: usize,
    /// URL to the next page of items. (`null` if none)
    pub next: String,
    /// The offset of the items returned (as set in the query or by default)
    pub offset: usize,
    /// URL to the previous page of items. (`null` if none)
    pub previous: String,
    /// The total number of items available to return.
    pub total: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-playhistoryobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayHistoryObject {
    /// The context the track was played from.
    pub context: ContextObject,
    /// The date and time the track was played.
    //played_at: Timestamp,
    /// The track the user listened to.
    pub track: SimplifiedTrackObject,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-playererrorobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerErrorObject {
    pub message: String,
    /// * `NO_PREV_TRACK` - The command requires a previous track, but there is none in the context.
    /// * `NO_NEXT_TRACK` - The command requires a next track, but there is none in the context.
    /// * `NO_SPECIFIC_TRACK` - The requested track does not exist.
    /// * `ALREADY_PAUSED` - The command requires playback to not be paused.
    /// * `NOT_PAUSED` - The command requires playback to be paused.
    /// * `NOT_PLAYING_LOCALLY` - The command requires playback on the local device.
    /// * `NOT_PLAYING_TRACK` - The command requires that a track is currently playing.
    /// * `NOT_PLAYING_CONTEXT` - The command requires that a context is currently playing.
    /// * `ENDLESS_CONTEXT` - The shuffle command cannot be applied on an endless context.
    /// * `CONTEXT_DISALLOW` - The command could not be performed on the context.
    /// * `ALREADY_PLAYING` - The track should not be restarted if the same track and context is already playing, and there is a resume point.
    /// * `RATE_LIMITED` - The user is rate limited due to too frequent track play, also known as cat-on-the-keyboard spamming.
    /// * `REMOTE_CONTROL_DISALLOW` - The context cannot be remote-controlled.
    /// * `DEVICE_NOT_CONTROLLABLE` - Not possible to remote control the device.
    /// * `VOLUME_CONTROL_DISALLOW` - Not possible to remote control the device’s volume.
    /// * `NO_ACTIVE_DEVICE` - Requires an active device and the user has none.
    /// * `PREMIUM_REQUIRED` - The request is prohibited for non-premium users.
    /// * `UNKNOWN` - Certain actions are restricted because of unknown reasons.
    pub reason: String,
    /// The HTTP status code. Either `404 NOT FOUND` or `403 FORBIDDEN`. Also returned in the response header.
    pub status: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-playlistobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistObject {
    /// `true` if the owner allows other users to modify the playlist.
    pub collaborative: bool,
    /// The playlist description. Only returned for modified, verified playlists, otherwise `null`.
    pub description: String,
    /// Known external URLs for this playlist.
    pub external_urls: ExternalUrlObject,
    /// Information about the followers of the playlist.
    pub followers: FollowersObject,
    /// A link to the Web API endpoint providing full details of the playlist.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the playlist.
    pub id: String,
    /// Images for the playlist. The array may be empty or contain up to three images. The images are returned by size in descending order. See [Working with Playlists](https://developer.spotify.com/documentation/general/guides/working-with-playlists/). _Note: If returned, the source URL for the image (`url`) is temporary and will expire in less than a day._
    pub images: Vec<ImageObject>,
    /// The name of the playlist.
    pub name: String,
    /// The user who owns the playlist
    pub owner: PublicUserObject,
    /// The playlist’s public/private status: `true` the playlist is public, `false` the playlist is private, `null` the playlist status is not relevant. For more about public/private status, see [Working with Playlists](https://developer.spotify.com/documentation/general/guides/working-with-playlists/)
    pub public: bool,
    /// The version identifier for the current playlist. Can be supplied in other requests to target a specific playlist version
    pub snapshot_id: String,
    /// Information about the tracks of the playlist. Note, a track object may be `null`. This can happen if a track is no longer available.
    pub tracks: Vec<PlaylistTrackObject>,
    /// The object type: “playlist”
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the playlist.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-playlisttrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistTrackObject {
    /// The date and time the track or episode was added. Note that some very old playlists may return `null` in this field.
    //added_at: Timestamp,
    /// The Spotify user who added the track or episode. Note that some very old playlists may return `null` in this field.
    pub added_by: PublicUserObject,
    /// Whether this track or episode is a [local file](https://developer.spotify.com/documentation/general/guides/local-files-spotify-playlists/) or not.
    pub is_local: bool,
    /// Information about the track or episode.
    pub track: PlaylistItemType<TrackObject, EpisodeObject>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaylistItemType<T, E> {
    Track(T),
    Episode(E),
}

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-playlisttracksrefobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistTracksRefObject {
    /// A link to the Web API endpoint where full details of the playlist’s tracks can be retrieved.
    pub href: String,
    /// Number of tracks in the playlist.
    pub total: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-privateuserobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivateUserObject {
    /// The country of the user, as set in the user’s account profile. An [ISO 3166-1 alpha-2 country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2). This field is only available when the current user has granted access to the [user-read-private](https://developer.spotify.com/documentation/general/guides/authorization-guide/#list-of-scopes) scope.
    pub country: String,
    /// The name displayed on the user’s profile. `null` if not available.
    pub display_name: String,
    /// The user’s email address, as entered by the user when creating their account. **_Important!_** This email address is unverified; there is no proof that it actually belongs to the user. This field is only available when the current user has granted access to the [user-read-email](https://developer.spotify.com/documentation/general/guides/authorization-guide/#list-of-scopes) scope.
    pub email: String,
    /// The user’s explicit content settings. This field is only available when the current user has granted access to the [user-read-private](https://developer.spotify.com/documentation/general/guides/authorization-guide/#list-of-scopes) scope.
    pub explicit_content: ExplicitContentSettingsObject,
    /// Known external URLs for this user.
    pub external_urls: ExternalUrlObject,
    /// Information about the followers of the user.
    pub followers: FollowersObject,
    /// A link to the Web API endpoint for this user.
    pub href: String,
    /// The [Spotify user ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the user.
    pub id: String,
    /// The user’s profile image.
    pub images: Vec<ImageObject>,
    /// The user’s Spotify subscription level: “premium”, “free”, etc. (The subscription level “open” can be considered the same as “free”.) This field is only available when the current user has granted access to the [user-read-private](https://developer.spotify.com/documentation/general/guides/authorization-guide/#list-of-scopes) scope.
    pub product: String,
    /// The object type: “user”
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the user.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-publicuserobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicUserObject {
    /// The name displayed on the user’s profile. `null` if not available.
    pub display_name: String,
    /// Known public external URLs for this user.
    pub external_urls: ExternalUrlObject,
    /// Information about the followers of this user.
    pub followers: FollowersObject,
    /// A link to the Web API endpoint for this user.
    pub href: String,
    /// The [Spotify user ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for this user.
    pub id: String,
    /// The user’s profile image.
    pub images: Vec<ImageObject>,
    /// The object type: “user”
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for this user.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-recommendationseedobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecommendationSeedObject {
    /// The number of tracks available after min_* and max_* filters have been applied.
    #[serde(rename = "afterFilteringSize")]
    pub after_filtering_size: usize,
    /// The number of tracks available after relinking for regional availability.
    #[serde(rename = "afterRelinkingSize")]
    pub after_relinking_size: usize,
    /// A link to the full track or artist data for this seed. For tracks this will be a link to a [Track Object](https://developer.spotify.com/documentation/web-api/reference/#object-trackobject). For artists a link to [an Artist Object](https://developer.spotify.com/documentation/web-api/reference/#object-artistobject). For genre seeds, this value will be `null`.
    pub href: String,
    /// The id used to select this seed. This will be the same as the string used in the `seed_artists`, `seed_tracks` or `seed_genres` parameter.
    pub id: String,
    /// The number of recommended tracks available for this seed.
    #[serde(rename = "initialPoolSize")]
    pub initial_pool_size: usize,
    /// The entity type of this seed. One of `artist`, `track` or `genre`.
    #[serde(rename = "type")]
    pub _type: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-recommendationsobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecommendationsObject {
    /// An array of [recommendation seed objects](https://developer.spotify.com/documentation/web-api/reference/#object-recommendationseedobject).
    pub seeds: Vec<RecommendationSeedObject>,
    /// An array of [track object (simplified)](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedtrackobject) ordered according to the parameters supplied.
    pub tracks: Vec<SimplifiedTrackObject>,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-resumepointobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResumePointObject {
    /// Whether or not the episode has been fully played by the user.
    pub fully_played: bool,
    /// The user’s most recent position in the episode in milliseconds.
    pub resume_position_ms: usize,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-savedalbumobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedAlbumObject {
    /// The date and time the album was saved Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision; see for example, release_date in an album object.
    pub added_at: String,
    /// Information about the album.
    pub album: AlbumObject,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-savedepisodeobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedEpisodeObject {
    /// The date and time the episode was saved. Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ.
    pub added_at: String,
    /// Information about the episode.
    pub episode: EpisodeObject,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-savedshowobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedShowObject {
    /// The date and time the show was saved. Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision; see for example, release_date in an album object.
    pub added_at: String,
    /// Information about the show.
    pub show: SimplifiedShowObject,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-savedtrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedTrackObject {
    /// The date and time the track was saved. Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision; see for example, release_date in an album object.
    pub added_at: String,
    /// Information about the track.
    pub track: TrackObject,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-showobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShowObject {
    /// A list of the countries in which the show can be played, identified by their [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    pub available_markets: Vec<String>,
    /// The copyright statements of the show.
    pub copyrights: Vec<CopyrightObject>,
    /// A description of the show.
    pub description: String,
    /// A list of the show’s episodes.
    pub episodes: Vec<SimplifiedEpisodeObject>,
    /// Whether or not the show has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,
    /// External URLs for this show.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the show.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the show.
    pub id: String,
    /// The cover art for the show in various sizes, widest first.
    pub images: Vec<ImageObject>,
    /// True if all of the show’s episodes are hosted outside of Spotify’s CDN. This field might be `null` in some cases.
    pub is_externally_hosted: bool,
    /// A list of the languages used in the show, identified by their [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,
    /// The media type of the show.
    pub media_type: String,
    /// The name of the episode.
    pub name: String,
    /// The publisher of the show.
    pub publisher: String,
    /// The object type: “show”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the show.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedalbumobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedAlbumObject {
    /// The type of the album: `album`, `single`, or `compilation`.
    pub album_type: String,
    /// The artists of the album. Each artist object includes a link in `href` to more detailed information about the artist.
    pub artists: Vec<ArtistObject>,
    /// The markets in which the album is available: [ISO 3166-1 alpha-2 country codes.](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) Note that an album is considered available in a market when at least 1 of its tracks is available in that market.
    pub available_markets: Vec<String>,
    /// Known external URLs for this album.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the album.
    pub href: String,
    /// The Spotify ID for the album.
    pub id: String,
    /// The cover art for the album in various sizes, widest first.
    pub images: Vec<ImageObject>,
    /// The name of the album. In case of an album takedown, the value may be an empty string.
    pub name: String,
    /// The date the album was first released, for example “1981-12-15”. Depending on the precision, it might be shown as “1981” or “1981-12”.
    pub release_date: String,
    /// The precision with which release_date value is known: “year” , “month” , or “day”.
    pub release_date_precision: String,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject) for more details.
    pub restrictions: AlbumRestrictionObject,
    /// The object type: “album"
    #[serde(rename = "type")]
    pub _type: String,
    /// The Spotify URI for the album.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedartistobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedArtistObject {
    /// Known external URLs for this artist.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the artist.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub id: String,
    /// The name of the artist.
    pub name: String,
    /// The object type: `"artist"`
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedepisodeobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedEpisodeObject {
    /// A URL to a 30 second preview (MP3 format) of the episode. `null` if not available.
    pub audio_preview_url: String,
    /// A description of the episode. HTML tags are stripped away from this field, use `html_description` field in case HTML tags are needed.
    pub description: String,
    /// The episode length in milliseconds.
    pub duration_ms: usize,
    /// Whether or not the episode has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,
    /// External URLs for this episode.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the episode.
    pub href: String,
    /// A description of the episode. This field may contain HTML tags.
    pub html_description: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the episode.
    pub id: String,
    /// The cover art for the episode in various sizes, widest first.
    pub images: Vec<ImageObject>,
    /// True if the episode is hosted outside of Spotify’s CDN.
    pub is_externally_hosted: bool,
    /// True if the episode is playable in the given market. Otherwise false.
    pub is_playable: bool,
    /// **Note: This field is deprecated and might be removed in the future. Please use the languages field instead.** The language used in the episode, identified by a [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub language: String,
    /// A list of the languages used in the episode, identified by their [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,
    /// The name of the episode.
    pub name: String,
    /// The date the episode was first released, for example `"1981-12-15"`. Depending on the precision, it might be shown as `"1981"` or `"1981-12"`.
    pub release_date: String,
    /// The precision with which `release_date` value is known: `"year"`, `"month"`, or `"day"`.
    pub release_date_precision: String,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-episoderestrictionobject) for more details.
    pub restrictions: EpisodeRestrictionObject,
    /// The user’s most recent position in the episode. Set if the supplied access token is a user token and has the scope `user-read-playback-position`.
    pub resume_point: ResumePointObject,
    /// The object type: “episode”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the episode.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedplaylistobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedPlaylistObject {
    /// `true` if the owner allows other users to modify the playlist.
    pub collaborative: bool,
    /// The playlist description. Only returned for modified, verified playlists, otherwise `null`.
    pub description: String,
    /// Known external URLs for this playlist.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the playlist.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the playlist.
    pub id: String,
    /// Images for the playlist. The array may be empty or contain up to three images. The images are returned by size in descending order. See [Working with Playlists](https://developer.spotify.com/documentation/general/guides/working-with-playlists/). _Note: If returned, the source URL for the image (`url`) is temporary and will expire in less than a day._
    pub images: Vec<ImageObject>,
    /// The name of the playlist.
    pub name: String,
    /// The user who owns the playlist
    pub owner: PublicUserObject,
    /// The playlist’s public/private status: `true` the playlist is public, `false` the playlist is private, `null` the playlist status is not relevant. For more about public/private status, see [Working with Playlists](https://developer.spotify.com/documentation/general/guides/working-with-playlists/)
    pub public: bool,
    /// The version identifier for the current playlist. Can be supplied in other requests to target a specific playlist version
    pub snapshot_id: String,
    /// Information about the tracks of the playlist. Note, a track object may be `null`. This can happen if a track is no longer available.
    pub tracks: Vec<PlaylistTrackObject>,
    /// The object type: “playlist”
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the playlist.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedshowobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedShowObject {
    /// A list of the countries in which the show can be played, identified by their [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    pub available_markets: Vec<String>,
    /// The copyright statements of the show.
    pub copyrights: Vec<CopyrightObject>,
    /// A description of the show.
    pub description: String,
    /// Whether or not the show has explicit content (true = yes it does; false = no it does not OR unknown).
    pub explicit: bool,
    /// External URLs for this show.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the show.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the show.
    pub id: String,
    /// The cover art for the show in various sizes, widest first.
    pub images: Vec<ImageObject>,
    /// True if all of the show’s episodes are hosted outside of Spotify’s CDN. This field might be `null` in some cases.
    pub is_externally_hosted: bool,
    /// A list of the languages used in the show, identified by their [ISO 639](https://en.wikipedia.org/wiki/ISO_639) code.
    pub languages: Vec<String>,
    /// The media type of the show.
    pub media_type: String,
    /// The name of the episode.
    pub name: String,
    /// The publisher of the show.
    pub publisher: String,
    /// The object type: “show”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the show.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedtrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedTrackObject {
    /// The artists who performed the track. Each artist object includes a link in `href` to more detailed information about the artist.
    pub artists: Vec<ArtistObject>,
    /// A list of the countries in which the track can be played, identified by their [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    pub available_markets: Vec<String>,
    /// The disc number (usually `1` unless the album consists of more than one disc).
    pub disc_number: usize,
    /// The track length in milliseconds.
    pub duration_ms: usize,
    /// Whether or not the track has explicit lyrics (`true` = yes it does; `false` = no it does not OR unknown).
    pub explicit: bool,
    /// Known external URLs for this track.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the track.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub id: String,
    /// Whether or not the track is from a local file.
    pub is_local: bool,
    /// Part of the response when [Track Relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/) is applied. If `true`, the track is playable in the given market. Otherwise `false`.
    pub is_playable: bool,
    /// Part of the response when [Track Relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/) is applied, and the requested track has been replaced with different track. The track in the `linked_from` object contains information about the originally requested track.
    pub linked_from: LinkedFrom,
    /// The name of the track.
    pub name: String,
    /// A link to a 30 second preview (MP3 format) of the track. Can be `null`
    pub preview_url: String,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-trackrestrictionobject) for more details.
    pub restrictions: TrackRestrictionObject,
    /// The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub track_number: usize,
    /// The object type: “track”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-trackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackObject {
    /// The album on which the track appears. The album object includes a link in `href` to full information about the album.
    pub album: SimplifiedAlbumObject,
    /// The artists who performed the track. Each artist object includes a link in `href` to more detailed information about the artist.
    pub artists: Vec<ArtistObject>,
    /// A list of the countries in which the track can be played, identified by their [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    pub available_markets: Vec<String>,
    /// The disc number (usually `1` unless the album consists of more than one disc).
    pub disc_number: usize,
    /// The track length in milliseconds.
    pub duration_ms: usize,
    /// Whether or not the track has explicit lyrics ( `true` = yes it does; `false` = no it does not OR unknown).
    pub explicit: bool,
    /// Known external IDs for the track.
    pub external_ids: ExternalIdObject,
    /// Known external URLs for this track.
    pub external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the track.
    pub href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub id: String,
    /// Whether or not the track is from a local file.
    pub is_local: bool,
    /// Part of the response when [Track Relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/) is applied. If `true`, the track is playable in the given market. Otherwise `false`.
    pub is_playable: bool,
    /// Part of the response when [Track Relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/) is applied, and the requested track has been replaced with different track. The track in the `linked_from` object contains information about the originally requested track.
    pub linked_from: LinkedFrom,
    /// The name of the track.
    pub name: String,
    /// The popularity of the track. The value will be between 0 and 100, with 100 being the most popular.
    /// The popularity of a track is a value between 0 and 100, with 100 being the most popular. The popularity is calculated by algorithm and is based, in the most part, on the total number of plays the track has had and how recent those plays are.
    /// Generally speaking, songs that are being played a lot now will have a higher popularity than songs that were played a lot in the past. Duplicate tracks (e.g. the same track from a single and an album) are rated independently. Artist and album popularity is derived mathematically from track popularity. Note that the popularity value may lag actual popularity by a few days: the value is not updated in real time.
    pub popularity: usize,
    /// A link to a 30 second preview (MP3 format) of the track. Can be `null`
    pub preview_url: String,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-trackrestrictionobject) for more details.
    pub restrictions: TrackRestrictionObject,
    /// The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub track_number: usize,
    /// The object type: “track”.
    #[serde(rename = "type")]
    pub _type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub uri: String,
}
/// [Reference](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LinkedFrom {
    pub external_urls: ExternalUrlObject,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-trackrestrictionobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackRestrictionObject {
    /// The reason for the restriction. Supported values:
    /// * `market` - The content item is not available in the given market.
    /// * `product` - The content item is not available for the user’s subscription type.
    /// * `explicit` - The content item is explicit and the user’s account is set to not play explicit content. Additional reasons may be added in the future. Note: If you use this field, make sure that your application safely handles unknown values.
    pub reason: String,
}
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-tuneabletrackobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TuneableTrackObject {
    /// A confidence measure from 0.0 to 1.0 of whether the track is acoustic. 1.0 represents high confidence the track is acoustic.
    pub acousticness: usize,
    /// Danceability describes how suitable a track is for dancing based on a combination of musical elements including tempo, rhythm stability, beat strength, and overall regularity. A value of 0.0 is least danceable and 1.0 is most danceable.
    pub danceability: usize,
    /// The duration of the track in milliseconds.
    pub duration_ms: usize,
    /// Energy is a measure from 0.0 to 1.0 and represents a perceptual measure of intensity and activity. Typically, energetic tracks feel fast, loud, and noisy. For example, death metal has high energy, while a Bach prelude scores low on the scale. Perceptual features contributing to this attribute include dynamic range, perceived loudness, timbre, onset rate, and general entropy.
    pub energy: usize,
    /// Predicts whether a track contains no vocals. “Ooh” and “aah” sounds are treated as instrumental in this context. Rap or spoken word tracks are clearly “vocal”. The closer the instrumentalness value is to 1.0, the greater likelihood the track contains no vocal content. Values above 0.5 are intended to represent instrumental tracks, but confidence is higher as the value approaches 1.0.
    pub instrumentalness: usize,
    /// The key the track is in. Integers map to pitches using standard [Pitch Class notation](https://en.wikipedia.org/wiki/Pitch_class). E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on.
    pub key: usize,
    /// Detects the presence of an audience in the recording. Higher liveness values represent an increased probability that the track was performed live. A value above 0.8 provides strong likelihood that the track is live.
    pub liveness: usize,
    /// The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typical range between -60 and 0 db.
    pub loudness: usize,
    /// Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0.
    pub mode: usize,
    /// The popularity of the track. The value will be between 0 and 100, with 100 being the most popular. The popularity is calculated by algorithm and is based, in the most part, on the total number of plays the track has had and how recent those plays are. _Note: When applying track relinking via the `market` parameter, it is expected to find relinked tracks with popularities that do not match `min_*`, `max_*` and `target_*` popularities. These relinked tracks are accurate replacements for unplayable tracks with the expected popularity scores. Original, non-relinked tracks are available via the `linked_from attribute` of the [relinked track response](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/)._
    pub popularity: usize,
    /// Speechiness detects the presence of spoken words in a track. The more exclusively speech-like the recording (e.g. talk show, audio book, poetry), the closer to 1.0 the attribute value. Values above 0.66 describe tracks that are probably made entirely of spoken words. Values between 0.33 and 0.66 describe tracks that may contain both music and speech, either in sections or layered, including such cases as rap music. Values below 0.33 most likely represent music and other non-speech-like tracks.
    pub speechiness: usize,
    /// The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration.
    pub tempo: usize,
    /// An estimated overall time signature of a track. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure).
    pub time_signature: usize,
    /// A measure from 0.0 to 1.0 describing the musical positiveness conveyed by a track. Tracks with high valence sound more positive (e.g. happy, cheerful, euphoric), while tracks with low valence sound more negative (e.g. sad, depressed, angry).
    pub valence: usize,
}
