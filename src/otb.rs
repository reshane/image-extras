//! OTB (Over The air Bitmap) Format is an obsolete image format from the 1990s.
//!
//! # Related Links
//! * <https://en.wikipedia.org/wiki/Wireless_Application_Protocol_Bitmap_Format> - The WBMP format on Wikipedia
//! * <https://www.wapforum.org/what/technical/SPEC-WAESpec-19990524.pdf> - The WAP Specification

use std::io::{BufRead, Seek, Write};

use image::error::{
    DecodingError, EncodingError, ImageFormatHint, UnsupportedError, UnsupportedErrorKind,
};
use image::{ColorType, ExtendedColorType, ImageDecoder, ImageEncoder, ImageError, ImageResult};

/// Encoder for Otb images.
pub struct OtbEncoder<'a, W> {
    writer: &'a mut W,
    threshold: u8,
}

impl<'a, W: Write> OtbEncoder<'a, W> {
    pub fn new(writer: &'a mut W) -> Result<OtbEncoder<'a, W>, ImageError> {
        Ok(OtbEncoder {
            writer,
            threshold: 127_u8,
        })
    }

    pub fn with_threshold(mut self, threshold: u8) -> OtbEncoder<'a, W> {
        self.threshold = threshold;
        self
    }
}

impl<'a, W: Write> ImageEncoder for OtbEncoder<'a, W> {
    fn write_image(
        mut self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: ExtendedColorType,
    ) -> std::result::Result<(), ImageError> {
        todo!()
    }
}

/// Decoder for Otb images.
pub struct OtbDecoder<R>
where
    R: BufRead + Seek,
{
    reader: R,
    dimensions: (u32, u32),
    out_color: Option<ExtendedColorType>,
}

impl<R> OtbDecoder<R>
where
    R: BufRead + Seek,
{
    /// Create a new `OtbDecoder`.
    pub fn new(reader: R) -> Result<OtbDecoder<R>, ImageError> {
        let mut decoder = Self::new_decoder(reader);
        decoder.read_metadata()?;
        Ok(decoder)
    }

    fn new_decoder(reader: R) -> OtbDecoder<R> {
        Self {
            reader,
            dimensions: (0, 0),
            out_color: None,
        }
    }

    fn read_metadata(&mut self) -> Result<(), ImageError> {
        // InfoField - 00 for single byte width/height values
        let info_field_buf: &mut [u8; 1] = &mut [0; 1];
        self.reader.read_exact(info_field_buf)?;
        let info_field = info_field_buf[0];
        if info_field != 0 {
            dbg!(format!("{:0x}", info_field));
            todo!();
        }
        // Width
        let width_buf: &mut [u8; 1] = &mut [0; 1];
        self.reader.read_exact(width_buf)?;
        let width = width_buf[0];
        if width == 0 {
            dbg!(width);
            todo!();
        }
        // Height
        let height_buf: &mut [u8; 1] = &mut [0; 1];
        self.reader.read_exact(height_buf)?;
        let height = height_buf[0];
        if height == 0 {
            dbg!(height);
            todo!();
        }
        // Depth
        let depth_buf: &mut [u8; 1] = &mut [0; 1];
        self.reader.read_exact(depth_buf)?;
        let depth = depth_buf[0];
        if depth != 1 {
            dbg!(depth);
            todo!();
        }

        self.dimensions = (width as u32, height as u32);

        dbg!(self.dimensions);

        Ok(())
    }
}

impl<R: BufRead + Seek> ImageDecoder for OtbDecoder<R> {
    fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    fn color_type(&self) -> ColorType {
        ColorType::L8
    }

    fn original_color_type(&self) -> ExtendedColorType {
        ExtendedColorType::L1
    }

    fn read_image(mut self, buf: &mut [u8]) -> ImageResult<()> {
        if (buf.len() as u32) < (self.dimensions.0 * self.dimensions.1) {
            todo!();
        }

        let mut byte_buf = Vec::<u8>::with_capacity(buf.len() / 8);
        let _ = self.reader.read_to_end(&mut byte_buf)?;
        dbg!((self.dimensions.0 * self.dimensions.1) / 8);
        dbg!(self.dimensions.0 * self.dimensions.1);

        let mut buf_idx = 0;
        let mut bytes_idx = 0;
        while buf_idx < buf.len() && bytes_idx < byte_buf.len() {
            let byte = byte_buf[bytes_idx];
            'inner: for shft in (0..7).rev() {
                buf[buf_idx] = if (byte >> shft) & 1 == 0 {
                    0x00
                } else {
                    0xFF
                };
                buf_idx += 1;
                if buf_idx >= buf.len() {
                    break 'inner;
                }
            }
            bytes_idx += 1;
        }
        Ok(())
    }

    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> ImageResult<()> {
        (*self).read_image(buf)?;
        Ok(())
    }
}
