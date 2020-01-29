use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use super::{Error, configuration};

pub struct HlsSegmentServiceApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl HlsSegmentServiceApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> HlsSegmentServiceApiClient {
        HlsSegmentServiceApiClient {
            configuration,
        }
    }
}

impl HlsSegmentServiceApiClient {
    pub async fn delete_videos_activeencodings(&self, device_id: &str, play_session_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Videos/ActiveEncodings", configuration.base_path);
        let mut req_builder = req_client.request(reqwest::Method::DELETE, uri_str.as_str());

        req_builder = req_builder.query(&[("DeviceId", &device_id.to_string())]);
        req_builder = req_builder.query(&[("PlaySessionId", &play_session_id.to_string())]);
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.bearer_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

		req_builder.send().await?;
        Ok(())
    }

    pub async fn get_videos_by_id_hls_by_playlistid_by_segmentid_by_segmentcontainer(&self, playlist_id: &str, segment_id: &str, segment_container: &str, id: &str, container: &str, device_profile_id: Option<&str>, device_id: Option<&str>, audio_codec: Option<&str>, enable_auto_stream_copy: Option<bool>, audio_sample_rate: Option<i32>, audio_bit_rate: Option<i32>, audio_channels: Option<i32>, max_audio_channels: Option<i32>, _static: Option<bool>, profile: Option<&str>, level: Option<&str>, framerate: Option<f32>, max_framerate: Option<f32>, copy_timestamps: Option<bool>, start_time_ticks: Option<i64>, width: Option<i32>, height: Option<i32>, max_width: Option<i32>, max_height: Option<i32>, video_bit_rate: Option<i32>, subtitle_stream_index: Option<i32>, subtitle_method: Option<&str>, max_ref_frames: Option<i32>, max_video_bit_depth: Option<i32>, video_codec: Option<&str>, audio_stream_index: Option<i32>, video_stream_index: Option<i32>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let req_client = &configuration.client;

        let uri_str = format!("{}/Videos/{Id}/hls/{PlaylistId}/{SegmentId}.{SegmentContainer}", configuration.base_path, PlaylistId=crate::apis::urlencode(playlist_id), SegmentId=crate::apis::urlencode(segment_id), SegmentContainer=crate::apis::urlencode(segment_container), Id=crate::apis::urlencode(id));
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
