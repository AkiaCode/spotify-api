/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-albumobject)
pub struct AlbumObject {
    /// The type of the album: `album`, `single`, or `compilation`.
    album_type: String,
    /// The artists of the album. Each artist object includes a link in `href` to more detailed information about the artist.
    artists: Vec<ArtistObject>,
    /// The markets in which the album is available: [ISO 3166-1 alpha-2 country codes.](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) Note that an album is considered available in a market when at least 1 of its tracks is available in that market.
    available_markets: Vec<String>,
    /// The copyright statements of the album.
    copyrights: Vec<CopyrightObject>,
    /// Known external IDs for the album.
    external_ids: ExternalIdObject,
    /// Known external URLs for this album.
    external_urls: ExternalUrlObject,
    /// A list of the genres used to classify the album. For example: “Prog Rock” , “Post-Grunge”. (If not yet classified, the array is empty.)
    genres: Vec<String>,
    /// A link to the Web API endpoint providing full details of the album.
    href: String,
    /// The Spotify ID for the album.
    id: String,
    /// The cover art for the album in various sizes, widest first.
    images: Vec<ImageObject>,
    /// The label for the album.
    label: String,
    /// The name of the album. In case of an album takedown, the value may be an empty string.
    name: String,
    /// The popularity of the album. The value will be between 0 and 100, with 100 being the most popular. The popularity is calculated from the popularity of the album’s individual tracks.
    popularity: usize,
    /// The date the album was first released, for example “1981-12-15”. Depending on the precision, it might be shown as “1981” or “1981-12”.
    release_date: String,
    /// The precision with which release_date value is known: “year” , “month” , or “day”.
    release_date_precision: String,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject) for more details.
    restrictions: AlbumRestrictionObject,
    /// The tracks of the album.
    tracks: Vec<SimplifiedTrackObject>,
    /// The object type: “album"
    r#type: String,
    /// The Spotify URI for the album.
    uri: String,
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject)
pub struct AlbumRestrictionObject {
    /// The reason for the restriction. Supported values:
    /// * `market` - The content item is not available in the given market.
    /// * `product` - The content item is not available for the user’s subscription type.
    /// * `explicit` - The content item is explicit and the user’s account is set to not play explicit content. Additional reasons may be added in the future. **Note**: If you use this field, make sure that your application safely handles unknown values.
    reason: String,
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-artistobject)
pub struct ArtistObject {
    /// Known external URLs for this artist.
    external_urls: ExternalUrlObject,
    /// Information about the followers of the artist.
    followers: FollowersObject,
    /// A list of the genres the artist is associated with. For example: `"Prog Rock"` , `"Post-Grunge"`. (If not yet classified, the array is empty.)
    genres: Vec<String>,
    /// A link to the Web API endpoint providing full details of the artist.
    href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    id: String,
    /// Images of the artist in various sizes, widest first.
    images: Vec<ImageObject>,
    /// The name of the artist.
    name: String,
    /// The popularity of the artist. The value will be between 0 and 100, with 100 being the most popular. The artist’s popularity is calculated from the popularity of all the artist’s tracks.
    popularity: usize,
    /// The object type: `"artist"`
    r#type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    uri: String,
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-audiofeaturesobject)
pub struct AudioFeaturesObject {
    /// A confidence measure from 0.0 to 1.0 of whether the track is acoustic. 1.0 represents high confidence the track is acoustic.
    acousticness: usize,
    /// An HTTP URL to access the full audio analysis of this track. An access token is required to access this data.
    analysis_url: String,
    /// Danceability describes how suitable a track is for dancing based on a combination of musical elements including tempo, rhythm stability, beat strength, and overall regularity. A value of 0.0 is least danceable and 1.0 is most danceable.
    danceability: usize,
    /// The duration of the track in milliseconds.
    duration_ms: usize,
    /// Energy is a measure from 0.0 to 1.0 and represents a perceptual measure of intensity and activity. Typically, energetic tracks feel fast, loud, and noisy. For example, death metal has high energy, while a Bach prelude scores low on the scale. Perceptual features contributing to this attribute include dynamic range, perceived loudness, timbre, onset rate, and general entropy.
    energy: usize,
    /// The Spotify ID for the track.
    id: String,
    /// Predicts whether a track contains no vocals. “Ooh” and “aah” sounds are treated as instrumental in this context. Rap or spoken word tracks are clearly “vocal”. The closer the instrumentalness value is to 1.0, the greater likelihood the track contains no vocal content. Values above 0.5 are intended to represent instrumental tracks, but confidence is higher as the value approaches 1.0.
    instrumentalness: usize,
    /// The key the track is in. Integers map to pitches using standard [Pitch Class notation](https://en.wikipedia.org/wiki/Pitch_class). E.g. 0 = C, 1 = C♯/D♭, 2 = D, and so on.
    key: usize,
    /// Detects the presence of an audience in the recording. Higher liveness values represent an increased probability that the track was performed live. A value above 0.8 provides strong likelihood that the track is live.
    liveness: usize,
    /// The overall loudness of a track in decibels (dB). Loudness values are averaged across the entire track and are useful for comparing relative loudness of tracks. Loudness is the quality of a sound that is the primary psychological correlate of physical strength (amplitude). Values typical range between -60 and 0 db.
    loudness: usize,
    /// Mode indicates the modality (major or minor) of a track, the type of scale from which its melodic content is derived. Major is represented by 1 and minor is 0.
    mode: usize,
    /// Speechiness detects the presence of spoken words in a track. The more exclusively speech-like the recording (e.g. talk show, audio book, poetry), the closer to 1.0 the attribute value. Values above 0.66 describe tracks that are probably made entirely of spoken words. Values between 0.33 and 0.66 describe tracks that may contain both music and speech, either in sections or layered, including such cases as rap music. Values below 0.33 most likely represent music and other non-speech-like tracks.
    speechiness: usize,
    /// The overall estimated tempo of a track in beats per minute (BPM). In musical terminology, tempo is the speed or pace of a given piece and derives directly from the average beat duration.
    tempo: usize,
    /// An estimated overall time signature of a track. The time signature (meter) is a notational convention to specify how many beats are in each bar (or measure).
    time_signature: usize,
    /// A link to the Web API endpoint providing full details of the track.
    track_href: String,
    /// The object type: “audio_features”
    r#type: String,
    /// The Spotify URI for the track.
    uri: String,
    /// A measure from 0.0 to 1.0 describing the musical positiveness conveyed by a track. Tracks with high valence sound more positive (e.g. happy, cheerful, euphoric), while tracks with low valence sound more negative (e.g. sad, depressed, angry).
    valence: usize,
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-categoryobject)
pub struct CategoryObject {
    /// A link to the Web API endpoint returning full details of the category.
    href: String,
    /// The category icon, in various sizes.
    icons: Vec<ImageObject>,
    /// The [Spotify category ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) of the category.
    id: String,
    /// The name of the category.
    name: String,
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-contextobject)
pub struct ContextObject {
    /// External URLs for this context.
    external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the track.
    href: String,
    /// The object type, e.g. “artist”, “playlist”, “album”, “show”.
    r#type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the context.
    uri: String,
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-copyrightobject)
pub struct CopyrightObject {
    /// The copyright text for this content.
    text: String,
    /// The type of copyright: `C` = the copyright, `P` = the sound recording (performance) copyright.
    r#type: String,
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-currentlyplayingcontextobject)
pub struct CurrentlyPlayingContextObject {}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-currentlyplayingobject)
pub struct CurrentlyPlayingObject {}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-cursorobject)
pub struct CursorObject {
    /// The cursor to use as key to find the next page of items.
    after: String
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-cursorpagingobject)
pub struct CursorPagingObject {
    /// The cursors used to find the next set of items.
    cursors: CursorObject,
    /// A link to the Web API endpoint returning the full result of the request.
    href: String,
    /// The requested data.
    //items: Vec<Object>
    /// The maximum number of items in the response (as set in the query or by default)
    limit: usize,
    /// URL to the next page of items. (`null` if none)
    next: String,
    /// The total number of items available to return.
    total: usize,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-deviceobject)
