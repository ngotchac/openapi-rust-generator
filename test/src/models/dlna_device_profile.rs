use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaDeviceProfile {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Identification"))]
    pub identification: Box<crate::models::DlnaDeviceIdentification>,
    #[serde(rename(deserialize = "FriendlyName"))]
    pub friendly_name: String,
    #[serde(rename(deserialize = "Manufacturer"))]
    pub manufacturer: String,
    #[serde(rename(deserialize = "ManufacturerUrl"))]
    pub manufacturer_url: String,
    #[serde(rename(deserialize = "ModelName"))]
    pub model_name: String,
    #[serde(rename(deserialize = "ModelDescription"))]
    pub model_description: String,
    #[serde(rename(deserialize = "ModelNumber"))]
    pub model_number: String,
    #[serde(rename(deserialize = "ModelUrl"))]
    pub model_url: String,
    #[serde(rename(deserialize = "SerialNumber"))]
    pub serial_number: String,
    #[serde(rename(deserialize = "EnableAlbumArtInDidl"))]
    pub enable_album_art_in_didl: bool,
    #[serde(rename(deserialize = "EnableSingleAlbumArtLimit"))]
    pub enable_single_album_art_limit: bool,
    #[serde(rename(deserialize = "EnableSingleSubtitleLimit"))]
    pub enable_single_subtitle_limit: bool,
    #[serde(rename(deserialize = "SupportedMediaTypes"))]
    pub supported_media_types: String,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "AlbumArtPn"))]
    pub album_art_pn: String,
    #[serde(rename(deserialize = "MaxAlbumArtWidth"))]
    pub max_album_art_width: i32,
    #[serde(rename(deserialize = "MaxAlbumArtHeight"))]
    pub max_album_art_height: i32,
    #[serde(rename(deserialize = "MaxIconWidth"), skip_serializing_if = "Option::is_none")]
    pub max_icon_width: Option<i32>,
    #[serde(rename(deserialize = "MaxIconHeight"), skip_serializing_if = "Option::is_none")]
    pub max_icon_height: Option<i32>,
    #[serde(rename(deserialize = "MaxStreamingBitrate"), skip_serializing_if = "Option::is_none")]
    pub max_streaming_bitrate: Option<i64>,
    #[serde(rename(deserialize = "MaxStaticBitrate"), skip_serializing_if = "Option::is_none")]
    pub max_static_bitrate: Option<i64>,
    #[serde(rename(deserialize = "MusicStreamingTranscodingBitrate"), skip_serializing_if = "Option::is_none")]
    pub music_streaming_transcoding_bitrate: Option<i32>,
    #[serde(rename(deserialize = "MaxStaticMusicBitrate"), skip_serializing_if = "Option::is_none")]
    pub max_static_music_bitrate: Option<i32>,
    #[serde(rename(deserialize = "SonyAggregationFlags"))]
    pub sony_aggregation_flags: String,
    #[serde(rename(deserialize = "ProtocolInfo"))]
    pub protocol_info: String,
    #[serde(rename(deserialize = "TimelineOffsetSeconds"))]
    pub timeline_offset_seconds: i32,
    #[serde(rename(deserialize = "RequiresPlainVideoItems"))]
    pub requires_plain_video_items: bool,
    #[serde(rename(deserialize = "RequiresPlainFolders"))]
    pub requires_plain_folders: bool,
    #[serde(rename(deserialize = "EnableMSMediaReceiverRegistrar"))]
    pub enable_ms_media_receiver_registrar: bool,
    #[serde(rename(deserialize = "IgnoreTranscodeByteRangeRequests"))]
    pub ignore_transcode_byte_range_requests: bool,
    #[serde(rename(deserialize = "XmlRootAttributes"))]
    pub xml_root_attributes: Vec<Box<crate::models::DlnaXmlAttribute>>,
    #[serde(rename(deserialize = "DirectPlayProfiles"))]
    pub direct_play_profiles: Vec<Box<crate::models::DlnaDirectPlayProfile>>,
    #[serde(rename(deserialize = "TranscodingProfiles"))]
    pub transcoding_profiles: Vec<Box<crate::models::DlnaTranscodingProfile>>,
    #[serde(rename(deserialize = "ContainerProfiles"))]
    pub container_profiles: Vec<Box<crate::models::DlnaContainerProfile>>,
    #[serde(rename(deserialize = "CodecProfiles"))]
    pub codec_profiles: Vec<Box<crate::models::DlnaCodecProfile>>,
    #[serde(rename(deserialize = "ResponseProfiles"))]
    pub response_profiles: Vec<Box<crate::models::DlnaResponseProfile>>,
    #[serde(rename(deserialize = "SubtitleProfiles"))]
    pub subtitle_profiles: Vec<Box<crate::models::DlnaSubtitleProfile>>,
}


