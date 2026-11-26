//! Reads a dropped audio file for Auto Loudness: WAV (PCM 16, 24 and 32 bit, float 32 and 64,
//! WAVE_FORMAT_EXTENSIBLE) and AIFF / AIFC (big-endian PCM, `sowt` little-endian, `fl32`, `fl64`).
//! These are what a host renders when a clip is dragged out of it (Pro Tools renders a clip to WAV
//! or AIFF when it is dropped on a plug-in window). Mono files are read as both channels; files
//! with more channels give their first two.

pub struct Audio {
    pub sample_rate: f64,
    pub left: Vec<f32>,
    pub right: Vec<f32>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Format {
    Int { bits: u16, big: bool },
    Float { bits: u16, big: bool },
}

pub fn read(path: &std::path::Path) -> Result<Audio, String> {
    let data = std::fs::read(path).map_err(|e| e.to_string())?;
    parse(&data)
}

pub fn parse(data: &[u8]) -> Result<Audio, String> {
    match data.get(0..4) {
        Some(b"RIFF") | Some(b"RF64") => wav(data),
        Some(b"FORM") => aiff(data),
        _ => Err("not a WAV or AIFF file".into()),
    }
}

fn u16le(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}
fn u32le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}
fn u16be(b: &[u8]) -> u16 {
    u16::from_be_bytes([b[0], b[1]])
}
fn u32be(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

/// Chunks after a 12-byte header: (id, body). Sizes are padded to even, as both formats require.
fn chunks(data: &[u8], big: bool) -> Vec<(&[u8], &[u8])> {
    let mut out = Vec::new();
    let mut pos = 12;
    while pos + 8 <= data.len() {
        let id = &data[pos..pos + 4];
        let size = if big { u32be(&data[pos + 4..]) } else { u32le(&data[pos + 4..]) } as usize;
        let start = pos + 8;
        let end = start.saturating_add(size).min(data.len());
        out.push((id, &data[start..end]));
        pos = start.saturating_add(size).saturating_add(size & 1);
    }
    out
}

fn wav(data: &[u8]) -> Result<Audio, String> {
    let mut fmt = None;
    let mut body = None;
    for (id, chunk) in chunks(data, false) {
        match id {
            b"fmt " if chunk.len() >= 16 => {
                let mut tag = u16le(chunk);
                let channels = u16le(&chunk[2..]);
                let rate = u32le(&chunk[4..]);
                let bits = u16le(&chunk[14..]);
                if tag == 0xFFFE && chunk.len() >= 26 {
                    tag = u16le(&chunk[24..]); // the sub-format GUID starts with the format tag
                }
                let format = match tag {
                    1 => Format::Int { bits, big: false },
                    3 => Format::Float { bits, big: false },
                    _ => return Err(format!("WAV format {tag} is not PCM or float")),
                };
                fmt = Some((format, channels, f64::from(rate)));
            }
            b"data" => body = Some(chunk),
            _ => {}
        }
    }
    let (format, channels, rate) = fmt.ok_or("no fmt chunk")?;
    decode(body.ok_or("no data chunk")?, format, channels, rate)
}

/// The 80-bit extended float AIFF stores the sample rate in.
fn extended(b: &[u8]) -> f64 {
    let exponent = i32::from(u16be(b) & 0x7FFF) - 16383;
    let mantissa = u64::from_be_bytes([b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9]]);
    let value = mantissa as f64 * 2f64.powi(exponent - 63);
    if b[0] & 0x80 != 0 { -value } else { value }
}

