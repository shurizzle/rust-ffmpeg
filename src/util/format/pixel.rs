use std::error;
use std::ffi::{CStr, CString, NulError};
use std::fmt;
use std::str::{from_utf8_unchecked, FromStr};

use ffi::AVPixelFormat::*;
use ffi::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Pixel {
    None,

    YUV420P,
    YUYV422,
    RGB24,
    BGR24,
    YUV422P,
    YUV444P,
    YUV410P,
    YUV411P,
    GRAY8,
    MonoWhite,
    MonoBlack,
    PAL8,
    YUVJ420P,
    YUVJ422P,
    YUVJ444P,
    UYVY422,
    UYYVYY411,
    BGR8,
    BGR4,
    BGR4_BYTE,
    RGB8,
    RGB4,
    RGB4_BYTE,
    NV12,
    NV21,

    ARGB,
    RGBA,
    ABGR,
    BGRA,

    GRAY16BE,
    GRAY16LE,
    YUV440P,
    YUVJ440P,
    YUVA420P,
    RGB48BE,
    RGB48LE,

    RGB565BE,
    RGB565LE,
    RGB555BE,
    RGB555LE,

    BGR565BE,
    BGR565LE,
    BGR555BE,
    BGR555LE,

    #[cfg(feature = "ff_api_vaapi")]
    VAAPI_MOCO,
    #[cfg(feature = "ff_api_vaapi")]
    VAAPI_IDCT,
    #[cfg(feature = "ff_api_vaapi")]
    VAAPI_VLD,

    VAAPI,

    YUV420P16LE,
    YUV420P16BE,
    YUV422P16LE,
    YUV422P16BE,
    YUV444P16LE,
    YUV444P16BE,
    DXVA2_VLD,

    RGB444LE,
    RGB444BE,
    BGR444LE,
    BGR444BE,
    YA8,

    Y400A,
    GRAY8A,

    BGR48BE,
    BGR48LE,

    YUV420P9BE,
    YUV420P9LE,
    YUV420P10BE,
    YUV420P10LE,
    YUV422P10BE,
    YUV422P10LE,
    YUV444P9BE,
    YUV444P9LE,
    YUV444P10BE,
    YUV444P10LE,
    YUV422P9BE,
    YUV422P9LE,
    GBRP,
    GBR24P,
    GBRP9BE,
    GBRP9LE,
    GBRP10BE,
    GBRP10LE,
    GBRP16BE,
    GBRP16LE,
    YUVA422P,
    YUVA444P,
    YUVA420P9BE,
    YUVA420P9LE,
    YUVA422P9BE,
    YUVA422P9LE,
    YUVA444P9BE,
    YUVA444P9LE,
    YUVA420P10BE,
    YUVA420P10LE,
    YUVA422P10BE,
    YUVA422P10LE,
    YUVA444P10BE,
    YUVA444P10LE,
    YUVA420P16BE,
    YUVA420P16LE,
    YUVA422P16BE,
    YUVA422P16LE,
    YUVA444P16BE,
    YUVA444P16LE,

    VDPAU,

    XYZ12LE,
    XYZ12BE,
    NV16,
    NV20LE,
    NV20BE,

    RGBA64BE,
    RGBA64LE,
    BGRA64BE,
    BGRA64LE,

    YVYU422,

    YA16BE,
    YA16LE,

    GBRAP,
    GBRAP16BE,
    GBRAP16LE,
    QSV,
    MMAL,

    D3D11VA_VLD,

    CUDA,

    ZRGB,
    RGBZ,
    ZBGR,
    BGRZ,

    YUV420P12BE,
    YUV420P12LE,
    YUV420P14BE,
    YUV420P14LE,
    YUV422P12BE,
    YUV422P12LE,
    YUV422P14BE,
    YUV422P14LE,
    YUV444P12BE,
    YUV444P12LE,
    YUV444P14BE,
    YUV444P14LE,
    GBRP12BE,
    GBRP12LE,
    GBRP14BE,
    GBRP14LE,
    YUVJ411P,

    BAYER_BGGR8,
    BAYER_RGGB8,
    BAYER_GBRG8,
    BAYER_GRBG8,
    BAYER_BGGR16LE,
    BAYER_BGGR16BE,
    BAYER_RGGB16LE,
    BAYER_RGGB16BE,
    BAYER_GBRG16LE,
    BAYER_GBRG16BE,
    BAYER_GRBG16LE,
    BAYER_GRBG16BE,

    XVMC,

    YUV440P10LE,
    YUV440P10BE,
    YUV440P12LE,
    YUV440P12BE,
    AYUV64LE,
    AYUV64BE,

    VIDEOTOOLBOX,

    P010LE,
    P010BE,

    GBRAP12BE,
    GBRAP12LE,

    GBRAP10BE,
    GBRAP10LE,

    MEDIACODEC,

    GRAY12BE,
    GRAY12LE,
    GRAY10BE,
    GRAY10LE,

    P016LE,
    P016BE,

    D3D11,

    GRAY9BE,
    GRAY9LE,

    GBRPF32BE,
    GBRPF32LE,
    GBRAPF32BE,
    GBRAPF32LE,

    DRM_PRIME,
    OPENCL,

    RGB32,
    RGB32_1,
    BGR32,
    BGR32_1,
    ZRGB32,
    ZBGR32,

    GRAY9,
    GRAY10,
    GRAY12,
    GRAY16,
    YA16,
    RGB48,
    RGB565,
    RGB555,
    RGB444,
    RGBA64,
    BGR48,
    BGR565,
    BGR555,
    BGR444,
    BGRA64,

    YUV420P9,
    YUV422P9,
    YUV444P9,
    YUV420P10,
    YUV422P10,
    YUV440P10,
    YUV444P10,
    YUV420P12,
    YUV422P12,
    YUV440P12,
    YUV444P12,
    YUV420P14,
    YUV422P14,
    YUV444P14,
    YUV420P16,
    YUV422P16,
    YUV444P16,

    GBRP9,
    GBRP10,
    GBRP12,
    GBRP14,
    GBRP16,
    GBRAP10,
    GBRAP12,
    GBRAP16,

    BAYER_BGGR16,
    BAYER_RGGB16,
    BAYER_GBRG16,
    BAYER_GRBG16,

    GBRPF32,
    GBRAPF32,

    YUVA420P9,
    YUVA422P9,
    YUVA444P9,
    YUVA420P10,
    YUVA422P10,
    YUVA444P10,
    YUVA420P16,
    YUVA422P16,
    YUVA444P16,

    XYZ12,
    NV20,
    AYUV64,
    P010,
    P016
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Descriptor {
    ptr: *const AVPixFmtDescriptor,
}

unsafe impl Send for Descriptor {}
unsafe impl Sync for Descriptor {}

impl Pixel {
    pub fn descriptor(self) -> Option<Descriptor> {
        unsafe {
            let ptr = av_pix_fmt_desc_get(self.into());

            ptr.as_ref().map(|ptr| Descriptor { ptr })
        }
    }
}

impl Descriptor {
    pub fn as_ptr(self) -> *const AVPixFmtDescriptor {
        self.ptr
    }

    pub fn name(self) -> &'static str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
    }

    pub fn nb_components(self) -> u8 {
        unsafe { (*self.as_ptr()).nb_components }
    }

    pub fn log2_chroma_w(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_w }
    }

    pub fn log2_chroma_h(self) -> u8 {
        unsafe { (*self.as_ptr()).log2_chroma_h }
    }
}

