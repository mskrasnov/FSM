/* drm.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Get information about video
//!
//! ## Example
//! ```no-test
//! use ferrix_lib::drm::Video;
//! use ferrix_lib::traits::ToJson;
//!
//! let video = Video::new().unwrap();
//! for dev in &video.devices {
//!     dbg!(dev);
//! }
//! let json = video.to_json().unwrap();
//! dbg!(json);
//! ```
//!
//! ## EDID structure, version 1.4
//!
//! <small>From <a href="https://en.wikipedia.org/wiki/Extended_Display_Identification_Data">WikiPedia</a></small>
//!
//! | Bytes | Description |
//! |-------|-------------|
//! | 0-7 | Fixed header pattern `00 FF FF FF FF FF FF 00` |
//! | 8-9 | Manufacturer ID. "IBM", "PHL" |
//! | 10-11 | Manufacturer product code. 16-bit hex number, little endian. "PHL" + "C0CF" |
//! | 12-15 | Serial number. 32 bits, little-endian |
//! | 16 | Week of manufacture; or `FF` model year flag |
//! | 17 | Year of manufacture, or year or model, if model year flag is set. Year = datavalue + 1990 |
//! | 18 | EDID version, usually `01` (for 1.3 and 1.4) |
//! | 19 | EDID revision, usually `03` (for 1.3) or `04` (for 1.4) |
//! | 20 | Video input parameters bitmap |
//! | 21 | Horizontal screen size, in cm (range 1-255). If vertical screen size is 0, landscape aspect ratio (range 1.00-3.54), datavalue = (ARx100) - 99 (example: 16:9, 79; 4:3, 34.) |
//! | 22 | Vertical screen size, in cm |
//! | 23 | Display gamma, factory default (range 1.00 - 3.54), datavalue = (gamma x 100) - 100 = (gamma - 1) x 100. If 255, gamma is defined by DI-EXT block |
//! | 24 | Supported features bitmap |
//! | ... | ... |
//!
//! **EDID Detailed Timing Descriptor** (TODO)
//!
//! | Bytes | Description                                         |
//! |-------|-----------------------------------------------------|
//! | 0-1 | Pixel clock. `00` - reserved; otherwise in 10 kHz units (0.01 - 655.35 MHz, little-endian) |
//! | 2 | Horizontal active pixels 8 lsbits (0-255)               |
//! | 3 | Horizontal blanking pixels 8 lsbits (0-255)             |
//! | 4 | ...                                                     |
//! | 5 | Vertical active lines 8 lsbits (0-255)                  |
//! | 6 | Vertical blanking lines 8 lsbits (0-255)                |
//! | 7 | ...                                                     |
//! | 8 | Horizontal front porch (sync offset) pixels 8 lsbits (0-255) from blanking start |
//! | 9 | Horizontal sync pulse width pixels 8 lsbits (0-255)     |
//! | 10 | ...                                                    |
//! | 11 | ...                                                    |
//! | 12 | Horizontal image size, mm, 8 lsbits (0-255 mm, 161 in) |
//! | 13 | Vertical image size, mm, ...                           |
//! | ... | ...                                                   |

use crate::traits::ToJson;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{read, read_dir, read_to_string},
    path::Path,
};

/// Information about video devices
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub devices: Vec<DRM>,
}

impl Video {
    pub fn new() -> Result<Self> {
        let prefix = Path::new("/sys/class/drm/");
        let mut devices = vec![];

        for i in 0..=u8::MAX {
            let path = prefix.join(format!("card{i}"));
            if !path.is_dir() {
                continue;
            }
            let dir_contents = read_dir(path)?.filter(|dir| match &dir {
                Ok(dir) => dir.path().is_dir(),
                Err(_) => false,
            });

            for d in dir_contents {
                let d = d?.path(); // {prefix}/{card_i}/{card_i}-*
                let fname = match d.file_name() {
                    Some(fname) => fname.to_str().unwrap_or(""),
                    None => "",
                };
                if d.is_dir() && fname.contains("card") {
                    let drm = DRM::new(d)?;
                    if !drm.is_empty_info() {
                        devices.push(drm);
                    }
                }
            }
        }
        Ok(Self { devices })
    }
}

impl ToJson for Video {}

/// Information about selected display
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DRM {
    /// Is enabled
    pub enabled: bool,

    /// Data from EDID
    pub edid: Option<EDID>,

    /// Supported modes of this screen (in HxV format)
    pub modes: Vec<String>,
}

impl DRM {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let enabled = {
            let txt = read_to_string(path.join("enabled"));
            match txt {
                Ok(txt) => {
                    let contents = txt.trim();
                    if contents == "enabled" { true } else { false }
                }
                Err(_) => false,
            }
        };
        let modes = read_to_string(path.join("modes"))?
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let edid = EDID::new(path);

        Ok(Self {
            enabled,
            edid: match edid {
                Ok(edid) => Some(edid),
                Err(why) => {
                    // может быть, просто вываливать ошибку если не смогли прочитать EDID?
                    if enabled {
                        return Err(why);
                    } else {
                        None
                    }
                }
            },
            modes,
        })
    }

    pub fn is_empty_info(&self) -> bool {
        !self.enabled && self.edid.is_none() && self.modes.is_empty()
    }
}

/// Information from `edid` file (EDID v1.4 only supported yet)
///
/// Read [Wikipedia](https://en.wikipedia.org/wiki/Extended_Display_Identification_Data) for details.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EDID {
    /// Raw EDID bytes (at least 128 bytes)
    pub raw: Vec<u8>,

    /// Manufacturer ID
    ///
    /// This is a legacy Plug and Play ID assigned by the UEFI forum, which is a big-endian
    /// 16-bit value made up three 5-bit letters: 00001 = 'A', 00010 = 'B', etc.
    ///
    /// > Byte offset: 8-9
    pub manufacturer: String,

    /// General alphanumeric description of the display. Extracted from the Monitor Descriptor
    /// block with type `0xFE`
    pub description: Option<String>,

    /// Manufacturer product code. 16-bit hex-nubmer, little-endian.
    /// For example, "LGC" + "C0CF"
    ///
    /// > Byte offset: 10-11
    pub product_code: u16,

    /// Serial number. 32 bits, little-endian
    ///
    /// Note: This may be zero in the serial number is provided as a text string instead.
    ///
    /// > Byte offset: 12-15
    pub serial_number: u32,

    /// Display product name/model
    ///
    /// Extracted from the Monitor Descriptor block with type `0xFC`
    pub model: String,

    /// Text serial number
    ///
    /// Extracted from the Monitor Descriptor block with type `0xFF`
    pub serial: Option<String>,

    /// Week of manufacture; or `FF` model year flag
    ///
    /// > **WARN:** week numbering isn't consistent between manufacturers.
    /// 
    /// > Byte offset: 16
    pub week: u8,

    /// Year of manufacture, or year of model, if model year flag is set
    ///
    /// > Byte offset: 17
    pub year: u16,

    /// EDID version, usually `01` for 1.3 and 1.4
    ///
    /// > Byte offset: 18
    pub edid_version: u8,

    /// EDID revision, usually `03` for 1.3 or `04` for 1.4
    ///
    /// > Byte offset: 19
    pub edid_revision: u8,

    /// Video input parameters (signal type, voltage levels, etc.)
    ///
    /// > Byte offset: 20
    pub video_input: VideoInputParams,

    /// Horizontal screen size, in centimetres (range 1-255). A value of `0` indicates the size
    /// is not specified
    ///
    /// > Byte offset: 21
    pub hscreen_size: u8,

    /// Vertical screen size, in centimetres. A value of `0` indicates the size is not specified
    ///
    /// > Byte offset: 22
    pub vscreen_size: u8,

    /// Calculated screen diagonal in inches, derived from `hscreen_size` and `vscreen_size`
    ///
    /// `None` if either dimension is `0`.
    pub diagonal_inches: Option<f32>,

    /// Calculated aspect ratio of the preferred timing mode (e.g. "16:10", "4:3", etc.
    /// or "1.60:1")
    pub aspect_ratio: Option<String>,

    /// Active horizontal resolution in pixels. Derived from the first Detailed Timing Descriptor
    /// (preferred timing)
    pub resolution_width: Option<u32>,

    /// Active vertical resolution in pixels. Derived from the first DTD (preferred timing)
    pub resolution_height: Option<u32>,

    /// Pixel clock frequency in MHz. Derived from the first DTD (preferred timing)
    pub pixel_clock_mhz: Option<f32>,

    /// Display gamma, factory default
    ///
    /// Formula: `(value + 100) / 100`
    ///
    /// > Byte offset: 23
    pub display_gamma: u8,

    /// List of all parsed DTDs. The first entry is typically the "Preferred Timing" (native
    /// resolution)
    pub detailed_timings: Vec<DetailedTiming>,

    /// Supported vertical/horizontal frequency ranges and maximum pixel clock. Extracted from the
    /// Monitor Range Limits Descriptor (type `0xFD`)
    pub range_limits: Option<RangeLimits>,

    /// Number of 128-byte extension blocks following this base block
    ///
    /// > Byte offset: 126
    pub extension_blocks: u8,

    /// Checksum of the base block. The sum of all 128 bytes should be `0` modulo `256`
    ///
    /// > Byte offset: 127
    pub checksum: u8,
}

impl EDID {
    /// Parses an EDID structure from a `edid` file located in the given directory path
    pub fn new<P: AsRef<Path>>(edid_dir_path: P) -> Result<Self> {
        let path = edid_dir_path.as_ref().join("edid");
        let data = read(&path)
            .map_err(|err| anyhow!("Failed to read EDID file at: {}: {}", path.display(), err))?;

        if data.len() < 128 {
            return Err(anyhow!("EDID data too short ({} bytes)", data.len()));
        }

        if data[0..8] != [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00] {
            return Err(anyhow!("Invalid EDID header on path {}", path.display()));
        }

        let manufacturer = {
            let word = u16::from_be_bytes([data[8], data[9]]);

            let c1 = ((word >> 10) & 0x1F) as u8 + 64;
            let c2 = ((word >> 5) & 0x1F) as u8 + 64;
            let c3 = (word & 0x1F) as u8 + 64;

            format!("{}{}{}", c1 as char, c2 as char, c3 as char)
        };

        let product_code = u16::from_le_bytes([data[10], data[11]]);
        let serial_number = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);

        let week = data[16];
        let year = data[17] as u16 + 1990;
        let edid_version = data[18];
        let edid_revision = data[19];

        let video_input = VideoInputParams::new(&data);
        let hscreen_size = data[21];
        let vscreen_size = data[22];
        let display_gamma = data[23];

        let diagonal_inches = if hscreen_size > 0 && vscreen_size > 0 {
            let diag_cm = ((hscreen_size as f32).powi(2) + (vscreen_size as f32).powi(2)).sqrt();
            Some(diag_cm / 2.54)
        } else {
            None
        };

        let mut model = String::new();
        let mut description = None::<String>;
        let mut serial = None::<String>;
        let mut detailed_timings = Vec::new();
        let mut range_limits = None;

        for i in 0..4 {
            let start = 54 + i * 18;
            let block = &data[start..start + 18];

            if block[0] == 0x00 && block[1] == 0x00 {
                match block[3] {
                    0xFC => model = extract_text(block),
                    0xFE => description = Some(extract_text(block)),
                    0xFF => serial = Some(extract_text(block)),
                    0xFD => range_limits = Some(RangeLimits::parse(block)),
                    _ => {}
                }
            } else {
                detailed_timings.push(DetailedTiming::parse(block));
            }
        }

        let (resolution_width, resolution_height, pixel_clock_mhz, aspect_ratio) = detailed_timings
            .first()
            .map_or((None, None, None, None), |dtd| {
                (
                    Some(dtd.h_active),
                    Some(dtd.v_active),
                    Some(dtd.pixel_clock_hz as f32 / 1_000_000.),
                    Some(dtd.aspect_ratio.clone()),
                )
            });

        let extension_blocks = data[126];
        let checksum = data[127];

        Ok(Self {
            raw: data,

            manufacturer,
            product_code,
            serial_number,
            week,
            year,
            edid_version,
            edid_revision,
            video_input,
            hscreen_size,
            vscreen_size,
            diagonal_inches,
            display_gamma,
            pixel_clock_mhz,
            aspect_ratio,
            resolution_width,
            resolution_height,

            model,
            description,
            serial,

            detailed_timings,
            range_limits,

            extension_blocks,
            checksum,
        })
    }
}

fn extract_text(block: &[u8]) -> String {
    let data = &block[5..18];
    let end = data
        .iter()
        .position(|&b| b == 0x0A || b == 0x00)
        .unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).trim().to_string()
}

/// Single Detailed Timing Descriptor (DTD) block
///
/// A DTD contains precise timing information for a specific display mode, including active
/// resolution, banking intervals, sync pulses, and polarity.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailedTiming {
    /// Pixel clock frequency in Hz
    pub pixel_clock_hz: u64,

    /// Active horizontal resolution in pixels
    pub h_active: u32,

    /// Total horizontal blanking interval in pixels
    pub h_blanking: u32,

    /// Active vertical resolution in lines
    pub v_active: u32,

    /// Total vertical blanking interval in lines
    pub v_blanking: u32,

    /// Horizontal front porch (sync offset) in px
    pub h_front_porch: u32,

    /// Horizontal sync pulse width in px
    pub h_sync_pulse: u32,

    /// Vertical front porch (sync offset) in lines
    pub v_front_porch: u32,

    /// Vertical sync pulse width in lines
    pub v_sync_pulse: u32,

    /// Horizontal back porch in pixels
    pub h_back_porch: u32,

    /// Vertical back porch in lines
    pub v_back_porch: u32,

    /// `true` if the horizontal sync pulse is positive polarity
    pub h_sync_positive: bool,

    /// `true` if the vertical sync pulse is positive polarity
    pub v_sync_positive: bool,

    /// Calculated aspect ratio for this specific timing mode
    pub aspect_ratio: String,
}

impl DetailedTiming {
    /// Parses an 18-byte DTD block into a structured format
    pub fn parse(block: &[u8]) -> Self {
        let pixel_clock_10khz = u16::from_le_bytes([block[0], block[1]]);
        let pixel_clock_hz = pixel_clock_10khz as u64 * 10_000;

        /* Horizontal parameters */
        let h_active_low = block[2] as u32;
        let h_blanking_low = block[3] as u32;
        let h_active_high = ((block[4] >> 4) as u32) << 8;
        let h_blanking_high = ((block[4] & 0x0F) as u32) << 8;

        let h_active = h_active_high | h_active_low;
        let h_blanking = h_blanking_high | h_blanking_low;

        /* Vertical parameters */
        let v_active_low = block[5] as u32;
        let v_blanking_low = block[6] as u32;
        let v_active_high = ((block[7] >> 4) as u32) << 8;
        let v_blanking_high = ((block[7] & 0x0F) as u32) << 8;

        let v_active = v_active_high | v_active_low;
        let v_blanking = v_blanking_high | v_blanking_low;

        /* Sync */
        let h_sync_offset_low = (block[8] >> 4) as u32;
        let h_sync_pulse_low = (block[8] & 0x0F) as u32;
        let v_sync_offset_low = (block[9] >> 4) as u32;
        let v_sync_pulse_low = (block[9] & 0x0F) as u32;

        let h_sync_offset_high = ((block[11] >> 2) & 0x03) as u32;
        let h_sync_pulse_high = (block[11] & 0x03) as u32;
        let v_sync_offset_high = ((block[11] >> 6) & 0x03) as u32;
        let v_sync_pulse_high = ((block[11] >> 4) & 0x03) as u32;

        let h_front_porch = (h_sync_offset_high << 4) | h_sync_offset_low;
        let h_sync_pulse = (h_sync_pulse_high << 4) | h_sync_pulse_low;
        let v_front_porch = (v_sync_offset_high << 4) | v_sync_offset_low;
        let v_sync_pulse = (v_sync_pulse_high << 4) | v_sync_pulse_low;

        /* Back porch */
        // blanking - front_porch - sync_pulse
        let h_back_porch = h_blanking.saturating_sub(h_front_porch + h_sync_pulse);
        let v_back_porch = v_blanking.saturating_sub(v_front_porch + v_sync_pulse);

        let h_sync_positive = (block[17] & 0x02) != 0;
        let v_sync_positive = (block[17] & 0x04) != 0;

        let aspect_ratio = calc_aspect_ratio(h_active, v_active);

        Self {
            pixel_clock_hz,
            h_active,
            h_blanking,
            v_active,
            v_blanking,
            h_front_porch,
            h_sync_pulse,
            v_front_porch,
            v_sync_pulse,
            h_back_porch,
            v_back_porch,
            h_sync_positive,
            v_sync_positive,
            aspect_ratio,
        }
    }
}