fn aiff(data: &[u8]) -> Result<Audio, String> {
    let aifc = data.get(8..12) == Some(b"AIFC");
    if !aifc && data.get(8..12) != Some(b"AIFF") {
        return Err("FORM is not AIFF or AIFC".into());
    }
    let mut comm = None;
    let mut sound = None;
    for (id, chunk) in chunks(data, true) {
        match id {
            b"COMM" if chunk.len() >= 18 => {
                let channels = u16be(chunk);
                let bits = u16be(&chunk[6..]);
                let rate = extended(&chunk[8..18]);
                let format = if aifc && chunk.len() >= 22 {
                    match &chunk[18..22] {
                        b"NONE" | b"twos" => Format::Int { bits, big: true },
                        b"sowt" => Format::Int { bits, big: false },
                        b"fl32" | b"FL32" => Format::Float { bits: 32, big: true },
                        b"fl64" | b"FL64" => Format::Float { bits: 64, big: true },
                        other => return Err(format!("AIFC compression {} is not supported", String::from_utf8_lossy(other))),
                    }
                } else {
                    Format::Int { bits, big: true }
                };
                comm = Some((format, channels, rate));
            }
            // SSND: offset, block size, then the samples.
            b"SSND" if chunk.len() >= 8 => {
                let offset = u32be(chunk) as usize;
                sound = Some(chunk.get(8 + offset..).unwrap_or(&[]));
            }
            _ => {}
        }
    }
    let (format, channels, rate) = comm.ok_or("no COMM chunk")?;
    decode(sound.ok_or("no SSND chunk")?, format, channels, rate)
}