impl From<AVPixelFormat> for Pixel {
    #[inline]
    fn from(value: AVPixelFormat) -> Self {
        match value {
            AV_PIX_FMT_NONE => Pixel::None,
            AV_PIX_FMT_YUV420P => Pixel::YUV420P,
            AV_PIX_FMT_YUYV422 => Pixel::YUYV422,
            AV_PIX_FMT_RGB24 => Pixel::RGB24,
            AV_PIX_FMT_BGR24 => Pixel::BGR24,
            AV_PIX_FMT_YUV422P => Pixel::YUV422P,
            AV_PIX_FMT_YUV444P => Pixel::YUV444P,
            AV_PIX_FMT_YUV410P => Pixel::YUV410P,
            AV_PIX_FMT_YUV411P => Pixel::YUV411P,
            AV_PIX_FMT_GRAY8 => Pixel::GRAY8,
            AV_PIX_FMT_MONOWHITE => Pixel::MonoWhite,
            AV_PIX_FMT_MONOBLACK => Pixel::MonoBlack,
            AV_PIX_FMT_PAL8 => Pixel::PAL8,
            AV_PIX_FMT_YUVJ420P => Pixel::YUVJ420P,
            AV_PIX_FMT_YUVJ422P => Pixel::YUVJ422P,
            AV_PIX_FMT_YUVJ444P => Pixel::YUVJ444P,
            AV_PIX_FMT_UYVY422 => Pixel::UYVY422,
            AV_PIX_FMT_UYYVYY411 => Pixel::UYYVYY411,
            AV_PIX_FMT_BGR8 => Pixel::BGR8,
            AV_PIX_FMT_BGR4 => Pixel::BGR4,
            AV_PIX_FMT_BGR4_BYTE => Pixel::BGR4_BYTE,
            AV_PIX_FMT_RGB8 => Pixel::RGB8,
            AV_PIX_FMT_RGB4 => Pixel::RGB4,
            AV_PIX_FMT_RGB4_BYTE => Pixel::RGB4_BYTE,
            AV_PIX_FMT_NV12 => Pixel::NV12,
            AV_PIX_FMT_NV21 => Pixel::NV21,

            AV_PIX_FMT_ARGB => Pixel::ARGB,
            AV_PIX_FMT_RGBA => Pixel::RGBA,
            AV_PIX_FMT_ABGR => Pixel::ABGR,
            AV_PIX_FMT_BGRA => Pixel::BGRA,

            AV_PIX_FMT_GRAY16BE => Pixel::GRAY16BE,
            AV_PIX_FMT_GRAY16LE => Pixel::GRAY16LE,
            AV_PIX_FMT_YUV440P => Pixel::YUV440P,
            AV_PIX_FMT_YUVJ440P => Pixel::YUVJ440P,
            AV_PIX_FMT_YUVA420P => Pixel::YUVA420P,
            AV_PIX_FMT_RGB48BE => Pixel::RGB48BE,
            AV_PIX_FMT_RGB48LE => Pixel::RGB48LE,

            AV_PIX_FMT_RGB565BE => Pixel::RGB565BE,
            AV_PIX_FMT_RGB565LE => Pixel::RGB565LE,
            AV_PIX_FMT_RGB555BE => Pixel::RGB555BE,
            AV_PIX_FMT_RGB555LE => Pixel::RGB555LE,

            AV_PIX_FMT_BGR565BE => Pixel::BGR565BE,
            AV_PIX_FMT_BGR565LE => Pixel::BGR565LE,
            AV_PIX_FMT_BGR555BE => Pixel::BGR555BE,
            AV_PIX_FMT_BGR555LE => Pixel::BGR555LE,

            #[cfg(feature = "ff_api_vaapi")]
            AV_PIX_FMT_VAAPI_MOCO => Pixel::VAAPI_MOCO,
            #[cfg(feature = "ff_api_vaapi")]
            AV_PIX_FMT_VAAPI_IDCT => Pixel::VAAPI_IDCT,
            #[cfg(feature = "ff_api_vaapi")]
            AV_PIX_FMT_VAAPI_VLD => Pixel::VAAPI_VLD,

            #[cfg(not(feature = "ff_api_vaapi"))]
            AV_PIX_FMT_VAAPI => Pixel::VAAPI,

            AV_PIX_FMT_YUV420P16LE => Pixel::YUV420P16LE,
            AV_PIX_FMT_YUV420P16BE => Pixel::YUV420P16BE,
            AV_PIX_FMT_YUV422P16LE => Pixel::YUV422P16LE,
            AV_PIX_FMT_YUV422P16BE => Pixel::YUV422P16BE,
            AV_PIX_FMT_YUV444P16LE => Pixel::YUV444P16LE,
            AV_PIX_FMT_YUV444P16BE => Pixel::YUV444P16BE,
            AV_PIX_FMT_DXVA2_VLD => Pixel::DXVA2_VLD,

            AV_PIX_FMT_RGB444LE => Pixel::RGB444LE,
            AV_PIX_FMT_RGB444BE => Pixel::RGB444BE,
            AV_PIX_FMT_BGR444LE => Pixel::BGR444LE,
            AV_PIX_FMT_BGR444BE => Pixel::BGR444BE,
            AV_PIX_FMT_YA8 => Pixel::YA8,

            AV_PIX_FMT_BGR48BE => Pixel::BGR48BE,
            AV_PIX_FMT_BGR48LE => Pixel::BGR48LE,

            AV_PIX_FMT_YUV420P9BE => Pixel::YUV420P9BE,
            AV_PIX_FMT_YUV420P9LE => Pixel::YUV420P9LE,
            AV_PIX_FMT_YUV420P10BE => Pixel::YUV420P10BE,
            AV_PIX_FMT_YUV420P10LE => Pixel::YUV420P10LE,
            AV_PIX_FMT_YUV422P10BE => Pixel::YUV422P10BE,
            AV_PIX_FMT_YUV422P10LE => Pixel::YUV422P10LE,
            AV_PIX_FMT_YUV444P9BE => Pixel::YUV444P9BE,
            AV_PIX_FMT_YUV444P9LE => Pixel::YUV444P9LE,
            AV_PIX_FMT_YUV444P10BE => Pixel::YUV444P10BE,
            AV_PIX_FMT_YUV444P10LE => Pixel::YUV444P10LE,
            AV_PIX_FMT_YUV422P9BE => Pixel::YUV422P9BE,
            AV_PIX_FMT_YUV422P9LE => Pixel::YUV422P9LE,
            AV_PIX_FMT_GBRP => Pixel::GBRP,
            AV_PIX_FMT_GBRP9BE => Pixel::GBRP9BE,
            AV_PIX_FMT_GBRP9LE => Pixel::GBRP9LE,
            AV_PIX_FMT_GBRP10BE => Pixel::GBRP10BE,
            AV_PIX_FMT_GBRP10LE => Pixel::GBRP10LE,
            AV_PIX_FMT_GBRP16BE => Pixel::GBRP16BE,
            AV_PIX_FMT_GBRP16LE => Pixel::GBRP16LE,
            AV_PIX_FMT_YUVA422P => Pixel::YUVA422P,
            AV_PIX_FMT_YUVA444P => Pixel::YUVA444P,
            AV_PIX_FMT_YUVA420P9BE => Pixel::YUVA420P9BE,
            AV_PIX_FMT_YUVA420P9LE => Pixel::YUVA420P9LE,
            AV_PIX_FMT_YUVA422P9BE => Pixel::YUVA422P9BE,
            AV_PIX_FMT_YUVA422P9LE => Pixel::YUVA422P9LE,
            AV_PIX_FMT_YUVA444P9BE => Pixel::YUVA444P9BE,
            AV_PIX_FMT_YUVA444P9LE => Pixel::YUVA444P9LE,
            AV_PIX_FMT_YUVA420P10BE => Pixel::YUVA420P10BE,
            AV_PIX_FMT_YUVA420P10LE => Pixel::YUVA420P10LE,
            AV_PIX_FMT_YUVA422P10BE => Pixel::YUVA422P10BE,
            AV_PIX_FMT_YUVA422P10LE => Pixel::YUVA422P10LE,
            AV_PIX_FMT_YUVA444P10BE => Pixel::YUVA444P10BE,
            AV_PIX_FMT_YUVA444P10LE => Pixel::YUVA444P10LE,
            AV_PIX_FMT_YUVA420P16BE => Pixel::YUVA420P16BE,
            AV_PIX_FMT_YUVA420P16LE => Pixel::YUVA420P16LE,
            AV_PIX_FMT_YUVA422P16BE => Pixel::YUVA422P16BE,
            AV_PIX_FMT_YUVA422P16LE => Pixel::YUVA422P16LE,
            AV_PIX_FMT_YUVA444P16BE => Pixel::YUVA444P16BE,
            AV_PIX_FMT_YUVA444P16LE => Pixel::YUVA444P16LE,

            AV_PIX_FMT_VDPAU => Pixel::VDPAU,

            AV_PIX_FMT_XYZ12LE => Pixel::XYZ12LE,
            AV_PIX_FMT_XYZ12BE => Pixel::XYZ12BE,
            AV_PIX_FMT_NV16 => Pixel::NV16,
            AV_PIX_FMT_NV20LE => Pixel::NV20LE,
            AV_PIX_FMT_NV20BE => Pixel::NV20BE,

            AV_PIX_FMT_RGBA64BE => Pixel::RGBA64BE,
            AV_PIX_FMT_RGBA64LE => Pixel::RGBA64LE,
            AV_PIX_FMT_BGRA64BE => Pixel::BGRA64BE,
            AV_PIX_FMT_BGRA64LE => Pixel::BGRA64LE,

            AV_PIX_FMT_YVYU422 => Pixel::YVYU422,

            AV_PIX_FMT_YA16BE => Pixel::YA16BE,
            AV_PIX_FMT_YA16LE => Pixel::YA16LE,

            AV_PIX_FMT_GBRAP => Pixel::GBRAP,
            AV_PIX_FMT_GBRAP16BE => Pixel::GBRAP16BE,
            AV_PIX_FMT_GBRAP16LE => Pixel::GBRAP16LE,
            AV_PIX_FMT_QSV => Pixel::QSV,
            AV_PIX_FMT_MMAL => Pixel::MMAL,

            AV_PIX_FMT_D3D11VA_VLD => Pixel::D3D11VA_VLD,

            AV_PIX_FMT_CUDA => Pixel::CUDA,

            AV_PIX_FMT_0RGB => Pixel::ZRGB,
            AV_PIX_FMT_RGB0 => Pixel::RGBZ,
            AV_PIX_FMT_0BGR => Pixel::ZBGR,
            AV_PIX_FMT_BGR0 => Pixel::BGRZ,

            AV_PIX_FMT_YUV420P12BE => Pixel::YUV420P12BE,
            AV_PIX_FMT_YUV420P12LE => Pixel::YUV420P12LE,
            AV_PIX_FMT_YUV420P14BE => Pixel::YUV420P14BE,
            AV_PIX_FMT_YUV420P14LE => Pixel::YUV420P14LE,
            AV_PIX_FMT_YUV422P12BE => Pixel::YUV422P12BE,
            AV_PIX_FMT_YUV422P12LE => Pixel::YUV422P12LE,
            AV_PIX_FMT_YUV422P14BE => Pixel::YUV422P14BE,
            AV_PIX_FMT_YUV422P14LE => Pixel::YUV422P14LE,
            AV_PIX_FMT_YUV444P12BE => Pixel::YUV444P12BE,
            AV_PIX_FMT_YUV444P12LE => Pixel::YUV444P12LE,
            AV_PIX_FMT_YUV444P14BE => Pixel::YUV444P14BE,
            AV_PIX_FMT_YUV444P14LE => Pixel::YUV444P14LE,
            AV_PIX_FMT_GBRP12BE => Pixel::GBRP12BE,
            AV_PIX_FMT_GBRP12LE => Pixel::GBRP12LE,
            AV_PIX_FMT_GBRP14BE => Pixel::GBRP14BE,
            AV_PIX_FMT_GBRP14LE => Pixel::GBRP14LE,
            AV_PIX_FMT_YUVJ411P => Pixel::YUVJ411P,

            AV_PIX_FMT_BAYER_BGGR8 => Pixel::BAYER_BGGR8,
            AV_PIX_FMT_BAYER_RGGB8 => Pixel::BAYER_RGGB8,
            AV_PIX_FMT_BAYER_GBRG8 => Pixel::BAYER_GBRG8,
            AV_PIX_FMT_BAYER_GRBG8 => Pixel::BAYER_GRBG8,
            AV_PIX_FMT_BAYER_BGGR16LE => Pixel::BAYER_BGGR16LE,
            AV_PIX_FMT_BAYER_BGGR16BE => Pixel::BAYER_BGGR16BE,
            AV_PIX_FMT_BAYER_RGGB16LE => Pixel::BAYER_RGGB16LE,
            AV_PIX_FMT_BAYER_RGGB16BE => Pixel::BAYER_RGGB16BE,
            AV_PIX_FMT_BAYER_GBRG16LE => Pixel::BAYER_GBRG16LE,
            AV_PIX_FMT_BAYER_GBRG16BE => Pixel::BAYER_GBRG16BE,
            AV_PIX_FMT_BAYER_GRBG16LE => Pixel::BAYER_GRBG16LE,
            AV_PIX_FMT_BAYER_GRBG16BE => Pixel::BAYER_GRBG16BE,

            AV_PIX_FMT_XVMC => Pixel::XVMC,

            AV_PIX_FMT_YUV440P10LE => Pixel::YUV440P10LE,
            AV_PIX_FMT_YUV440P10BE => Pixel::YUV440P10BE,
            AV_PIX_FMT_YUV440P12LE => Pixel::YUV440P12LE,
            AV_PIX_FMT_YUV440P12BE => Pixel::YUV440P12BE,
            AV_PIX_FMT_AYUV64LE => Pixel::AYUV64LE,
            AV_PIX_FMT_AYUV64BE => Pixel::AYUV64BE,

            AV_PIX_FMT_VIDEOTOOLBOX => Pixel::VIDEOTOOLBOX,

            AV_PIX_FMT_P010LE => Pixel::P010LE,
            AV_PIX_FMT_P010BE => Pixel::P010BE,

            AV_PIX_FMT_GBRAP12BE => Pixel::GBRAP12BE,
            AV_PIX_FMT_GBRAP12LE => Pixel::GBRAP12LE,

            AV_PIX_FMT_GBRAP10BE => Pixel::GBRAP10BE,
            AV_PIX_FMT_GBRAP10LE => Pixel::GBRAP10LE,

            AV_PIX_FMT_MEDIACODEC => Pixel::MEDIACODEC,

            AV_PIX_FMT_GRAY12BE => Pixel::GRAY12BE,
            AV_PIX_FMT_GRAY12LE => Pixel::GRAY12LE,
            AV_PIX_FMT_GRAY10BE => Pixel::GRAY10BE,
            AV_PIX_FMT_GRAY10LE => Pixel::GRAY10LE,

            AV_PIX_FMT_P016LE => Pixel::P016LE,
            AV_PIX_FMT_P016BE => Pixel::P016BE,

            AV_PIX_FMT_D3D11 => Pixel::D3D11,

            AV_PIX_FMT_GRAY9BE => Pixel::GRAY9BE,
            AV_PIX_FMT_GRAY9LE => Pixel::GRAY9LE,

            AV_PIX_FMT_GBRPF32BE => Pixel::GBRPF32BE,
            AV_PIX_FMT_GBRPF32LE => Pixel::GBRPF32LE,
            AV_PIX_FMT_GBRAPF32BE => Pixel::GBRAPF32BE,
            AV_PIX_FMT_GBRAPF32LE => Pixel::GBRAPF32LE,

            AV_PIX_FMT_DRM_PRIME => Pixel::DRM_PRIME,
            AV_PIX_FMT_OPENCL => Pixel::OPENCL,

            AV_PIX_FMT_NB => Pixel::None,
        }
    }
}