pub struct DeviceObject {
    /// The device ID. This may be `null`.
    id: String,
    /// If this device is the currently active device.
    is_active: bool,
    /// If this device is currently in a private session.
    is_private_session: bool,
    /// Whether controlling this device is restricted. At present if this is “true” then no Web API commands will be accepted by this device.
    is_restricted: bool,
    /// The name of the device.
    name: String,
    /// Device type, such as “computer”, “smartphone” or “speaker”.
    r#type: String,
    /// The current volume in percent. This may be null.
    volume_percent: usize,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-devicesobject)
pub struct DevicesObject {
    /// A list of 0..n Device objects
    devices: Vec<DeviceObject>
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-disallowsobject)
pub struct DisallowsObject {}
/// [Docs]()
pub struct EpisodeObject {}
/// [Docs]()
pub struct EpisodeRestrictionObject {}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-errorobject)
pub struct ErrorObject {
    /// A short description of the cause of the error.
    message: String,
    /// The HTTP status code (also returned in the response header; see [Response Status Codes](https://developer.spotify.com/documentation/web-api/#response-status-codes) for more information).
    status: usize,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-explicitcontentsettingsobject)
pub struct ExplicitContentSettingsObject {
    /// When `true`, indicates that explicit content should not be played.
    filter_enabled: bool,
    /// When `true`, indicates that the explicit content setting is locked and can’t be changed by the user.
    filter_locked: bool,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-externalidobject)
pub struct ExternalIdObject {
    /// [International Article Number](https://en.wikipedia.org/wiki/International_Article_Number)
    ean: String,
    /// [International Standard Recording Code](https://en.wikipedia.org/wiki/International_Standard_Recording_Code)
    isrc: String,
    /// [Universal Product Code](https://en.wikipedia.org/wiki/Universal_Product_Code)
    upc: String,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-externalurlobject)