fn decode(body: &[u8], format: Format, channels: u16, rate: f64) -> Result<Audio, String> {
    if channels == 0 || !(1000.0..=1_000_000.0).contains(&rate) {
        return Err("no channels, or an impossible sample rate".into());
    }
    let (bits, big, float) = match format {
        Format::Int { bits, big } => (bits, big, false),
        Format::Float { bits, big } => (bits, big, true),
    };
    let bytes = usize::from(bits.div_ceil(8));
    if !matches!((float, bits), (false, 8 | 16 | 24 | 32) | (true, 32 | 64)) {
        return Err(format!("{bits}-bit samples are not supported"));
    }
    let frame = bytes * usize::from(channels);
    let frames = body.len() / frame;
    let sample = |b: &[u8]| -> f32 {
        let mut v = [0u8; 8];
        for i in 0..bytes {
            v[i] = if big { b[bytes - 1 - i] } else { b[i] };
        }
        match (float, bits) {
            (true, 32) => f32::from_le_bytes([v[0], v[1], v[2], v[3]]),
            (true, _) => f64::from_le_bytes(v) as f32,
            // 8-bit WAV is unsigned; 8-bit AIFF is signed.
            (false, 8) => if big { f32::from(v[0] as i8) / 128.0 } else { (f32::from(v[0]) - 128.0) / 128.0 },
            (false, 16) => f32::from(i16::from_le_bytes([v[0], v[1]])) / 32_768.0,
            (false, 24) => (i32::from_le_bytes([0, v[0], v[1], v[2]]) >> 8) as f32 / 8_388_608.0,
            (false, _) => i32::from_le_bytes([v[0], v[1], v[2], v[3]]) as f32 / 2_147_483_648.0,
        }
    };
    let mut left = Vec::with_capacity(frames);
    let mut right = Vec::with_capacity(frames);
    for f in body.chunks_exact(frame).take(frames) {
        let l = sample(&f[..bytes]);
        let r = if channels > 1 { sample(&f[bytes..2 * bytes]) } else { l };
        left.push(if l.is_finite() { l } else { 0.0 });
        right.push(if r.is_finite() { r } else { 0.0 });
    }
    if left.is_empty() {
        return Err("the file holds no samples".into());
    }
    Ok(Audio { sample_rate: rate, left, right })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav_file(tag: u16, bits: u16, channels: u16, samples: &[u8], extensible: bool) -> Vec<u8> {
        let mut fmt = Vec::new();
        let block = channels * bits / 8;
        fmt.extend_from_slice(&(if extensible { 0xFFFE } else { tag }).to_le_bytes());
        fmt.extend_from_slice(&channels.to_le_bytes());
        fmt.extend_from_slice(&48_000u32.to_le_bytes());
        fmt.extend_from_slice(&(48_000 * u32::from(block)).to_le_bytes());
        fmt.extend_from_slice(&block.to_le_bytes());
        fmt.extend_from_slice(&bits.to_le_bytes());
        if extensible {
            fmt.extend_from_slice(&22u16.to_le_bytes());
            fmt.extend_from_slice(&bits.to_le_bytes());
            fmt.extend_from_slice(&3u32.to_le_bytes());
            fmt.extend_from_slice(&tag.to_le_bytes());
            fmt.extend_from_slice(&[0u8; 14]);
        }
        let mut body = b"WAVE".to_vec();
        for (id, chunk) in [(b"fmt ", &fmt[..]), (b"junk", &[1u8, 2, 3][..]), (b"data", samples)] {
            body.extend_from_slice(id);
            body.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
            body.extend_from_slice(chunk);
            if chunk.len() % 2 == 1 {
                body.push(0);
            }
        }
        let mut out = b"RIFF".to_vec();
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    fn aiff_file(compression: Option<&[u8; 4]>, bits: u16, samples: &[u8]) -> Vec<u8> {
        let mut comm = Vec::new();
        comm.extend_from_slice(&2u16.to_be_bytes());
        comm.extend_from_slice(&((samples.len() / (2 * usize::from(bits / 8))) as u32).to_be_bytes());
        comm.extend_from_slice(&bits.to_be_bytes());
        // 48000 as an 80-bit extended float.
        comm.extend_from_slice(&[0x40, 0x0E, 0xBB, 0x80, 0, 0, 0, 0, 0, 0]);
        if let Some(c) = compression {
            comm.extend_from_slice(c);
            comm.extend_from_slice(&[0, 0]);
        }
        let mut ssnd = vec![0u8; 8];
        ssnd.extend_from_slice(samples);
        let mut body = if compression.is_some() { b"AIFC".to_vec() } else { b"AIFF".to_vec() };
        for (id, chunk) in [(b"COMM", &comm[..]), (b"SSND", &ssnd[..])] {
            body.extend_from_slice(id);
            body.extend_from_slice(&(chunk.len() as u32).to_be_bytes());
            body.extend_from_slice(chunk);
        }
        let mut out = b"FORM".to_vec();
        out.extend_from_slice(&(body.len() as u32).to_be_bytes());
        out.extend_from_slice(&body);
        out
    }

    #[test]
    fn reads_wav_pcm_float_and_extensible() {
        // Stereo frames: (0.5, -0.25) in 16-bit.
        let s16: Vec<u8> = [16_384i16, -8_192].iter().flat_map(|v| v.to_le_bytes()).collect();
        let a = parse(&wav_file(1, 16, 2, &s16, false)).unwrap();
        assert_eq!((a.sample_rate, a.left[0], a.right[0]), (48_000.0, 0.5, -0.25));
        // 24-bit: 0x400000 = 0.5.
        let a = parse(&wav_file(1, 24, 2, &[0, 0, 0x40, 0, 0, 0xE0], true)).unwrap();
        assert_eq!((a.left[0], a.right[0]), (0.5, -0.25));
        let f: Vec<u8> = [0.75f32, -0.5].iter().flat_map(|v| v.to_le_bytes()).collect();
        let a = parse(&wav_file(3, 32, 2, &f, false)).unwrap();
        assert_eq!((a.left[0], a.right[0]), (0.75, -0.5));
        // Mono goes to both channels.
        let a = parse(&wav_file(1, 16, 1, &16_384i16.to_le_bytes(), false)).unwrap();
        assert_eq!((a.left[0], a.right[0]), (0.5, 0.5));
    }

    #[test]
    fn reads_aiff_and_aifc() {
        let be: Vec<u8> = [16_384i16, -8_192].iter().flat_map(|v| v.to_be_bytes()).collect();
        let a = parse(&aiff_file(None, 16, &be)).unwrap();
        assert_eq!((a.sample_rate, a.left[0], a.right[0]), (48_000.0, 0.5, -0.25));
        let le: Vec<u8> = [16_384i16, -8_192].iter().flat_map(|v| v.to_le_bytes()).collect();
        let a = parse(&aiff_file(Some(b"sowt"), 16, &le)).unwrap();
        assert_eq!((a.left[0], a.right[0]), (0.5, -0.25));
        let f: Vec<u8> = [0.75f32, -0.5].iter().flat_map(|v| v.to_be_bytes()).collect();
        let a = parse(&aiff_file(Some(b"fl32"), 32, &f)).unwrap();
        assert_eq!((a.left[0], a.right[0]), (0.75, -0.5));
    }

    #[test]
    fn refuses_what_it_cannot_read() {
        assert!(parse(b"OggS....").is_err());
        assert!(parse(&wav_file(2, 16, 2, &[0; 8], false)).is_err()); // ADPCM
        assert!(parse(&aiff_file(Some(b"ulaw"), 16, &[0; 8])).is_err());
        assert!(parse(&wav_file(1, 16, 2, &[], false)).is_err());
    }
}
