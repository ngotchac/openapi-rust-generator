use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct VideoHlsServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl VideoHlsServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> VideoHlsServiceApiClient {
        VideoHlsServiceApiClient {
            configuration,
        }
    }
}

impl VideoHlsServiceApiClient {
    pub async fn get_videos_by_id_live_m3u8(&self, id: &str, container: &str, device_profile_id: Option<&str>, device_id: Option<&str>, audio_codec: Option<&str>, enable_auto_stream_copy: Option<bool>, audio_sample_rate: Option<i32>, audio_bit_rate: Option<i32>, audio_channels: Option<i32>, max_audio_channels: Option<i32>, _static: Option<bool>, profile: Option<&str>, level: Option<&str>, framerate: Option<f32>, max_framerate: Option<f32>, copy_timestamps: Option<bool>, start_time_ticks: Option<i64>, width: Option<i32>, height: Option<i32>, max_width: Option<i32>, max_height: Option<i32>, video_bit_rate: Option<i32>, subtitle_stream_index: Option<i32>, subtitle_method: Option<&str>, max_ref_frames: Option<i32>, max_video_bit_depth: Option<i32>, video_codec: Option<&str>, audio_stream_index: Option<i32>, video_stream_index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Videos/{Id}/live.m3u8", configuration.base_path, Id=crate::apis::urlencode(id));
        let mut req_builder = req_client.request(reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = device_profile_id {
            req_builder = req_builder.query(&[("DeviceProfileId", &s.to_string())]);
        }
        if let Some(ref s) = device_id {
            req_builder = req_builder.query(&[("DeviceId", &s.to_string())]);
        }
        req_builder = req_builder.query(&[("Container", &container.to_string())]);
        if let Some(ref s) = audio_codec {
            req_builder = req_builder.query(&[("AudioCodec", &s.to_string())]);
        }
        if let Some(ref s) = enable_auto_stream_copy {
            req_builder = req_builder.query(&[("EnableAutoStreamCopy", &s.to_string())]);
        }
        if let Some(ref s) = audio_sample_rate {
            req_builder = req_builder.query(&[("AudioSampleRate", &s.to_string())]);
        }
        if let Some(ref s) = audio_bit_rate {
            req_builder = req_builder.query(&[("AudioBitRate", &s.to_string())]);
        }
        if let Some(ref s) = audio_channels {
            req_builder = req_builder.query(&[("AudioChannels", &s.to_string())]);
        }
        if let Some(ref s) = max_audio_channels {
            req_builder = req_builder.query(&[("MaxAudioChannels", &s.to_string())]);
        }
        if let Some(ref s) = _static {
            req_builder = req_builder.query(&[("Static", &s.to_string())]);
        }
        if let Some(ref s) = profile {
            req_builder = req_builder.query(&[("Profile", &s.to_string())]);
        }
        if let Some(ref s) = level {
            req_builder = req_builder.query(&[("Level", &s.to_string())]);
        }
        if let Some(ref s) = framerate {
            req_builder = req_builder.query(&[("Framerate", &s.to_string())]);
        }
        if let Some(ref s) = max_framerate {
            req_builder = req_builder.query(&[("MaxFramerate", &s.to_string())]);
        }
        if let Some(ref s) = copy_timestamps {
            req_builder = req_builder.query(&[("CopyTimestamps", &s.to_string())]);
        }
        if let Some(ref s) = start_time_ticks {
            req_builder = req_builder.query(&[("StartTimeTicks", &s.to_string())]);
        }
        if let Some(ref s) = width {
            req_builder = req_builder.query(&[("Width", &s.to_string())]);
        }
        if let Some(ref s) = height {
            req_builder = req_builder.query(&[("Height", &s.to_string())]);
        }
        if let Some(ref s) = max_width {
            req_builder = req_builder.query(&[("MaxWidth", &s.to_string())]);
        }
        if let Some(ref s) = max_height {
            req_builder = req_builder.query(&[("MaxHeight", &s.to_string())]);
        }
        if let Some(ref s) = video_bit_rate {
            req_builder = req_builder.query(&[("VideoBitRate", &s.to_string())]);
        }
        if let Some(ref s) = subtitle_stream_index {
            req_builder = req_builder.query(&[("SubtitleStreamIndex", &s.to_string())]);
        }
        if let Some(ref s) = subtitle_method {
            req_builder = req_builder.query(&[("SubtitleMethod", &s.to_string())]);
        }
        if let Some(ref s) = max_ref_frames {
            req_builder = req_builder.query(&[("MaxRefFrames", &s.to_string())]);
        }
        if let Some(ref s) = max_video_bit_depth {
            req_builder = req_builder.query(&[("MaxVideoBitDepth", &s.to_string())]);
        }
        if let Some(ref s) = video_codec {
            req_builder = req_builder.query(&[("VideoCodec", &s.to_string())]);
        }
        if let Some(ref s) = audio_stream_index {
            req_builder = req_builder.query(&[("AudioStreamIndex", &s.to_string())]);
        }
        if let Some(ref s) = video_stream_index {
            req_builder = req_builder.query(&[("VideoStreamIndex", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

		req_builder.send().await?;
        Ok(())
    }

}