impl Into<AVPixelFormat> for Pixel {
    #[inline]
    fn into(self) -> AVPixelFormat {
        match self {
            Pixel::None => AV_PIX_FMT_NONE,
            Pixel::YUV420P => AV_PIX_FMT_YUV420P,
            Pixel::YUYV422 => AV_PIX_FMT_YUYV422,
            Pixel::RGB24 => AV_PIX_FMT_RGB24,
            Pixel::BGR24 => AV_PIX_FMT_BGR24,
            Pixel::YUV422P => AV_PIX_FMT_YUV422P,
            Pixel::YUV444P => AV_PIX_FMT_YUV444P,
            Pixel::YUV410P => AV_PIX_FMT_YUV410P,
            Pixel::YUV411P => AV_PIX_FMT_YUV411P,
            Pixel::GRAY8 => AV_PIX_FMT_GRAY8,
            Pixel::MonoWhite => AV_PIX_FMT_MONOWHITE,
            Pixel::MonoBlack => AV_PIX_FMT_MONOBLACK,
            Pixel::PAL8 => AV_PIX_FMT_PAL8,
            Pixel::YUVJ420P => AV_PIX_FMT_YUVJ420P,
            Pixel::YUVJ422P => AV_PIX_FMT_YUVJ422P,
            Pixel::YUVJ444P => AV_PIX_FMT_YUVJ444P,
            Pixel::UYVY422 => AV_PIX_FMT_UYVY422,
            Pixel::UYYVYY411 => AV_PIX_FMT_UYYVYY411,
            Pixel::BGR8 => AV_PIX_FMT_BGR8,
            Pixel::BGR4 => AV_PIX_FMT_BGR4,
            Pixel::BGR4_BYTE => AV_PIX_FMT_BGR4_BYTE,
            Pixel::RGB8 => AV_PIX_FMT_RGB8,
            Pixel::RGB4 => AV_PIX_FMT_RGB4,
            Pixel::RGB4_BYTE => AV_PIX_FMT_RGB4_BYTE,
            Pixel::NV12 => AV_PIX_FMT_NV12,
            Pixel::NV21 => AV_PIX_FMT_NV21,

            Pixel::ARGB => AV_PIX_FMT_ARGB,
            Pixel::RGBA => AV_PIX_FMT_RGBA,
            Pixel::ABGR => AV_PIX_FMT_ABGR,
            Pixel::BGRA => AV_PIX_FMT_BGRA,

            Pixel::GRAY16BE => AV_PIX_FMT_GRAY16BE,
            Pixel::GRAY16LE => AV_PIX_FMT_GRAY16LE,
            Pixel::YUV440P => AV_PIX_FMT_YUV440P,
            Pixel::YUVJ440P => AV_PIX_FMT_YUVJ440P,
            Pixel::YUVA420P => AV_PIX_FMT_YUVA420P,
            Pixel::RGB48BE => AV_PIX_FMT_RGB48BE,
            Pixel::RGB48LE => AV_PIX_FMT_RGB48LE,

            Pixel::RGB565BE => AV_PIX_FMT_RGB565BE,
            Pixel::RGB565LE => AV_PIX_FMT_RGB565LE,
            Pixel::RGB555BE => AV_PIX_FMT_RGB555BE,
            Pixel::RGB555LE => AV_PIX_FMT_RGB555LE,

            Pixel::BGR565BE => AV_PIX_FMT_BGR565BE,
            Pixel::BGR565LE => AV_PIX_FMT_BGR565LE,
            Pixel::BGR555BE => AV_PIX_FMT_BGR555BE,
            Pixel::BGR555LE => AV_PIX_FMT_BGR555LE,

            #[cfg(feature = "ff_api_vaapi")]
            Pixel::VAAPI_MOCO => AV_PIX_FMT_VAAPI_MOCO,
            #[cfg(feature = "ff_api_vaapi")]
            Pixel::VAAPI_IDCT => AV_PIX_FMT_VAAPI_IDCT,
            #[cfg(feature = "ff_api_vaapi")]
            Pixel::VAAPI_VLD => AV_PIX_FMT_VAAPI_VLD,

            Pixel::VAAPI => AV_PIX_FMT_VAAPI,

            Pixel::YUV420P16LE => AV_PIX_FMT_YUV420P16LE,
            Pixel::YUV420P16BE => AV_PIX_FMT_YUV420P16BE,
            Pixel::YUV422P16LE => AV_PIX_FMT_YUV422P16LE,
            Pixel::YUV422P16BE => AV_PIX_FMT_YUV422P16BE,
            Pixel::YUV444P16LE => AV_PIX_FMT_YUV444P16LE,
            Pixel::YUV444P16BE => AV_PIX_FMT_YUV444P16BE,
            Pixel::DXVA2_VLD => AV_PIX_FMT_DXVA2_VLD,

            Pixel::RGB444LE => AV_PIX_FMT_RGB444LE,
            Pixel::RGB444BE => AV_PIX_FMT_RGB444BE,
            Pixel::BGR444LE => AV_PIX_FMT_BGR444LE,
            Pixel::BGR444BE => AV_PIX_FMT_BGR444BE,
            Pixel::YA8 => AV_PIX_FMT_YA8,

            Pixel::Y400A => AV_PIX_FMT_Y400A,
            Pixel::GRAY8A => AV_PIX_FMT_GRAY8A,

            Pixel::BGR48BE => AV_PIX_FMT_BGR48BE,
            Pixel::BGR48LE => AV_PIX_FMT_BGR48LE,

            Pixel::YUV420P9BE => AV_PIX_FMT_YUV420P9BE,
            Pixel::YUV420P9LE => AV_PIX_FMT_YUV420P9LE,
            Pixel::YUV420P10BE => AV_PIX_FMT_YUV420P10BE,
            Pixel::YUV420P10LE => AV_PIX_FMT_YUV420P10LE,
            Pixel::YUV422P10BE => AV_PIX_FMT_YUV422P10BE,
            Pixel::YUV422P10LE => AV_PIX_FMT_YUV422P10LE,
            Pixel::YUV444P9BE => AV_PIX_FMT_YUV444P9BE,
            Pixel::YUV444P9LE => AV_PIX_FMT_YUV444P9LE,
            Pixel::YUV444P10BE => AV_PIX_FMT_YUV444P10BE,
            Pixel::YUV444P10LE => AV_PIX_FMT_YUV444P10LE,
            Pixel::YUV422P9BE => AV_PIX_FMT_YUV422P9BE,
            Pixel::YUV422P9LE => AV_PIX_FMT_YUV422P9LE,
            Pixel::GBRP => AV_PIX_FMT_GBRP,
            Pixel::GBR24P => AV_PIX_FMT_GBR24P,
            Pixel::GBRP9BE => AV_PIX_FMT_GBRP9BE,
            Pixel::GBRP9LE => AV_PIX_FMT_GBRP9LE,
            Pixel::GBRP10BE => AV_PIX_FMT_GBRP10BE,
            Pixel::GBRP10LE => AV_PIX_FMT_GBRP10LE,
            Pixel::GBRP16BE => AV_PIX_FMT_GBRP16BE,
            Pixel::GBRP16LE => AV_PIX_FMT_GBRP16LE,
            Pixel::YUVA422P => AV_PIX_FMT_YUVA422P,
            Pixel::YUVA444P => AV_PIX_FMT_YUVA444P,
            Pixel::YUVA420P9BE => AV_PIX_FMT_YUVA420P9BE,
            Pixel::YUVA420P9LE => AV_PIX_FMT_YUVA420P9LE,
            Pixel::YUVA422P9BE => AV_PIX_FMT_YUVA422P9BE,
            Pixel::YUVA422P9LE => AV_PIX_FMT_YUVA422P9LE,
            Pixel::YUVA444P9BE => AV_PIX_FMT_YUVA444P9BE,
            Pixel::YUVA444P9LE => AV_PIX_FMT_YUVA444P9LE,
            Pixel::YUVA420P10BE => AV_PIX_FMT_YUVA420P10BE,
            Pixel::YUVA420P10LE => AV_PIX_FMT_YUVA420P10LE,
            Pixel::YUVA422P10BE => AV_PIX_FMT_YUVA422P10BE,
            Pixel::YUVA422P10LE => AV_PIX_FMT_YUVA422P10LE,
            Pixel::YUVA444P10BE => AV_PIX_FMT_YUVA444P10BE,
            Pixel::YUVA444P10LE => AV_PIX_FMT_YUVA444P10LE,
            Pixel::YUVA420P16BE => AV_PIX_FMT_YUVA420P16BE,
            Pixel::YUVA420P16LE => AV_PIX_FMT_YUVA420P16LE,
            Pixel::YUVA422P16BE => AV_PIX_FMT_YUVA422P16BE,
            Pixel::YUVA422P16LE => AV_PIX_FMT_YUVA422P16LE,
            Pixel::YUVA444P16BE => AV_PIX_FMT_YUVA444P16BE,
            Pixel::YUVA444P16LE => AV_PIX_FMT_YUVA444P16LE,

            Pixel::VDPAU => AV_PIX_FMT_VDPAU,

            Pixel::XYZ12LE => AV_PIX_FMT_XYZ12LE,
            Pixel::XYZ12BE => AV_PIX_FMT_XYZ12BE,
            Pixel::NV16 => AV_PIX_FMT_NV16,
            Pixel::NV20LE => AV_PIX_FMT_NV20LE,
            Pixel::NV20BE => AV_PIX_FMT_NV20BE,

            Pixel::RGBA64BE => AV_PIX_FMT_RGBA64BE,
            Pixel::RGBA64LE => AV_PIX_FMT_RGBA64LE,
            Pixel::BGRA64BE => AV_PIX_FMT_BGRA64BE,
            Pixel::BGRA64LE => AV_PIX_FMT_BGRA64LE,

            Pixel::YVYU422 => AV_PIX_FMT_YVYU422,

            Pixel::YA16BE => AV_PIX_FMT_YA16BE,
            Pixel::YA16LE => AV_PIX_FMT_YA16LE,

            Pixel::GBRAP => AV_PIX_FMT_GBRAP,
            Pixel::GBRAP16BE => AV_PIX_FMT_GBRAP16BE,
            Pixel::GBRAP16LE => AV_PIX_FMT_GBRAP16LE,
            Pixel::QSV => AV_PIX_FMT_QSV,
            Pixel::MMAL => AV_PIX_FMT_MMAL,

            Pixel::D3D11VA_VLD => AV_PIX_FMT_D3D11VA_VLD,

            Pixel::CUDA => AV_PIX_FMT_CUDA,

            Pixel::ZRGB => AV_PIX_FMT_0RGB,
            Pixel::RGBZ => AV_PIX_FMT_RGB0,
            Pixel::ZBGR => AV_PIX_FMT_0BGR,
            Pixel::BGRZ => AV_PIX_FMT_BGR0,

            Pixel::YUV420P12BE => AV_PIX_FMT_YUV420P12BE,
            Pixel::YUV420P12LE => AV_PIX_FMT_YUV420P12LE,
            Pixel::YUV420P14BE => AV_PIX_FMT_YUV420P14BE,
            Pixel::YUV420P14LE => AV_PIX_FMT_YUV420P14LE,
            Pixel::YUV422P12BE => AV_PIX_FMT_YUV422P12BE,
            Pixel::YUV422P12LE => AV_PIX_FMT_YUV422P12LE,
            Pixel::YUV422P14BE => AV_PIX_FMT_YUV422P14BE,
            Pixel::YUV422P14LE => AV_PIX_FMT_YUV422P14LE,
            Pixel::YUV444P12BE => AV_PIX_FMT_YUV444P12BE,
            Pixel::YUV444P12LE => AV_PIX_FMT_YUV444P12LE,
            Pixel::YUV444P14BE => AV_PIX_FMT_YUV444P14BE,
            Pixel::YUV444P14LE => AV_PIX_FMT_YUV444P14LE,
            Pixel::GBRP12BE => AV_PIX_FMT_GBRP12BE,
            Pixel::GBRP12LE => AV_PIX_FMT_GBRP12LE,
            Pixel::GBRP14BE => AV_PIX_FMT_GBRP14BE,
            Pixel::GBRP14LE => AV_PIX_FMT_GBRP14LE,
            Pixel::YUVJ411P => AV_PIX_FMT_YUVJ411P,

            Pixel::BAYER_BGGR8 => AV_PIX_FMT_BAYER_BGGR8,
            Pixel::BAYER_RGGB8 => AV_PIX_FMT_BAYER_RGGB8,
            Pixel::BAYER_GBRG8 => AV_PIX_FMT_BAYER_GBRG8,
            Pixel::BAYER_GRBG8 => AV_PIX_FMT_BAYER_GRBG8,
            Pixel::BAYER_BGGR16LE => AV_PIX_FMT_BAYER_BGGR16LE,
            Pixel::BAYER_BGGR16BE => AV_PIX_FMT_BAYER_BGGR16BE,
            Pixel::BAYER_RGGB16LE => AV_PIX_FMT_BAYER_RGGB16LE,
            Pixel::BAYER_RGGB16BE => AV_PIX_FMT_BAYER_RGGB16BE,
            Pixel::BAYER_GBRG16LE => AV_PIX_FMT_BAYER_GBRG16LE,
            Pixel::BAYER_GBRG16BE => AV_PIX_FMT_BAYER_GBRG16BE,
            Pixel::BAYER_GRBG16LE => AV_PIX_FMT_BAYER_GRBG16LE,
            Pixel::BAYER_GRBG16BE => AV_PIX_FMT_BAYER_GRBG16BE,

            Pixel::XVMC => AV_PIX_FMT_XVMC,

            Pixel::YUV440P10LE => AV_PIX_FMT_YUV440P10LE,
            Pixel::YUV440P10BE => AV_PIX_FMT_YUV440P10BE,
            Pixel::YUV440P12LE => AV_PIX_FMT_YUV440P12LE,
            Pixel::YUV440P12BE => AV_PIX_FMT_YUV440P12BE,
            Pixel::AYUV64LE => AV_PIX_FMT_AYUV64LE,
            Pixel::AYUV64BE => AV_PIX_FMT_AYUV64BE,

            Pixel::VIDEOTOOLBOX => AV_PIX_FMT_VIDEOTOOLBOX,

            Pixel::P010LE => AV_PIX_FMT_P010LE,
            Pixel::P010BE => AV_PIX_FMT_P010BE,

            Pixel::GBRAP12BE => AV_PIX_FMT_GBRAP12BE,
            Pixel::GBRAP12LE => AV_PIX_FMT_GBRAP12LE,

            Pixel::GBRAP10BE => AV_PIX_FMT_GBRAP10BE,
            Pixel::GBRAP10LE => AV_PIX_FMT_GBRAP10LE,

            Pixel::MEDIACODEC => AV_PIX_FMT_MEDIACODEC,

            Pixel::GRAY12BE => AV_PIX_FMT_GRAY12BE,
            Pixel::GRAY12LE => AV_PIX_FMT_GRAY12LE,
            Pixel::GRAY10BE => AV_PIX_FMT_GRAY10BE,
            Pixel::GRAY10LE => AV_PIX_FMT_GRAY10LE,

            Pixel::P016LE => AV_PIX_FMT_P016LE,
            Pixel::P016BE => AV_PIX_FMT_P016BE,

            Pixel::D3D11 => AV_PIX_FMT_D3D11,

            Pixel::GRAY9BE => AV_PIX_FMT_GRAY9BE,
            Pixel::GRAY9LE => AV_PIX_FMT_GRAY9LE,

            Pixel::GBRPF32BE => AV_PIX_FMT_GBRPF32BE,
            Pixel::GBRPF32LE => AV_PIX_FMT_GBRPF32LE,
            Pixel::GBRAPF32BE => AV_PIX_FMT_GBRAPF32BE,
            Pixel::GBRAPF32LE => AV_PIX_FMT_GBRAPF32LE,

            Pixel::DRM_PRIME => AV_PIX_FMT_DRM_PRIME,
            Pixel::OPENCL => AV_PIX_FMT_OPENCL,

            #[cfg(target_endian = "big")]
            Pixel::RGB32 => AV_PIX_FMT_ARGB,
            #[cfg(target_endian = "big")]
            Pixel::RGB32_1 => AV_PIX_FMT_RGBA,
            #[cfg(target_endian = "big")]
            Pixel::BGR32 => AV_PIX_FMT_ABGR,
            #[cfg(target_endian = "big")]
            Pixel::BGR32_1 => AV_PIX_FMT_BGRA,
            #[cfg(target_endian = "big")]
            Pixel::ZRGB32 => AV_PIX_FMT_0RGB,
            #[cfg(target_endian = "big")]
            Pixel::ZBGR32 => AV_PIX_FMT_0BGR,

            #[cfg(target_endian = "big")]
            Pixel::GRAY9 => AV_PIX_FMT_GRAY9BE,
            #[cfg(target_endian = "big")]
            Pixel::GRAY10 => AV_PIX_FMT_GRAY10BE,
            #[cfg(target_endian = "big")]
            Pixel::GRAY12 => AV_PIX_FMT_GRAY12BE,
            #[cfg(target_endian = "big")]
            Pixel::GRAY16 => AV_PIX_FMT_GRAY16BE,
            #[cfg(target_endian = "big")]
            Pixel::YA16 => AV_PIX_FMT_YA16BE,
            #[cfg(target_endian = "big")]
            Pixel::RGB48 => AV_PIX_FMT_RGB48BE,
            #[cfg(target_endian = "big")]
            Pixel::RGB565 => AV_PIX_FMT_RGB565BE,
            #[cfg(target_endian = "big")]
            Pixel::RGB555 => AV_PIX_FMT_RGB555BE,
            #[cfg(target_endian = "big")]
            Pixel::RGB444 => AV_PIX_FMT_RGB444BE,
            #[cfg(target_endian = "big")]
            Pixel::RGBA64 => AV_PIX_FMT_RGBA64BE,
            #[cfg(target_endian = "big")]
            Pixel::BGR48 => AV_PIX_FMT_BGR48BE,
            #[cfg(target_endian = "big")]
            Pixel::BGR565 => AV_PIX_FMT_BGR565BE,
            #[cfg(target_endian = "big")]
            Pixel::BGR555 => AV_PIX_FMT_BGR555BE,
            #[cfg(target_endian = "big")]
            Pixel::BGR444 => AV_PIX_FMT_BGR444BE,
            #[cfg(target_endian = "big")]
            Pixel::BGRA64 => AV_PIX_FMT_BGRA64BE,

            #[cfg(target_endian = "big")]
            Pixel::YUV420P9 => AV_PIX_FMT_YUV420P9BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV422P9 => AV_PIX_FMT_YUV422P9BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV444P9 => AV_PIX_FMT_YUV444P9BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV420P10 => AV_PIX_FMT_YUV420P10BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV422P10 => AV_PIX_FMT_YUV422P10BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV440P10 => AV_PIX_FMT_YUV440P10BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV444P10 => AV_PIX_FMT_YUV444P10BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV420P12 => AV_PIX_FMT_YUV420P12BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV422P12 => AV_PIX_FMT_YUV422P12BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV440P12 => AV_PIX_FMT_YUV440P12BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV444P12 => AV_PIX_FMT_YUV444P12BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV420P14 => AV_PIX_FMT_YUV420P14BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV422P14 => AV_PIX_FMT_YUV422P14BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV444P14 => AV_PIX_FMT_YUV444P14BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV420P16 => AV_PIX_FMT_YUV420P16BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV422P16 => AV_PIX_FMT_YUV422P16BE,
            #[cfg(target_endian = "big")]
            Pixel::YUV444P16 => AV_PIX_FMT_YUV444P16BE,

            #[cfg(target_endian = "big")]
            Pixel::GBRP9 => AV_PIX_FMT_GBRP9BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRP10 => AV_PIX_FMT_GBRP10BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRP12 => AV_PIX_FMT_GBRP12BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRP14 => AV_PIX_FMT_GBRP14BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRP16 => AV_PIX_FMT_GBRP16BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRAP10 => AV_PIX_FMT_GBRAP10BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRAP12 => AV_PIX_FMT_GBRAP12BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRAP16 => AV_PIX_FMT_GBRAP16BE,

            #[cfg(target_endian = "big")]
            Pixel::BAYER_BGGR16 => AV_PIX_FMT_BAYER_BGGR16BE,
            #[cfg(target_endian = "big")]
            Pixel::BAYER_RGGB16 => AV_PIX_FMT_BAYER_RGGB16BE,
            #[cfg(target_endian = "big")]
            Pixel::BAYER_GBRG16 => AV_PIX_FMT_BAYER_GBRG16BE,
            #[cfg(target_endian = "big")]
            Pixel::BAYER_GRBG16 => AV_PIX_FMT_BAYER_GRBG16BE,

            #[cfg(target_endian = "big")]
            Pixel::GBRPF32 => AV_PIX_FMT_GBRPF32BE,
            #[cfg(target_endian = "big")]
            Pixel::GBRAPF32 => AV_PIX_FMT_GBRAPF32BE,

            #[cfg(target_endian = "big")]
            Pixel::YUVA420P9 => AV_PIX_FMT_YUVA420P9BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA422P9 => AV_PIX_FMT_YUVA422P9BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA444P9 => AV_PIX_FMT_YUVA444P9BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA420P10 => AV_PIX_FMT_YUVA420P10BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA422P10 => AV_PIX_FMT_YUVA422P10BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA444P10 => AV_PIX_FMT_YUVA444P10BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA420P16 => AV_PIX_FMT_YUVA420P16BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA422P16 => AV_PIX_FMT_YUVA422P16BE,
            #[cfg(target_endian = "big")]
            Pixel::YUVA444P16 => AV_PIX_FMT_YUVA444P16BE,

            #[cfg(target_endian = "big")]
            Pixel::XYZ12 => AV_PIX_FMT_XYZ12BE,
            #[cfg(target_endian = "big")]
            Pixel::NV20 => AV_PIX_FMT_NV20BE,
            #[cfg(target_endian = "big")]
            Pixel::AYUV64 => AV_PIX_FMT_AYUV64BE,
            #[cfg(target_endian = "big")]
            Pixel::P010 => AV_PIX_FMT_P010BE,
            #[cfg(target_endian = "big")]
            Pixel::P016 => AV_PIX_FMT_P016BE,

            #[cfg(target_endian = "little")]
            Pixel::RGB32 => AV_PIX_FMT_BGRA,
            #[cfg(target_endian = "little")]
            Pixel::RGB32_1 => AV_PIX_FMT_ABGR,
            #[cfg(target_endian = "little")]
            Pixel::BGR32 => AV_PIX_FMT_RGBA,
            #[cfg(target_endian = "little")]
            Pixel::BGR32_1 => AV_PIX_FMT_ARGB,
            #[cfg(target_endian = "little")]
            Pixel::ZRGB32 => AV_PIX_FMT_BGR0,
            #[cfg(target_endian = "little")]
            Pixel::ZBGR32 => AV_PIX_FMT_RGB0,

            #[cfg(target_endian = "little")]
            Pixel::GRAY9 => AV_PIX_FMT_GRAY9LE,
            #[cfg(target_endian = "little")]
            Pixel::GRAY10 => AV_PIX_FMT_GRAY10LE,
            #[cfg(target_endian = "little")]
            Pixel::GRAY12 => AV_PIX_FMT_GRAY12LE,
            #[cfg(target_endian = "little")]
            Pixel::GRAY16 => AV_PIX_FMT_GRAY16LE,
            #[cfg(target_endian = "little")]
            Pixel::YA16 => AV_PIX_FMT_YA16LE,
            #[cfg(target_endian = "little")]
            Pixel::RGB48 => AV_PIX_FMT_RGB48LE,
            #[cfg(target_endian = "little")]
            Pixel::RGB565 => AV_PIX_FMT_RGB565LE,
            #[cfg(target_endian = "little")]
            Pixel::RGB555 => AV_PIX_FMT_RGB555LE,
            #[cfg(target_endian = "little")]
            Pixel::RGB444 => AV_PIX_FMT_RGB444LE,
            #[cfg(target_endian = "little")]
            Pixel::RGBA64 => AV_PIX_FMT_RGBA64LE,
            #[cfg(target_endian = "little")]
            Pixel::BGR48 => AV_PIX_FMT_BGR48LE,
            #[cfg(target_endian = "little")]
            Pixel::BGR565 => AV_PIX_FMT_BGR565LE,
            #[cfg(target_endian = "little")]
            Pixel::BGR555 => AV_PIX_FMT_BGR555LE,
            #[cfg(target_endian = "little")]
            Pixel::BGR444 => AV_PIX_FMT_BGR444LE,
            #[cfg(target_endian = "little")]
            Pixel::BGRA64 => AV_PIX_FMT_BGRA64LE,

            #[cfg(target_endian = "little")]
            Pixel::YUV420P9 => AV_PIX_FMT_YUV420P9LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV422P9 => AV_PIX_FMT_YUV422P9LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV444P9 => AV_PIX_FMT_YUV444P9LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV420P10 => AV_PIX_FMT_YUV420P10LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV422P10 => AV_PIX_FMT_YUV422P10LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV440P10 => AV_PIX_FMT_YUV440P10LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV444P10 => AV_PIX_FMT_YUV444P10LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV420P12 => AV_PIX_FMT_YUV420P12LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV422P12 => AV_PIX_FMT_YUV422P12LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV440P12 => AV_PIX_FMT_YUV440P12LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV444P12 => AV_PIX_FMT_YUV444P12LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV420P14 => AV_PIX_FMT_YUV420P14LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV422P14 => AV_PIX_FMT_YUV422P14LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV444P14 => AV_PIX_FMT_YUV444P14LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV420P16 => AV_PIX_FMT_YUV420P16LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV422P16 => AV_PIX_FMT_YUV422P16LE,
            #[cfg(target_endian = "little")]
            Pixel::YUV444P16 => AV_PIX_FMT_YUV444P16LE,

            #[cfg(target_endian = "little")]
            Pixel::GBRP9 => AV_PIX_FMT_GBRP9LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRP10 => AV_PIX_FMT_GBRP10LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRP12 => AV_PIX_FMT_GBRP12LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRP14 => AV_PIX_FMT_GBRP14LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRP16 => AV_PIX_FMT_GBRP16LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRAP10 => AV_PIX_FMT_GBRAP10LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRAP12 => AV_PIX_FMT_GBRAP12LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRAP16 => AV_PIX_FMT_GBRAP16LE,

            #[cfg(target_endian = "little")]
            Pixel::BAYER_BGGR16 => AV_PIX_FMT_BAYER_BGGR16LE,
            #[cfg(target_endian = "little")]
            Pixel::BAYER_RGGB16 => AV_PIX_FMT_BAYER_RGGB16LE,
            #[cfg(target_endian = "little")]
            Pixel::BAYER_GBRG16 => AV_PIX_FMT_BAYER_GBRG16LE,
            #[cfg(target_endian = "little")]
            Pixel::BAYER_GRBG16 => AV_PIX_FMT_BAYER_GRBG16LE,

            #[cfg(target_endian = "little")]
            Pixel::GBRPF32 => AV_PIX_FMT_GBRPF32LE,
            #[cfg(target_endian = "little")]
            Pixel::GBRAPF32 => AV_PIX_FMT_GBRAPF32LE,

            #[cfg(target_endian = "little")]
            Pixel::YUVA420P9 => AV_PIX_FMT_YUVA420P9LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA422P9 => AV_PIX_FMT_YUVA422P9LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA444P9 => AV_PIX_FMT_YUVA444P9LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA420P10 => AV_PIX_FMT_YUVA420P10LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA422P10 => AV_PIX_FMT_YUVA422P10LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA444P10 => AV_PIX_FMT_YUVA444P10LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA420P16 => AV_PIX_FMT_YUVA420P16LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA422P16 => AV_PIX_FMT_YUVA422P16LE,
            #[cfg(target_endian = "little")]
            Pixel::YUVA444P16 => AV_PIX_FMT_YUVA444P16LE,

            #[cfg(target_endian = "little")]
            Pixel::XYZ12 => AV_PIX_FMT_XYZ12LE,
            #[cfg(target_endian = "little")]
            Pixel::NV20 => AV_PIX_FMT_NV20LE,
            #[cfg(target_endian = "little")]
            Pixel::AYUV64 => AV_PIX_FMT_AYUV64LE,
            #[cfg(target_endian = "little")]
            Pixel::P010 => AV_PIX_FMT_P010LE,
            #[cfg(target_endian = "little")]
            Pixel::P016 => AV_PIX_FMT_P016LE
        }
    }
}

#[derive(Debug)]
pub enum ParsePixelError {
    NulError(NulError),
    UnknownFormat,
}

impl fmt::Display for ParsePixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsePixelError::NulError(ref e) => e.fmt(f),
            ParsePixelError::UnknownFormat => write!(f, "unknown pixel format"),
        }
    }
}

impl error::Error for ParsePixelError {
    fn description(&self) -> &str {
        match *self {
            ParsePixelError::NulError(ref e) => e.description(),
            ParsePixelError::UnknownFormat => "unknown pixel format",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ParsePixelError::NulError(ref e) => Some(e),
            ParsePixelError::UnknownFormat => None,
        }
    }
}

impl From<NulError> for ParsePixelError {
    fn from(x: NulError) -> ParsePixelError {
        ParsePixelError::NulError(x)
    }
}

impl FromStr for Pixel {
    type Err = ParsePixelError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Pixel, ParsePixelError> {
        let cstring = CString::new(s)?;
        let format = unsafe { av_get_pix_fmt(cstring.as_ptr()) }.into();

        if format == Pixel::None {
            Err(ParsePixelError::UnknownFormat)
        } else {
            Ok(format)
        }
    }
}