fn calc_aspect_ratio(width: u32, height: u32) -> String {
    if width == 0 || height == 0 {
        return "??:??".to_string();
    }

    let ratio = width as f64 / height as f64;

    if (ratio - 2.3333).abs() < 0.05 {
        "21:9".to_string()
    } else if (ratio - 1.7777).abs() < 0.05 {
        "16:9".to_string()
    } else if (ratio - 1.6).abs() < 0.05 {
        "16:10".to_string()
    } else if (ratio - 1.5).abs() < 0.05 {
        "3:2".to_string()
    } else if (ratio - 1.3333).abs() < 0.05 {
        "4:3".to_string()
    } else if (ratio - 1.25).abs() < 0.05 {
        "5:4".to_string()
    } else {
        format!("{ratio:.2}:1")
    }
}

/// Monitor Range Limits Descriptor (type `0xFD`)
///
/// Defines the operational limits of the display to help the graphics driver select
/// valid timing modes withoyt reading the EDID string
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RangeLimits {
    /// Minimum vertical field rate (refresh rate) in Hz
    pub min_v_freq_hz: u8,

    /// Maximum vertical field rate (refresh rate) in Hz
    pub max_v_freq_hz: u8,

    /// Minimum horizontal line rate in kHz
    pub min_h_freq_khz: u8,

    /// Maximum horizontal line rate in kHz
    pub max_h_freq_khz: u8,

    /// Maximum supported pixel clock in MHz (stored as tens of MHz in EDID, multiplied
    /// bu `10` here)
    pub max_pixel_clock_mhz: u16,
}

