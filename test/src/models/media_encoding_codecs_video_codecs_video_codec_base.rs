use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaEncodingCodecsVideoCodecsVideoCodecBase {
    #[serde(rename(deserialize = "CodecKind"))]
    pub codec_kind: CodecKind,
    #[serde(rename(deserialize = "MediaTypeName"))]
    pub media_type_name: String,
    #[serde(rename(deserialize = "VideoMediaType"))]
    pub video_media_type: VideoMediaType,
    #[serde(rename(deserialize = "MinWidth"), skip_serializing_if = "Option::is_none")]
    pub min_width: Option<i32>,
    #[serde(rename(deserialize = "MaxWidth"), skip_serializing_if = "Option::is_none")]
    pub max_width: Option<i32>,
    #[serde(rename(deserialize = "MinHeight"), skip_serializing_if = "Option::is_none")]
    pub min_height: Option<i32>,
    #[serde(rename(deserialize = "MaxHeight"), skip_serializing_if = "Option::is_none")]
    pub max_height: Option<i32>,
    #[serde(rename(deserialize = "WidthAlignment"), skip_serializing_if = "Option::is_none")]
    pub width_alignment: Option<i32>,
    #[serde(rename(deserialize = "HeightAlignment"), skip_serializing_if = "Option::is_none")]
    pub height_alignment: Option<i32>,
    #[serde(rename(deserialize = "MinFrameRate"), skip_serializing_if = "Option::is_none")]
    pub min_frame_rate: Option<i32>,
    #[serde(rename(deserialize = "MaxFrameRate"), skip_serializing_if = "Option::is_none")]
    pub max_frame_rate: Option<i32>,
    #[serde(rename(deserialize = "SupportedColorFormats"))]
    pub supported_color_formats: SupportedColorFormats,
    #[serde(rename(deserialize = "SupportedColorFormatStrings"))]
    pub supported_color_format_strings: Vec<String>,
    #[serde(rename(deserialize = "ProfileAndLevelInformation"))]
    pub profile_and_level_information: Vec<Box<crate::models::MediaEncodingCodecsCommonTypesProfileLevelInformation>>,
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Direction"))]
    pub direction: Direction,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Description"))]
    pub description: String,
    #[serde(rename(deserialize = "FrameworkCodec"))]
    pub framework_codec: String,
    #[serde(rename(deserialize = "IsHardwareCodec"))]
    pub is_hardware_codec: bool,
    #[serde(rename(deserialize = "SecondaryFramework"))]
    pub secondary_framework: SecondaryFramework,
    #[serde(rename(deserialize = "SecondaryFrameworkCodec"))]
    pub secondary_framework_codec: String,
    #[serde(rename(deserialize = "MaxInstanceCount"), skip_serializing_if = "Option::is_none")]
    pub max_instance_count: Option<i32>,
    #[serde(rename(deserialize = "MinBitRate"))]
    pub min_bit_rate: Box<crate::models::MediaEncodingCodecsCommonTypesBitRate>,
    #[serde(rename(deserialize = "MaxBitRate"))]
    pub max_bit_rate: Box<crate::models::MediaEncodingCodecsCommonTypesBitRate>,
    #[serde(rename(deserialize = "IsEnabledByDefault"))]
    pub is_enabled_by_default: bool,
    #[serde(rename(deserialize = "DefaultPriority"))]
    pub default_priority: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CodecKind {
    #[serde(rename = "Audio")]
    Audio,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "SubTitles")]
    SubTitles,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VideoMediaType {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "flv1")]
    Flv1,
    #[serde(rename = "h263")]
    H263,
    #[serde(rename = "h263p")]
    H263p,
    #[serde(rename = "h264")]
    H264,
    #[serde(rename = "hevc")]
    Hevc,
    #[serde(rename = "mjpeg")]
    Mjpeg,
    #[serde(rename = "mpeg1video")]
    Mpeg1video,
    #[serde(rename = "mpeg2video")]
    Mpeg2video,
    #[serde(rename = "mpeg4")]
    Mpeg4,
    #[serde(rename = "msvideo1")]
    Msvideo1,
    #[serde(rename = "theora")]
    Theora,
    #[serde(rename = "vc1image")]
    Vc1image,
    #[serde(rename = "vc1")]
    Vc1,
    #[serde(rename = "vp8")]
    Vp8,
    #[serde(rename = "vp9")]
    Vp9,
    #[serde(rename = "wmv1")]
    Wmv1,
    #[serde(rename = "wmv2")]
    Wmv2,
    #[serde(rename = "wmv3")]
    Wmv3,
    #[serde(rename = "_012v")]
    _012v,
    #[serde(rename = "_4xm")]
    _4xm,
    #[serde(rename = "_8bps")]
    _8bps,
    #[serde(rename = "a64_multi")]
    A64Multi,
    #[serde(rename = "a64_multi5")]
    A64Multi5,
    #[serde(rename = "aasc")]
    Aasc,
    #[serde(rename = "aic")]
    Aic,
    #[serde(rename = "alias_pix")]
    AliasPix,
    #[serde(rename = "amv")]
    Amv,
    #[serde(rename = "anm")]
    Anm,
    #[serde(rename = "ansi")]
    Ansi,
    #[serde(rename = "apng")]
    Apng,
    #[serde(rename = "asv1")]
    Asv1,
    #[serde(rename = "asv2")]
    Asv2,
    #[serde(rename = "aura")]
    Aura,
    #[serde(rename = "aura2")]
    Aura2,
    #[serde(rename = "av1")]
    Av1,
    #[serde(rename = "avrn")]
    Avrn,
    #[serde(rename = "avrp")]
    Avrp,
    #[serde(rename = "avs")]
    Avs,
    #[serde(rename = "avui")]
    Avui,
    #[serde(rename = "ayuv")]
    Ayuv,
    #[serde(rename = "bethsoftvid")]
    Bethsoftvid,
    #[serde(rename = "bfi")]
    Bfi,
    #[serde(rename = "binkvideo")]
    Binkvideo,
    #[serde(rename = "bintext")]
    Bintext,
    #[serde(rename = "bitpacked")]
    Bitpacked,
    #[serde(rename = "bmp")]
    Bmp,
    #[serde(rename = "bmv_video")]
    BmvVideo,
    #[serde(rename = "brender_pix")]
    BrenderPix,
    #[serde(rename = "c93")]
    C93,
    #[serde(rename = "cavs")]
    Cavs,
    #[serde(rename = "cdgraphics")]
    Cdgraphics,
    #[serde(rename = "cdxl")]
    Cdxl,
    #[serde(rename = "cfhd")]
    Cfhd,
    #[serde(rename = "cinepak")]
    Cinepak,
    #[serde(rename = "clearvideo")]
    Clearvideo,
    #[serde(rename = "cljr")]
    Cljr,
    #[serde(rename = "cllc")]
    Cllc,
    #[serde(rename = "cmv")]
    Cmv,
    #[serde(rename = "cpia")]
    Cpia,
    #[serde(rename = "cscd")]
    Cscd,
    #[serde(rename = "cyuv")]
    Cyuv,
    #[serde(rename = "daala")]
    Daala,
    #[serde(rename = "dds")]
    Dds,
    #[serde(rename = "dfa")]
    Dfa,
    #[serde(rename = "dirac")]
    Dirac,
    #[serde(rename = "dnxhd")]
    Dnxhd,
    #[serde(rename = "dpx")]
    Dpx,
    #[serde(rename = "dsicinvideo")]
    Dsicinvideo,
    #[serde(rename = "dvvideo")]
    Dvvideo,
    #[serde(rename = "dxa")]
    Dxa,
    #[serde(rename = "dxtory")]
    Dxtory,
    #[serde(rename = "dxv")]
    Dxv,
    #[serde(rename = "escape124")]
    Escape124,
    #[serde(rename = "escape130")]
    Escape130,
    #[serde(rename = "exr")]
    Exr,
    #[serde(rename = "ffv1")]
    Ffv1,
    #[serde(rename = "ffvhuff")]
    Ffvhuff,
    #[serde(rename = "fic")]
    Fic,
    #[serde(rename = "fits")]
    Fits,
    #[serde(rename = "flashsv")]
    Flashsv,
    #[serde(rename = "flashsv2")]
    Flashsv2,
    #[serde(rename = "flic")]
    Flic,
    #[serde(rename = "fmvc")]
    Fmvc,
    #[serde(rename = "fraps")]
    Fraps,
    #[serde(rename = "frwu")]
    Frwu,
    #[serde(rename = "g2m")]
    G2m,
    #[serde(rename = "gdv")]
    Gdv,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "h261")]
    H261,
    #[serde(rename = "h263i")]
    H263i,
    #[serde(rename = "hap")]
    Hap,
    #[serde(rename = "hnm4video")]
    Hnm4video,
    #[serde(rename = "hq_hqa")]
    HqHqa,
    #[serde(rename = "hqx")]
    Hqx,
    #[serde(rename = "huffyuv")]
    Huffyuv,
    #[serde(rename = "idcin")]
    Idcin,
    #[serde(rename = "idf")]
    Idf,
    #[serde(rename = "iff_ilbm")]
    IffIlbm,
    #[serde(rename = "indeo2")]
    Indeo2,
    #[serde(rename = "indeo3")]
    Indeo3,
    #[serde(rename = "indeo4")]
    Indeo4,
    #[serde(rename = "indeo5")]
    Indeo5,
    #[serde(rename = "interplayvideo")]
    Interplayvideo,
    #[serde(rename = "jpeg2000")]
    Jpeg2000,
    #[serde(rename = "jpegls")]
    Jpegls,
    #[serde(rename = "jv")]
    Jv,
    #[serde(rename = "kgv1")]
    Kgv1,
    #[serde(rename = "kmvc")]
    Kmvc,
    #[serde(rename = "lagarith")]
    Lagarith,
    #[serde(rename = "ljpeg")]
    Ljpeg,
    #[serde(rename = "loco")]
    Loco,
    #[serde(rename = "m101")]
    M101,
    #[serde(rename = "mad")]
    Mad,
    #[serde(rename = "magicyuv")]
    Magicyuv,
    #[serde(rename = "mdec")]
    Mdec,
    #[serde(rename = "mimic")]
    Mimic,
    #[serde(rename = "mjpegb")]
    Mjpegb,
    #[serde(rename = "mmvideo")]
    Mmvideo,
    #[serde(rename = "motionpixels")]
    Motionpixels,
    #[serde(rename = "msa1")]
    Msa1,
    #[serde(rename = "mscc")]
    Mscc,
    #[serde(rename = "msmpeg4v1")]
    Msmpeg4v1,
    #[serde(rename = "msmpeg4v2")]
    Msmpeg4v2,
    #[serde(rename = "msmpeg4v3")]
    Msmpeg4v3,
    #[serde(rename = "msrle")]
    Msrle,
    #[serde(rename = "mss1")]
    Mss1,
    #[serde(rename = "mss2")]
    Mss2,
    #[serde(rename = "mszh")]
    Mszh,
    #[serde(rename = "mts2")]
    Mts2,
    #[serde(rename = "mvc1")]
    Mvc1,
    #[serde(rename = "mvc2")]
    Mvc2,
    #[serde(rename = "mxpeg")]
    Mxpeg,
    #[serde(rename = "nuv")]
    Nuv,
    #[serde(rename = "paf_video")]
    PafVideo,
    #[serde(rename = "pam")]
    Pam,
    #[serde(rename = "pbm")]
    Pbm,
    #[serde(rename = "pcx")]
    Pcx,
    #[serde(rename = "pgm")]
    Pgm,
    #[serde(rename = "pgmyuv")]
    Pgmyuv,
    #[serde(rename = "pictor")]
    Pictor,
    #[serde(rename = "pixlet")]
    Pixlet,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "ppm")]
    Ppm,
    #[serde(rename = "prores")]
    Prores,
    #[serde(rename = "psd")]
    Psd,
    #[serde(rename = "ptx")]
    Ptx,
    #[serde(rename = "qdraw")]
    Qdraw,
    #[serde(rename = "qpeg")]
    Qpeg,
    #[serde(rename = "qtrle")]
    Qtrle,
    #[serde(rename = "r10k")]
    R10k,
    #[serde(rename = "r210")]
    R210,
    #[serde(rename = "rawvideo")]
    Rawvideo,
    #[serde(rename = "rl2")]
    Rl2,
    #[serde(rename = "roq")]
    Roq,
    #[serde(rename = "rpza")]
    Rpza,
    #[serde(rename = "rscc")]
    Rscc,
    #[serde(rename = "rv10")]
    Rv10,
    #[serde(rename = "rv20")]
    Rv20,
    #[serde(rename = "rv30")]
    Rv30,
    #[serde(rename = "rv40")]
    Rv40,
    #[serde(rename = "sanm")]
    Sanm,
    #[serde(rename = "scpr")]
    Scpr,
    #[serde(rename = "screenpresso")]
    Screenpresso,
    #[serde(rename = "sgi")]
    Sgi,
    #[serde(rename = "sgirle")]
    Sgirle,
    #[serde(rename = "sheervideo")]
    Sheervideo,
    #[serde(rename = "smackvideo")]
    Smackvideo,
    #[serde(rename = "smc")]
    Smc,
    #[serde(rename = "smvjpeg")]
    Smvjpeg,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "sp5x")]
    Sp5x,
    #[serde(rename = "speedhq")]
    Speedhq,
    #[serde(rename = "srgc")]
    Srgc,
    #[serde(rename = "sunrast")]
    Sunrast,
    #[serde(rename = "svg")]
    Svg,
    #[serde(rename = "svq1")]
    Svq1,
    #[serde(rename = "svq3")]
    Svq3,
    #[serde(rename = "targa")]
    Targa,
    #[serde(rename = "targa_y216")]
    TargaY216,
    #[serde(rename = "tdsc")]
    Tdsc,
    #[serde(rename = "tgq")]
    Tgq,
    #[serde(rename = "tgv")]
    Tgv,
    #[serde(rename = "thp")]
    Thp,
    #[serde(rename = "tiertexseqvideo")]
    Tiertexseqvideo,
    #[serde(rename = "tiff")]
    Tiff,
    #[serde(rename = "tmv")]
    Tmv,
    #[serde(rename = "tqi")]
    Tqi,
    #[serde(rename = "truemotion1")]
    Truemotion1,
    #[serde(rename = "truemotion2")]
    Truemotion2,
    #[serde(rename = "truemotion2rt")]
    Truemotion2rt,
    #[serde(rename = "tscc")]
    Tscc,
    #[serde(rename = "tscc2")]
    Tscc2,
    #[serde(rename = "txd")]
    Txd,
    #[serde(rename = "ulti")]
    Ulti,
    #[serde(rename = "utvideo")]
    Utvideo,
    #[serde(rename = "v210")]
    V210,
    #[serde(rename = "v210x")]
    V210x,
    #[serde(rename = "v308")]
    V308,
    #[serde(rename = "v408")]
    V408,
    #[serde(rename = "v410")]
    V410,
    #[serde(rename = "vb")]
    Vb,
    #[serde(rename = "vble")]
    Vble,
    #[serde(rename = "vcr1")]
    Vcr1,
    #[serde(rename = "vixl")]
    Vixl,
    #[serde(rename = "vmdvideo")]
    Vmdvideo,
    #[serde(rename = "vmnc")]
    Vmnc,
    #[serde(rename = "vp3")]
    Vp3,
    #[serde(rename = "vp5")]
    Vp5,
    #[serde(rename = "vp6")]
    Vp6,
    #[serde(rename = "vp6a")]
    Vp6a,
    #[serde(rename = "vp6f")]
    Vp6f,
    #[serde(rename = "vp7")]
    Vp7,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "wmv3image")]
    Wmv3image,
    #[serde(rename = "wnv1")]
    Wnv1,
    #[serde(rename = "wrapped_avframe")]
    WrappedAvframe,
    #[serde(rename = "ws_vqa")]
    WsVqa,
    #[serde(rename = "xan_wc3")]
    XanWc3,
    #[serde(rename = "xan_wc4")]
    XanWc4,
    #[serde(rename = "xbin")]
    Xbin,
    #[serde(rename = "xbm")]
    Xbm,
    #[serde(rename = "xface")]
    Xface,
    #[serde(rename = "xpm")]
    Xpm,
    #[serde(rename = "xwd")]
    Xwd,
    #[serde(rename = "y41p")]
    Y41p,
    #[serde(rename = "ylc")]
    Ylc,
    #[serde(rename = "yop")]
    Yop,
    #[serde(rename = "yuv4")]
    Yuv4,
    #[serde(rename = "zerocodec")]
    Zerocodec,
    #[serde(rename = "zlib")]
    Zlib,
    #[serde(rename = "zmbv")]
    Zmbv,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedColorFormats {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "yuv420p")]
    Yuv420p,
    #[serde(rename = "yuyv422")]
    Yuyv422,
    #[serde(rename = "rgb24")]
    Rgb24,
    #[serde(rename = "bgr24")]
    Bgr24,
    #[serde(rename = "yuv422p")]
    Yuv422p,
    #[serde(rename = "yuv444p")]
    Yuv444p,
    #[serde(rename = "yuv410p")]
    Yuv410p,
    #[serde(rename = "yuv411p")]
    Yuv411p,
    #[serde(rename = "gray")]
    Gray,
    #[serde(rename = "monow")]
    Monow,
    #[serde(rename = "monob")]
    Monob,
    #[serde(rename = "pal8")]
    Pal8,
    #[serde(rename = "yuvj420p")]
    Yuvj420p,
    #[serde(rename = "yuvj422p")]
    Yuvj422p,
    #[serde(rename = "yuvj444p")]
    Yuvj444p,
    #[serde(rename = "uyvy422")]
    Uyvy422,
    #[serde(rename = "uyyvyy411")]
    Uyyvyy411,
    #[serde(rename = "bgr8")]
    Bgr8,
    #[serde(rename = "bgr4")]
    Bgr4,
    #[serde(rename = "bgr4_byte")]
    Bgr4Byte,
    #[serde(rename = "rgb8")]
    Rgb8,
    #[serde(rename = "rgb4")]
    Rgb4,
    #[serde(rename = "rgb4_byte")]
    Rgb4Byte,
    #[serde(rename = "nv12")]
    Nv12,
    #[serde(rename = "nv21")]
    Nv21,
    #[serde(rename = "argb")]
    Argb,
    #[serde(rename = "rgba")]
    Rgba,
    #[serde(rename = "abgr")]
    Abgr,
    #[serde(rename = "bgra")]
    Bgra,
    #[serde(rename = "gray16")]
    Gray16,
    #[serde(rename = "yuv440p")]
    Yuv440p,
    #[serde(rename = "yuvj440p")]
    Yuvj440p,
    #[serde(rename = "yuva420p")]
    Yuva420p,
    #[serde(rename = "rgb48")]
    Rgb48,
    #[serde(rename = "rgb565")]
    Rgb565,
    #[serde(rename = "rgb555")]
    Rgb555,
    #[serde(rename = "bgr565")]
    Bgr565,
    #[serde(rename = "bgr555")]
    Bgr555,
    #[serde(rename = "vaapi_moco")]
    VaapiMoco,
    #[serde(rename = "vaapi_idct")]
    VaapiIdct,
    #[serde(rename = "vaapi_vld")]
    VaapiVld,
    #[serde(rename = "yuv420p16")]
    Yuv420p16,
    #[serde(rename = "yuv422p16")]
    Yuv422p16,
    #[serde(rename = "yuv444p16")]
    Yuv444p16,
    #[serde(rename = "dxva2_vld")]
    Dxva2Vld,
    #[serde(rename = "rgb444")]
    Rgb444,
    #[serde(rename = "bgr444")]
    Bgr444,
    #[serde(rename = "ya8")]
    Ya8,
    #[serde(rename = "bgr48")]
    Bgr48,
    #[serde(rename = "yuv420p9")]
    Yuv420p9,
    #[serde(rename = "yuv420p10")]
    Yuv420p10,
    #[serde(rename = "yuv422p10")]
    Yuv422p10,
    #[serde(rename = "yuv444p9")]
    Yuv444p9,
    #[serde(rename = "yuv444p10")]
    Yuv444p10,
    #[serde(rename = "yuv422p9")]
    Yuv422p9,
    #[serde(rename = "gbrp")]
    Gbrp,
    #[serde(rename = "gbrp9")]
    Gbrp9,
    #[serde(rename = "gbrp10")]
    Gbrp10,
    #[serde(rename = "gbrp16")]
    Gbrp16,
    #[serde(rename = "yuva422p")]
    Yuva422p,
    #[serde(rename = "yuva444p")]
    Yuva444p,
    #[serde(rename = "yuva420p9")]
    Yuva420p9,
    #[serde(rename = "yuva422p9")]
    Yuva422p9,
    #[serde(rename = "yuva444p9")]
    Yuva444p9,
    #[serde(rename = "yuva420p10")]
    Yuva420p10,
    #[serde(rename = "yuva422p10")]
    Yuva422p10,
    #[serde(rename = "yuva444p10")]
    Yuva444p10,
    #[serde(rename = "yuva420p16")]
    Yuva420p16,
    #[serde(rename = "yuva422p16")]
    Yuva422p16,
    #[serde(rename = "yuva444p16")]
    Yuva444p16,
    #[serde(rename = "vdpau")]
    Vdpau,
    #[serde(rename = "xyz12")]
    Xyz12,
    #[serde(rename = "nv16")]
    Nv16,
    #[serde(rename = "nv20")]
    Nv20,
    #[serde(rename = "rgba64")]
    Rgba64,
    #[serde(rename = "bgra64")]
    Bgra64,
    #[serde(rename = "yvyu422")]
    Yvyu422,
    #[serde(rename = "ya16")]
    Ya16,
    #[serde(rename = "gbrap")]
    Gbrap,
    #[serde(rename = "gbrap16")]
    Gbrap16,
    #[serde(rename = "qsv")]
    Qsv,
    #[serde(rename = "mmal")]
    Mmal,
    #[serde(rename = "d3d11va_vld")]
    D3d11vaVld,
    #[serde(rename = "cuda")]
    Cuda,
    #[serde(rename = "_0rgb")]
    _0rgb,
    #[serde(rename = "rgb0")]
    Rgb0,
    #[serde(rename = "_0bgr")]
    _0bgr,
    #[serde(rename = "bgr0")]
    Bgr0,
    #[serde(rename = "yuv420p12")]
    Yuv420p12,
    #[serde(rename = "yuv420p14")]
    Yuv420p14,
    #[serde(rename = "yuv422p12")]
    Yuv422p12,
    #[serde(rename = "yuv422p14")]
    Yuv422p14,
    #[serde(rename = "yuv444p12")]
    Yuv444p12,
    #[serde(rename = "yuv444p14")]
    Yuv444p14,
    #[serde(rename = "gbrp12")]
    Gbrp12,
    #[serde(rename = "gbrp14")]
    Gbrp14,
    #[serde(rename = "yuvj411p")]
    Yuvj411p,
    #[serde(rename = "bayer_bggr8")]
    BayerBggr8,
    #[serde(rename = "bayer_rggb8")]
    BayerRggb8,
    #[serde(rename = "bayer_gbrg8")]
    BayerGbrg8,
    #[serde(rename = "bayer_grbg8")]
    BayerGrbg8,
    #[serde(rename = "bayer_bggr16")]
    BayerBggr16,
    #[serde(rename = "bayer_rggb16")]
    BayerRggb16,
    #[serde(rename = "bayer_gbrg16")]
    BayerGbrg16,
    #[serde(rename = "bayer_grbg16")]
    BayerGrbg16,
    #[serde(rename = "xvmc")]
    Xvmc,
    #[serde(rename = "yuv440p10")]
    Yuv440p10,
    #[serde(rename = "yuv440p12")]
    Yuv440p12,
    #[serde(rename = "ayuv64")]
    Ayuv64,
    #[serde(rename = "videotoolbox_vld")]
    VideotoolboxVld,
    #[serde(rename = "p010")]
    P010,
    #[serde(rename = "gbrap12")]
    Gbrap12,
    #[serde(rename = "gbrap10")]
    Gbrap10,
    #[serde(rename = "mediacodec")]
    Mediacodec,
    #[serde(rename = "gray12")]
    Gray12,
    #[serde(rename = "gray10")]
    Gray10,
    #[serde(rename = "p016")]
    P016,
    #[serde(rename = "d3d11")]
    D3d11,
    #[serde(rename = "gray9")]
    Gray9,
    #[serde(rename = "gbrpf32")]
    Gbrpf32,
    #[serde(rename = "gbrapf32")]
    Gbrapf32,
    #[serde(rename = "drm_prime")]
    DrmPrime,
    #[serde(rename = "opencl")]
    Opencl,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "Encoder")]
    Encoder,
    #[serde(rename = "Decoder")]
    Decoder,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecondaryFramework {
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "None")]
    None,
    #[serde(rename = "AmdAmf")]
    AmdAmf,
    #[serde(rename = "MediaCodec")]
    MediaCodec,
    #[serde(rename = "NvEncDec")]
    NvEncDec,
    #[serde(rename = "OpenMax")]
    OpenMax,
    #[serde(rename = "QuickSync")]
    QuickSync,
    #[serde(rename = "VaApi")]
    VaApi,
    #[serde(rename = "V4L2")]
    V4L2,
    #[serde(rename = "DxVa")]
    DxVa,
    #[serde(rename = "D3d11va")]
    D3d11va,
    #[serde(rename = "VideoToolbox")]
    VideoToolbox,
}