pub struct ExternalUrlObject {
    /// The [Spotify URL](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the object.
    spotify: String,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-followersobject)
pub struct FollowersObject {
    /// A link to the Web API endpoint providing full details of the followers; `null` if not available. Please note that this will always be set to null, as the Web API does not support it at the moment.
    href: String,
    /// The total number of followers.
    total: usize,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-imageobject)
pub struct ImageObject {
    /// The image height in pixels. If unknown: `null` or not returned.
    height: usize,
    /// The source URL of the image.
    url: String,
    /// The image width in pixels. If unknown: `null` or not returned.
    width: usize,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-linkedtrackobject)
pub struct LinkedTrackObject {
    /// Known external URLs for this track.
    external_urls: ExternalUrlObject,
    /// A link to the Web API endpoint providing full details of the track.
    href: String,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    id: String,
    /// The object type: “track”.
    r#type: String,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    uri: String,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-pagingobject)
pub struct PagingObject {
    /// A link to the Web API endpoint returning the full result of the request
    href: String,
    /// The requested data.
    //items: Vec<Object>,
    /// The maximum number of items in the response (as set in the query or by default).
    limit: usize,
    /// URL to the next page of items. (`null` if none)
    next: String,
    /// The offset of the items returned (as set in the query or by default)
    offset: usize,
    /// URL to the previous page of items. (`null` if none)
    previous: String,
    /// The total number of items available to return.
    total: usize,
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-playhistoryobject)
pub struct PlayHistoryObject {
    /// The context the track was played from.
    context: ContextObject,
    /// The date and time the track was played.
    //played_at: Timestamp,
    /// The track the user listened to.
    track: SimplifiedTrackObject
}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-playererrorobject)
pub struct PlayerErrorObject {
    message: String,
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
    reason: String,
    /// The HTTP status code. Either `404 NOT FOUND` or `403 FORBIDDEN`. Also returned in the response header.
    status: usize
}
/// [Docs]()
pub struct PlaylistObject {}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-playlisttrackobject)
pub struct PlaylistTrackObject {
    /// The date and time the track or episode was added. Note that some very old playlists may return `null` in this field.
    //added_at: Timestamp,
    /// The Spotify user who added the track or episode. Note that some very old playlists may return `null` in this field.
    added_by: PublicUserObject,
    /// Whether this track or episode is a [local file](https://developer.spotify.com/documentation/general/guides/local-files-spotify-playlists/) or not.
    is_local: bool,
    /// Information about the track or episode.
    track: PlaylistItemType<TrackObject, EpisodeObject>
}

pub enum PlaylistItemType<T, E> {
    Track(T),
    Episode(E)
}

/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-playlisttracksrefobject)
pub struct PlaylistTracksRefObject {
    /// A link to the Web API endpoint where full details of the playlist’s tracks can be retrieved.
    href: String,
    /// Number of tracks in the playlist.
    total: usize,
}
/// [Docs]()
pub struct PrivateUserObject {}
/// [Docs]()
pub struct PublicUserObject {

}
/// [Docs]()
pub struct RecommendationSeedObject {

}
/// [Docs]()
pub struct RecommendationsObject {

}
/// [Docs]()
pub struct ResumePointObject {

}
/// [Docs]()
pub struct SavedAlbumObject {

}
/// [Docs]()
pub struct SavedEpisodeObject {

}
/// [Docs]()
pub struct SavedShowObject {

}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-savedtrackobject)
pub struct SavedTrackObject {
    /// The date and time the track was saved. Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ. If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision; see for example, release_date in an album object.
    //added_at: Timestamp,
    /// Information about the track.
    track: TrackObject,
}
/// [Docs]()
pub struct ShowObject {}
/// [Docs]()
pub struct SimplifiedAlbumObject {}
/// [Docs]()
pub struct SimplifiedArtistObject {

}
/// [Docs]()
pub struct SimplifiedEpisodeObject {}
/// [Docs]()
pub struct SimplifiedPlaylistObject {}
/// [Docs]()
pub struct SimplifiedShowObject {}
/// [Docs]()
pub struct SimplifiedTrackObject {}
/// [Docs]()
pub struct TrackObject {}
/// [Docs](https://developer.spotify.com/documentation/web-api/reference/#object-trackrestrictionobject)
pub struct TrackRestrictionObject {
    /// The reason for the restriction. Supported values:
    /// * `market` - The content item is not available in the given market.
    /// * `product` - The content item is not available for the user’s subscription type.
    /// * `explicit` - The content item is explicit and the user’s account is set to not play explicit content. Additional reasons may be added in the future. Note: If you use this field, make sure that your application safely handles unknown values.
    reason: String,
}
/// [Docs]()
pub struct TuneableTrackObject {}
