use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaStream {
    #[serde(rename(deserialize = "Codec"))]
    pub codec: String,
    #[serde(rename(deserialize = "CodecTag"))]
    pub codec_tag: String,
    #[serde(rename(deserialize = "Language"))]
    pub language: String,
    #[serde(rename(deserialize = "ColorTransfer"))]
    pub color_transfer: String,
    #[serde(rename(deserialize = "ColorPrimaries"))]
    pub color_primaries: String,
    #[serde(rename(deserialize = "ColorSpace"))]
    pub color_space: String,
    #[serde(rename(deserialize = "Comment"))]
    pub comment: String,
    #[serde(rename(deserialize = "TimeBase"))]
    pub time_base: String,
    #[serde(rename(deserialize = "CodecTimeBase"))]
    pub codec_time_base: String,
    #[serde(rename(deserialize = "Title"))]
    pub title: String,
    #[serde(rename(deserialize = "Extradata"))]
    pub extradata: String,
    #[serde(rename(deserialize = "VideoRange"))]
    pub video_range: String,
    #[serde(rename(deserialize = "DisplayTitle"))]
    pub display_title: String,
    #[serde(rename(deserialize = "DisplayLanguage"))]
    pub display_language: String,
    #[serde(rename(deserialize = "NalLengthSize"))]
    pub nal_length_size: String,
    #[serde(rename(deserialize = "IsInterlaced"))]
    pub is_interlaced: bool,
    #[serde(rename(deserialize = "IsAVC"), skip_serializing_if = "Option::is_none")]
    pub is_avc: Option<bool>,
    #[serde(rename(deserialize = "ChannelLayout"))]
    pub channel_layout: String,
    #[serde(rename(deserialize = "BitRate"), skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<i32>,
    #[serde(rename(deserialize = "BitDepth"), skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i32>,
    #[serde(rename(deserialize = "RefFrames"), skip_serializing_if = "Option::is_none")]
    pub ref_frames: Option<i32>,
    #[serde(rename(deserialize = "PacketLength"), skip_serializing_if = "Option::is_none")]
    pub packet_length: Option<i32>,
    #[serde(rename(deserialize = "Channels"), skip_serializing_if = "Option::is_none")]
    pub channels: Option<i32>,
    #[serde(rename(deserialize = "SampleRate"), skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    #[serde(rename(deserialize = "IsDefault"))]
    pub is_default: bool,
    #[serde(rename(deserialize = "IsForced"))]
    pub is_forced: bool,
    #[serde(rename(deserialize = "Height"), skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename(deserialize = "Width"), skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename(deserialize = "AverageFrameRate"), skip_serializing_if = "Option::is_none")]
    pub average_frame_rate: Option<f32>,
    #[serde(rename(deserialize = "RealFrameRate"), skip_serializing_if = "Option::is_none")]
    pub real_frame_rate: Option<f32>,
    #[serde(rename(deserialize = "Profile"))]
    pub profile: String,
    #[serde(rename(deserialize = "Type"))]
    pub _type: Type,
    #[serde(rename(deserialize = "AspectRatio"))]
    pub aspect_ratio: String,
    #[serde(rename(deserialize = "Index"))]
    pub index: i32,
    #[serde(rename(deserialize = "Score"), skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(rename(deserialize = "IsExternal"))]
    pub is_external: bool,
    #[serde(rename(deserialize = "DeliveryMethod"))]
    pub delivery_method: DeliveryMethod,
    #[serde(rename(deserialize = "DeliveryUrl"))]
    pub delivery_url: String,
    #[serde(rename(deserialize = "IsExternalUrl"), skip_serializing_if = "Option::is_none")]
    pub is_external_url: Option<bool>,
    #[serde(rename(deserialize = "IsTextSubtitleStream"))]
    pub is_text_subtitle_stream: bool,
    #[serde(rename(deserialize = "SupportsExternalStream"))]
    pub supports_external_stream: bool,
    #[serde(rename(deserialize = "Path"))]
    pub path: String,
    #[serde(rename(deserialize = "PixelFormat"))]
    pub pixel_format: String,
    #[serde(rename(deserialize = "Level"), skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
    #[serde(rename(deserialize = "IsAnamorphic"), skip_serializing_if = "Option::is_none")]
    pub is_anamorphic: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Subtitle")]
    Subtitle,
    #[serde(rename = "EmbeddedImage")]
    EmbeddedImage,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeliveryMethod {
    #[serde(rename = "Encode")]
    Encode,
    #[serde(rename = "Embed")]
    Embed,
    #[serde(rename = "External")]
    External,
    #[serde(rename = "Hls")]
    Hls,
}