impl RangeLimits {
    /// Parses an 18-byte MRL Descriptor block
    pub fn parse(block: &[u8]) -> Self {
        Self {
            min_v_freq_hz: block[5],
            max_v_freq_hz: block[6],
            min_h_freq_khz: block[7],
            max_h_freq_khz: block[8],
            max_pixel_clock_mhz: block[9] as u16 * 10,
        }
    }
}

/// Video input parameters bitmap
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VideoInputParams {
    Digital(VideoInputParamsDigital),
    Analog(VideoInputParamsAnalog),
}

impl VideoInputParams {
    pub fn new(data: &[u8]) -> Self {
        let d = data[20];
        let bit_depth = ((d >> 7) & 0b00000111) as u8;
        if bit_depth == 1 {
            Self::Digital(VideoInputParamsDigital::new(data))
        } else if bit_depth == 0 {
            Self::Analog(VideoInputParamsAnalog::new(data))
        } else {
            panic!("Unknown 7 bit of 20 byte ({bit_depth})!")
        }
    }
}

/// Digital input
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInputParamsDigital {
    /// Bit depth
    pub bit_depth: BitDepth,

    /// Video interface type
    pub video_interface: VideoInterface,
}

impl VideoInputParamsDigital {
    pub fn new(data: &[u8]) -> Self {
        let d = data[20];
        let bit_depth = BitDepth::from(((d >> 4) & 0b00000111) as u8);
        let video_interface = VideoInterface::from((d & 0b00000111) as u8);

        Self {
            bit_depth,
            video_interface,
        }
    }
}

