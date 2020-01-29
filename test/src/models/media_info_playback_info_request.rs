use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaInfoPlaybackInfoRequest {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "UserId"))]
    pub user_id: String,
    #[serde(rename(deserialize = "MaxStreamingBitrate"), skip_serializing_if = "Option::is_none")]
    pub max_streaming_bitrate: Option<i64>,
    #[serde(rename(deserialize = "StartTimeTicks"), skip_serializing_if = "Option::is_none")]
    pub start_time_ticks: Option<i64>,
    #[serde(rename(deserialize = "AudioStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub audio_stream_index: Option<i32>,
    #[serde(rename(deserialize = "SubtitleStreamIndex"), skip_serializing_if = "Option::is_none")]
    pub subtitle_stream_index: Option<i32>,
    #[serde(rename(deserialize = "MaxAudioChannels"), skip_serializing_if = "Option::is_none")]
    pub max_audio_channels: Option<i32>,
    #[serde(rename(deserialize = "MediaSourceId"))]
    pub media_source_id: String,
    #[serde(rename(deserialize = "LiveStreamId"))]
    pub live_stream_id: String,
    #[serde(rename(deserialize = "DeviceProfile"))]
    pub device_profile: Box<crate::models::DlnaDeviceProfile>,
    #[serde(rename(deserialize = "EnableDirectPlay"))]
    pub enable_direct_play: bool,
    #[serde(rename(deserialize = "EnableDirectStream"))]
    pub enable_direct_stream: bool,
    #[serde(rename(deserialize = "EnableTranscoding"))]
    pub enable_transcoding: bool,
    #[serde(rename(deserialize = "AllowVideoStreamCopy"))]
    pub allow_video_stream_copy: bool,
    #[serde(rename(deserialize = "AllowAudioStreamCopy"))]
    pub allow_audio_stream_copy: bool,
    #[serde(rename(deserialize = "IsPlayback"))]
    pub is_playback: bool,
    #[serde(rename(deserialize = "AutoOpenLiveStream"))]
    pub auto_open_live_stream: bool,
    #[serde(rename(deserialize = "DirectPlayProtocols"))]
    pub direct_play_protocols: DirectPlayProtocols,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DirectPlayProtocols {
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

