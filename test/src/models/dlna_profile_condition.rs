use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DlnaProfileCondition {
    #[serde(rename(deserialize = "Condition"))]
    pub condition: Condition,
    #[serde(rename(deserialize = "Property"))]
    pub property: Property,
    #[serde(rename(deserialize = "Value"))]
    pub value: String,
    #[serde(rename(deserialize = "IsRequired"))]
    pub is_required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Condition {
    #[serde(rename = "Equals")]
    Equals,
    #[serde(rename = "NotEquals")]
    NotEquals,
    #[serde(rename = "LessThanEqual")]
    LessThanEqual,
    #[serde(rename = "GreaterThanEqual")]
    GreaterThanEqual,
    #[serde(rename = "EqualsAny")]
    EqualsAny,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Property {
    #[serde(rename = "AudioChannels")]
    AudioChannels,
    #[serde(rename = "AudioBitrate")]
    AudioBitrate,
    #[serde(rename = "AudioProfile")]
    AudioProfile,
    #[serde(rename = "Width")]
    Width,
    #[serde(rename = "Height")]
    Height,
    #[serde(rename = "Has64BitOffsets")]
    Has64BitOffsets,
    #[serde(rename = "PacketLength")]
    PacketLength,
    #[serde(rename = "VideoBitDepth")]
    VideoBitDepth,
    #[serde(rename = "VideoBitrate")]
    VideoBitrate,
    #[serde(rename = "VideoFramerate")]
    VideoFramerate,
    #[serde(rename = "VideoLevel")]
    VideoLevel,
    #[serde(rename = "VideoProfile")]
    VideoProfile,
    #[serde(rename = "VideoTimestamp")]
    VideoTimestamp,
    #[serde(rename = "IsAnamorphic")]
    IsAnamorphic,
    #[serde(rename = "RefFrames")]
    RefFrames,
    #[serde(rename = "NumAudioStreams")]
    NumAudioStreams,
    #[serde(rename = "NumVideoStreams")]
    NumVideoStreams,
    #[serde(rename = "IsSecondaryAudio")]
    IsSecondaryAudio,
    #[serde(rename = "VideoCodecTag")]
    VideoCodecTag,
    #[serde(rename = "IsAvc")]
    IsAvc,
    #[serde(rename = "IsInterlaced")]
    IsInterlaced,
    #[serde(rename = "AudioSampleRate")]
    AudioSampleRate,
    #[serde(rename = "AudioBitDepth")]
    AudioBitDepth,
}

