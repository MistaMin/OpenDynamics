//! Parameter value text, the same for hosts (automation lanes, tooltips, typed values) and
//! the editor: dynamics controls in whole steps, at most three digits per number, no "0.x"
//! times (microseconds instead), kilo units from 1000, never "-0".

/// A non-negative number in at most three digits.
fn fit(v: f32) -> String {
    match v {
        v if v.round() >= 100.0 => format!("{v:.0}"),
        v if (v * 10.0).round() >= 100.0 => format!("{v:.1}"),
        v => format!("{v:.2}"),
    }
}

fn signed_whole(v: f32) -> String {
    let r = v.round();
    if r == 0.0 { "0".into() } else { format!("{r:.0}") }
}

/// The number at the start of `s` and the rest, lower-cased and trimmed.
fn split(s: &str) -> Option<(f32, String)> {
    let s = s.trim();
    let end = s.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+')).unwrap_or(s.len());
    let n: f32 = s[..end].trim().parse().ok()?;
    n.is_finite().then(|| (n, s[end..].trim().to_lowercase()))
}

/// A unitless value (a Q) in three digits.
pub fn plain(v: f32) -> String {
    if v < 0.0 { format!("-{}", fit(-v)) } else { fit(v) }
}

pub fn parse_plain(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    rest.is_empty().then_some(n)
}

pub fn db(v: f32) -> String {
    format!("{} dB", signed_whole(v))
}

/// Loudness in whole LUFS.
pub fn lufs(v: f32) -> String {
    format!("{} LUFS", signed_whole(v))
}

pub fn parse_lufs(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    matches!(rest.as_str(), "" | "lufs" | "lu").then_some(n)
}

/// True peak keeps a tenth: the ceiling is set in tenths of a dB.
pub fn dbtp(v: f32) -> String {
    let t = (v * 10.0).round() / 10.0;
    if t == 0.0 { "0.0 dBTP".into() } else { format!("{t:.1} dBTP") }
}

pub fn parse_dbtp(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    matches!(rest.as_str(), "" | "db" | "dbtp").then_some(n)
}

pub fn parse_db(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    matches!(rest.as_str(), "" | "db").then_some(n)
}

pub fn ratio(v: f32) -> String {
    format!("{:.0}:1", v.round().max(1.0))
}

pub fn parse_ratio(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    matches!(rest.as_str(), "" | ":1").then_some(n)
}

/// The decompressor's ratio is the x of 1:x.
pub fn upward_ratio(v: f32) -> String {
    format!("1:{:.0}", v.round().max(1.0))
}

pub fn parse_upward_ratio(s: &str) -> Option<f32> {
    let t = s.trim();
    match t.strip_prefix("1:") {
        Some(x) => x.trim().parse().ok().filter(|v: &f32| v.is_finite()),
        None => split(t).and_then(|(n, rest)| rest.is_empty().then_some(n)),
    }
}

pub fn ms(v: f32) -> String {
    if v < 1.0 {
        let us = (v * 1000.0).round();
        if us < 0.5 { "0 ms".into() } else { format!("{us:.0} µs") }
    } else if v >= 1000.0 {
        format!("{} s", fit(v / 1000.0))
    } else {
        format!("{} ms", fit(v))
    }
}

pub fn parse_ms(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    match rest.as_str() {
        "" | "ms" => Some(n),
        "µs" | "us" => Some(n / 1000.0),
        "s" => Some(n * 1000.0),
        _ => None,
    }
}

pub fn hz(v: f32) -> String {
    if v >= 1000.0 { format!("{} kHz", fit(v / 1000.0)) } else { format!("{} Hz", fit(v)) }
}

pub fn parse_hz(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    match rest.as_str() {
        "" | "hz" => Some(n),
        "khz" | "k" => Some(n * 1000.0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_and_reads_back() {
        assert_eq!(db(-18.0), "-18 dB");
        assert_eq!(db(-0.2), "0 dB");
        assert_eq!(ratio(4.0), "4:1");
        assert_eq!(upward_ratio(2.0), "1:2");
        assert_eq!(ms(0.01), "10 µs");
        assert_eq!(ms(0.0), "0 ms");
        assert_eq!(ms(120.0), "120 ms");
        assert_eq!(ms(5000.0), "5.00 s");
        assert_eq!(hz(22_000.0), "22.0 kHz");
        assert_eq!(hz(141.4), "141 Hz");
        assert_eq!(parse_db("-30 dB"), Some(-30.0));
        assert_eq!(parse_ratio("4:1"), Some(4.0));
        assert_eq!(parse_upward_ratio("1:3"), Some(3.0));
        assert_eq!(parse_ms("10 µs"), Some(0.01));
        assert_eq!(parse_ms("2.5 s"), Some(2500.0));
        assert_eq!(parse_hz("2.25 kHz"), Some(2250.0));
        assert_eq!(parse_db("loud"), None);
        assert_eq!(lufs(-14.2), "-14 LUFS");
        assert_eq!(dbtp(-1.04), "-1.0 dBTP");
        assert_eq!(dbtp(-0.01), "0.0 dBTP");
        assert_eq!(parse_lufs("-23 LUFS"), Some(-23.0));
        assert_eq!(parse_dbtp("-0.5 dBTP"), Some(-0.5));
    }
}