/// Bit depth
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BitDepth {
    Undefined,

    /// 6 bits per color
    B6,

    /// 8 bits per color
    B8,

    /// 10 bits per color
    B10,

    /// 12 bits per color
    B12,

    /// 14 bits per color
    B14,

    /// 16 bits per color
    B16,

    /// Reserved value
    Reserved,

    /// Unknown value (while EDID parsing)
    Unknown(u8),
}

impl From<u8> for BitDepth {
    fn from(value: u8) -> Self {
        match value {
            0b000 => Self::Undefined,
            0b001 => Self::B6,
            0b010 => Self::B8,
            0b011 => Self::B10,
            0b100 => Self::B12,
            0b101 => Self::B14,
            0b110 => Self::B16,
            0b111 => Self::Reserved,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for BitDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Undefined => "Undefined".to_string(),
                Self::B6 => "6 bits".to_string(),
                Self::B8 => "8 bits".to_string(),
                Self::B10 => "10 bits".to_string(),
                Self::B12 => "12 bits".to_string(),
                Self::B14 => "14 bits".to_string(),
                Self::B16 => "16 bits".to_string(),
                Self::Reserved => "Reserved value".to_string(),
                Self::Unknown(val) => format!("Unknown ({val})"),
            }
        )
    }
}

/// Video interface (EDID data may be incorrect)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VideoInterface {
    Undefined,
    DVI,
    HDMIa,
    HDMIb,
    MDDI,
    DisplayPort,
    Unknown(u8),
}

