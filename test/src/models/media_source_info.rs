use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaSourceInfo {
    #[serde(rename(deserialize = "Protocol"))]
    pub protocol: Protocol,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "EncoderPath"))]
    pub encoder_path: String,
    #[serde(rename(deserialize = "EncoderProtocol"))]
    pub encoder_protocol: EncoderProtocol,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
    #[serde(rename(deserialize = "Size"), skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "IsRemote"))]
    pub is_remote: bool,
    #[serde(rename(deserialize = "RunTimeTicks"), skip_serializing_if = "Option::is_none")]
    pub run_time_ticks: Option<i64>,
    #[serde(rename(deserialize = "SupportsTranscoding"))]
    pub supports_transcoding: bool,
    #[serde(rename(deserialize = "SupportsDirectStream"))]
    pub supports_direct_stream: bool,
    #[serde(rename(deserialize = "SupportsDirectPlay"))]
    pub supports_direct_play: bool,
    #[serde(rename(deserialize = "IsInfiniteStream"))]
    pub is_infinite_stream: bool,
    #[serde(rename(deserialize = "RequiresOpening"))]
    pub requires_opening: bool,
    #[serde(rename(deserialize = "OpenToken"))]
    pub open_token: String,
    #[serde(rename(deserialize = "RequiresClosing"))]
    pub requires_closing: bool,
    #[serde(rename(deserialize = "LiveStreamId"))]
    pub live_stream_id: String,
    #[serde(rename(deserialize = "BufferMs"), skip_serializing_if = "Option::is_none")]
    pub buffer_ms: Option<i32>,
    #[serde(rename(deserialize = "RequiresLooping"))]
    pub requires_looping: bool,
    #[serde(rename(deserialize = "SupportsProbing"))]
    pub supports_probing: bool,
    #[serde(rename(deserialize = "Video3DFormat"))]
    pub video3_d_format: Video3DFormat,
    #[serde(rename(deserialize = "MediaStreams"))]
    pub media_streams: Vec<Box<crate::models::MediaStream>>,
    #[serde(rename(deserialize = "Formats"))]
    pub formats: Vec<String>,
    #[serde(rename(deserialize = "Bitrate"), skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename(deserialize = "Timestamp"))]
    pub timestamp: Timestamp,
    #[serde(rename(deserialize = "RequiredHttpHeaders"))]
    pub required_http_headers: ::std::collections::HashMap<String, String>,
    #[serde(rename(deserialize = "TranscodingUrl"))]
    pub transcoding_url: String,
    #[serde(rename(deserialize = "TranscodingSubProtocol"))]
    pub transcoding_sub_protocol: String,
    #[serde(rename(deserialize = "TranscodingContainer"))]
    pub transcoding_container: String,
    #[serde(rename(deserialize = "AnalyzeDurationMs"), skip_serializing_if = "Option::is_none")]
    pub analyze_duration_ms: Option<i32>,
    #[serde(rename(deserialize = "ReadAtNativeFramerate"))]
    pub read_at_native_framerate: bool,
    #[serde(rename(deserialize = "DefaultAudioStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub default_audio_stream_index: Option<i32>,
    #[serde(rename(deserialize = "DefaultSubtitleStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub default_subtitle_stream_index: Option<i32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Http")]
    Http,
    #[serde(rename = "Rtmp")]
    Rtmp,
    #[serde(rename = "Rtsp")]
    Rtsp,
    #[serde(rename = "Udp")]
    Udp,
    #[serde(rename = "Rtp")]
    Rtp,
    #[serde(rename = "Ftp")]
    Ftp,
    #[serde(rename = "Mms")]
    Mms,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EncoderProtocol {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Http")]
    Http,
    #[serde(rename = "Rtmp")]
    Rtmp,
    #[serde(rename = "Rtsp")]
    Rtsp,
    #[serde(rename = "Udp")]
    Udp,
    #[serde(rename = "Rtp")]
    Rtp,
    #[serde(rename = "Ftp")]
    Ftp,
    #[serde(rename = "Mms")]
    Mms,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "Grouping")]
    Grouping,
    #[serde(rename = "Placeholder")]
    Placeholder,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Video3DFormat {
    #[serde(rename = "HalfSideBySide")]
    HalfSideBySide,
    #[serde(rename = "FullSideBySide")]
    FullSideBySide,
    #[serde(rename = "FullTopAndBottom")]
    FullTopAndBottom,
    #[serde(rename = "HalfTopAndBottom")]
    HalfTopAndBottom,
    #[serde(rename = "MVC")]
    MVC,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Timestamp {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Zero")]
    Zero,
    #[serde(rename = "Valid")]
    Valid,
}

