use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscodingInfo {
    #[serde(rename(deserialize = "AudioCodec"))]
    pub audio_codec: String,
    #[serde(rename(deserialize = "VideoCodec"))]
    pub video_codec: String,
    #[serde(rename(deserialize = "Container"))]
    pub container: String,
    #[serde(rename(deserialize = "IsVideoDirect"))]
    pub is_video_direct: bool,
    #[serde(rename(deserialize = "IsAudioDirect"))]
    pub is_audio_direct: bool,
    #[serde(rename(deserialize = "Bitrate"), skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i32>,
    #[serde(rename(deserialize = "Framerate"), skip_serializing_if = "Option::is_none")]
    pub framerate: Option<f32>,
    #[serde(rename(deserialize = "CompletionPercentage"), skip_serializing_if = "Option::is_none")]
    pub completion_percentage: Option<f64>,
    #[serde(rename(deserialize = "TranscodingPositionTicks"), skip_serializing_if = "Option::is_none")]
    pub transcoding_position_ticks: Option<f64>,
    #[serde(rename(deserialize = "TranscodingStartPositionTicks"), skip_serializing_if = "Option::is_none")]
    pub transcoding_start_position_ticks: Option<f64>,
    #[serde(rename(deserialize = "Width"), skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename(deserialize = "Height"), skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename(deserialize = "AudioChannels"), skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<i32>,
    #[serde(rename(deserialize = "TranscodeReasons"))]
    pub transcode_reasons: TranscodeReasons,
    #[serde(rename(deserialize = "CurrentCpuUsage"), skip_serializing_if = "Option::is_none")]
    pub current_cpu_usage: Option<f64>,
    #[serde(rename(deserialize = "AverageCpuUsage"), skip_serializing_if = "Option::is_none")]
    pub average_cpu_usage: Option<f64>,
    #[serde(rename(deserialize = "CpuHistory"))]
    pub cpu_history: Vec<Box<crate::models::TupleDoubleDouble>>,
    #[serde(rename(deserialize = "CurrentThrottle"), skip_serializing_if = "Option::is_none")]
    pub current_throttle: Option<i32>,
    #[serde(rename(deserialize = "VideoDecoder"))]
    pub video_decoder: String,
    #[serde(rename(deserialize = "VideoDecoderIsHardware"))]
    pub video_decoder_is_hardware: bool,
    #[serde(rename(deserialize = "VideoDecoderMediaType"))]
    pub video_decoder_media_type: String,
    #[serde(rename(deserialize = "VideoDecoderHwAccel"))]
    pub video_decoder_hw_accel: String,
    #[serde(rename(deserialize = "VideoEncoder"))]
    pub video_encoder: String,
    #[serde(rename(deserialize = "VideoEncoderIsHardware"))]
    pub video_encoder_is_hardware: bool,
    #[serde(rename(deserialize = "VideoEncoderMediaType"))]
    pub video_encoder_media_type: String,
    #[serde(rename(deserialize = "VideoEncoderHwAccel"))]
    pub video_encoder_hw_accel: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TranscodeReasons {
    #[serde(rename = "ContainerNotSupported")]
    ContainerNotSupported,
    #[serde(rename = "VideoCodecNotSupported")]
    VideoCodecNotSupported,
    #[serde(rename = "AudioCodecNotSupported")]
    AudioCodecNotSupported,
    #[serde(rename = "ContainerBitrateExceedsLimit")]
    ContainerBitrateExceedsLimit,
    #[serde(rename = "AudioBitrateNotSupported")]
    AudioBitrateNotSupported,
    #[serde(rename = "AudioChannelsNotSupported")]
    AudioChannelsNotSupported,
    #[serde(rename = "VideoResolutionNotSupported")]
    VideoResolutionNotSupported,
    #[serde(rename = "UnknownVideoStreamInfo")]
    UnknownVideoStreamInfo,
    #[serde(rename = "UnknownAudioStreamInfo")]
    UnknownAudioStreamInfo,
    #[serde(rename = "AudioProfileNotSupported")]
    AudioProfileNotSupported,
    #[serde(rename = "AudioSampleRateNotSupported")]
    AudioSampleRateNotSupported,
    #[serde(rename = "AnamorphicVideoNotSupported")]
    AnamorphicVideoNotSupported,
    #[serde(rename = "InterlacedVideoNotSupported")]
    InterlacedVideoNotSupported,
    #[serde(rename = "SecondaryAudioNotSupported")]
    SecondaryAudioNotSupported,
    #[serde(rename = "RefFramesNotSupported")]
    RefFramesNotSupported,
    #[serde(rename = "VideoBitDepthNotSupported")]
    VideoBitDepthNotSupported,
    #[serde(rename = "VideoBitrateNotSupported")]
    VideoBitrateNotSupported,
    #[serde(rename = "VideoFramerateNotSupported")]
    VideoFramerateNotSupported,
    #[serde(rename = "VideoLevelNotSupported")]
    VideoLevelNotSupported,
    #[serde(rename = "VideoProfileNotSupported")]
    VideoProfileNotSupported,
    #[serde(rename = "AudioBitDepthNotSupported")]
    AudioBitDepthNotSupported,
    #[serde(rename = "SubtitleCodecNotSupported")]
    SubtitleCodecNotSupported,
    #[serde(rename = "DirectPlayError")]
    DirectPlayError,
}