impl From<u8> for VideoInterface {
    fn from(value: u8) -> Self {
        match value {
            0b0000 => Self::Undefined,
            0b0001 => Self::DVI,
            0b0010 => Self::HDMIa,
            0b0011 => Self::HDMIb,
            0b0100 => Self::MDDI,
            0b0101 => Self::DisplayPort,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for VideoInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Undefined => "Undefined".to_string(),
                Self::DVI => "DVI".to_string(),
                Self::HDMIa => "HDMI-a".to_string(),
                Self::HDMIb => "HDMI-b".to_string(),
                Self::MDDI => "MDDI".to_string(),
                Self::DisplayPort => "Display Port".to_string(),
                Self::Unknown(val) => format!("Unknown (code: {val})"),
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInputParamsAnalog {
    /// Video white and sync levels, relative to blank:
    ///
    /// | Binary value | Data    |
    /// |--------------|---------|
    /// | `00` | +0.7/-0.3 V     |
    /// | `01` | +0.714/-0.286 V |
    /// | `10` | +1.0/-0.4 V     |
    /// | `11` | +0.7/0 V (EVC)  |
    pub white_sync_levels: u8,

    /// Blank-to-black setyp (pedestal) expected
    pub blank_to_black_setup: u8,

    /// Separate sync supported
    pub separate_sync_supported: u8,

    /// Composite sync supported
    pub composite_sync_supported: u8,

    /// Sync on green supported
    pub sync_on_green_supported: u8,

    /// VSync pulse must be serrated when composite or sync-on-green
    /// is used
    pub sync_on_green_isused: u8,
}

impl VideoInputParamsAnalog {
    /// NOTE: THIS FUNCTION MAY BE INCORRECT
    pub fn new(data: &[u8]) -> Self {
        let d = data[20];
        let white_sync_levels = ((d >> 5) & 0b00000011) as u8;
        let blank_to_black_setup = (d >> 4) as u8;
        let separate_sync_supported = (d >> 3) as u8;
        let composite_sync_supported = (d >> 2) as u8;
        let sync_on_green_supported = (d >> 1) as u8;
        let sync_on_green_isused = (d >> 0) as u8; // WARN: may be incorrect

        Self {
            white_sync_levels,
            blank_to_black_setup,
            separate_sync_supported,
            composite_sync_supported,
            sync_on_green_supported,
            sync_on_green_isused,
        }
    }
}
