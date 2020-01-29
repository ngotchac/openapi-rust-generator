use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaTranscodingProfile {
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "VideoCodec"))]
    pub video_codec: String,
    #[serde(rename(deserialize = "AudioCodec"))]
    pub audio_codec: String,
    #[serde(rename(deserialize = "Protocol"))]
    pub protocol: String,
    #[serde(rename(deserialize = "EstimateContentLength"))]
    pub estimate_content_length: bool,
    #[serde(rename(deserialize = "EnableMpegtsM2TsMode"))]
    pub enable_mpegts_m2_ts_mode: bool,
    #[serde(rename(deserialize = "TranscodeSeekInfo"))]
    pub transcode_seek_info: TranscodeSeekInfo,
    #[serde(rename(deserialize = "CopyTimestamps"))]
    pub copy_timestamps: bool,
    #[serde(rename(deserialize = "Context"))]
    pub context: Context,
    #[serde(rename(deserialize = "MaxAudioChannels"))]
    pub max_audio_channels: String,
    #[serde(rename(deserialize = "MinSegments"))]
    pub min_segments: i32,
    #[serde(rename(deserialize = "SegmentLength"))]
    pub segment_length: i32,
    #[serde(rename(deserialize = "BreakOnNonKeyFrames"))]
    pub break_on_non_key_frames: bool,
    #[serde(rename(deserialize = "ManifestSubtitles"))]
    pub manifest_subtitles: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Photo")]
    Photo,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TranscodeSeekInfo {
    #[serde(rename = "Auto")]
    Auto,
    #[serde(rename = "Bytes")]
    Bytes,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Context {
    #[serde(rename = "Streaming")]
    Streaming,
    #[serde(rename = "Static")]
    _Static,
}

