use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbyReportsApiModelReportHeader {
    #[serde(rename(deserialize = "HeaderFieldType"))]
    pub header_field_type: HeaderFieldType,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "FieldName"))]
    pub field_name: FieldName,
    #[serde(rename(deserialize = "SortField"))]
    pub sort_field: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: String,
    #[serde(rename(deserialize = "ItemViewType"))]
    pub item_view_type: ItemViewType,
    #[serde(rename(deserialize = "Visible"))]
    pub visible: bool,
    #[serde(rename(deserialize = "DisplayType"))]
    pub display_type: DisplayType,
    #[serde(rename(deserialize = "ShowHeaderLabel"))]
    pub show_header_label: bool,
    #[serde(rename(deserialize = "CanGroup"))]
    pub can_group: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HeaderFieldType {
    #[serde(rename = "String")]
    String,
    #[serde(rename = "Boolean")]
    Boolean,
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "Time")]
    Time,
    #[serde(rename = "DateTime")]
    DateTime,
    #[serde(rename = "Int")]
    Int,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Object")]
    Object,
    #[serde(rename = "Minutes")]
    Minutes,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FieldName {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Path")]
    Path,
    #[serde(rename = "Name")]
    Name,
    #[serde(rename = "PremiereDate")]
    PremiereDate,
    #[serde(rename = "DateAdded")]
    DateAdded,
    #[serde(rename = "ReleaseDate")]
    ReleaseDate,
    #[serde(rename = "Runtime")]
    Runtime,
    #[serde(rename = "PlayCount")]
    PlayCount,
    #[serde(rename = "Season")]
    Season,
    #[serde(rename = "SeasonNumber")]
    SeasonNumber,
    #[serde(rename = "Series")]
    Series,
    #[serde(rename = "Network")]
    Network,
    #[serde(rename = "Year")]
    Year,
    #[serde(rename = "ParentalRating")]
    ParentalRating,
    #[serde(rename = "CommunityRating")]
    CommunityRating,
    #[serde(rename = "Trailers")]
    Trailers,
    #[serde(rename = "Specials")]
    Specials,
    #[serde(rename = "GameSystem")]
    GameSystem,
    #[serde(rename = "AlbumArtist")]
    AlbumArtist,
    #[serde(rename = "Album")]
    Album,
    #[serde(rename = "Disc")]
    Disc,
    #[serde(rename = "Track")]
    Track,
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "EmbeddedImage")]
    EmbeddedImage,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Resolution")]
    Resolution,
    #[serde(rename = "Subtitles")]
    Subtitles,
    #[serde(rename = "Genres")]
    Genres,
    #[serde(rename = "Countries")]
    Countries,
    #[serde(rename = "Status")]
    Status,
    #[serde(rename = "Tracks")]
    Tracks,
    #[serde(rename = "EpisodeSeries")]
    EpisodeSeries,
    #[serde(rename = "EpisodeSeason")]
    EpisodeSeason,
    #[serde(rename = "EpisodeNumber")]
    EpisodeNumber,
    #[serde(rename = "AudioAlbumArtist")]
    AudioAlbumArtist,
    #[serde(rename = "MusicArtist")]
    MusicArtist,
    #[serde(rename = "AudioAlbum")]
    AudioAlbum,
    #[serde(rename = "Locked")]
    Locked,
    #[serde(rename = "ImagePrimary")]
    ImagePrimary,
    #[serde(rename = "ImageBackdrop")]
    ImageBackdrop,
    #[serde(rename = "ImageLogo")]
    ImageLogo,
    #[serde(rename = "Actor")]
    Actor,
    #[serde(rename = "Studios")]
    Studios,
    #[serde(rename = "Composer")]
    Composer,
    #[serde(rename = "Director")]
    Director,
    #[serde(rename = "GuestStar")]
    GuestStar,
    #[serde(rename = "Producer")]
    Producer,
    #[serde(rename = "Writer")]
    Writer,
    #[serde(rename = "Artist")]
    Artist,
    #[serde(rename = "Years")]
    Years,
    #[serde(rename = "ParentalRatings")]
    ParentalRatings,
    #[serde(rename = "CommunityRatings")]
    CommunityRatings,
    #[serde(rename = "Overview")]
    Overview,
    #[serde(rename = "ShortOverview")]
    ShortOverview,
    #[serde(rename = "Type")]
    _Type,
    #[serde(rename = "Date")]
    Date,
    #[serde(rename = "UserPrimaryImage")]
    UserPrimaryImage,
    #[serde(rename = "Severity")]
    Severity,
    #[serde(rename = "Item")]
    Item,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "UserId")]
    UserId,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ItemViewType {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Detail")]
    Detail,
    #[serde(rename = "Edit")]
    Edit,
    #[serde(rename = "List")]
    List,
    #[serde(rename = "ItemByNameDetails")]
    ItemByNameDetails,
    #[serde(rename = "StatusImage")]
    StatusImage,
    #[serde(rename = "EmbeddedImage")]
    EmbeddedImage,
    #[serde(rename = "SubtitleImage")]
    SubtitleImage,
    #[serde(rename = "TrailersImage")]
    TrailersImage,
    #[serde(rename = "SpecialsImage")]
    SpecialsImage,
    #[serde(rename = "LockDataImage")]
    LockDataImage,
    #[serde(rename = "TagsPrimaryImage")]
    TagsPrimaryImage,
    #[serde(rename = "TagsBackdropImage")]
    TagsBackdropImage,
    #[serde(rename = "TagsLogoImage")]
    TagsLogoImage,
    #[serde(rename = "UserPrimaryImage")]
    UserPrimaryImage,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DisplayType {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Screen")]
    Screen,
    #[serde(rename = "Export")]
    Export,
    #[serde(rename = "ScreenExport")]
    ScreenExport,
}

