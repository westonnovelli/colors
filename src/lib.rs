pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn to_rgb_string(rgb: &Rgb) -> String {
    format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b)
}

pub fn parse_hex(input: &str) -> Result<Rgb, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("empty input".to_string());
    }

    let s = trimmed.strip_prefix('#').unwrap_or(trimmed);

    let len = s.len();
    if len != 3 && len != 6 {
        return Err(format!("expected 3 or 6 hex digits, got {}", len));
    }

    let chars: Vec<char> = s.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if c.to_digit(16).is_none() {
            return Err(format!("invalid hex digit '{}' at position {}", c, i + 1));
        }
    }

    fn hex_val(c: char) -> u8 {
        c.to_digit(16).unwrap() as u8
    }

    let rgb = if len == 3 {
        let r = hex_val(chars[0]) * 17; // duplicate nibble
        let g = hex_val(chars[1]) * 17;
        let b = hex_val(chars[2]) * 17;
        Rgb { r, g, b }
    } else {
        let r = (hex_val(chars[0]) << 4) | hex_val(chars[1]);
        let g = (hex_val(chars[2]) << 4) | hex_val(chars[3]);
        let b = (hex_val(chars[4]) << 4) | hex_val(chars[5]);
        Rgb { r, g, b }
    };

    Ok(rgb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_shorthand_lowercase() {
        let rgb = parse_hex("fff").unwrap();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 255);
    }

    #[test]
    fn parses_shorthand_with_hash() {
        let rgb = parse_hex("#0f0").unwrap();
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn parses_six_digit() {
        let rgb = parse_hex("112233").unwrap();
        assert_eq!(rgb.r, 0x11);
        assert_eq!(rgb.g, 0x22);
        assert_eq!(rgb.b, 0x33);
    }

    #[test]
    fn parses_six_digit_with_hash() {
        let rgb = parse_hex("#112233").unwrap();
        assert_eq!(rgb.r, 0x11);
        assert_eq!(rgb.g, 0x22);
        assert_eq!(rgb.b, 0x33);
    }

    #[test]
    fn rejects_invalid_inputs() {
        for bad in ["", "abcd", "ggg", "1234", "#11223344"] {
            assert!(parse_hex(bad).is_err(), "expected error for input '{}'", bad);
        }
    }

    #[test]
    fn formats_rgb_string() {
        let s = to_rgb_string(&Rgb { r: 1, g: 2, b: 3 });
        assert_eq!(s, "rgb(1, 2, 3)");
    }
}
