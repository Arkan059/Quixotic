const SUFFIXES: [&str; 32] = [
    "", "K", "M", "B", "T", "Qa", "Qi", "Sx", "Sp", "Oc", "No", "Dc", "UDc", "DDc", "TDc", "QaDc",
    "QiDc", "SxDc", "SpDc", "OcDc", "NoDc", "Vg", "UVg", "DVg", "TVg", "QaVg", "QiVg", "SxVg",
    "SpVg", "OcVg", "NoVg", "Tg",
];

fn trim_decimal(value: String) -> String {
    if value.contains('.') {
        let trimmed = value.trim_end_matches('0').trim_end_matches('.');
        trimmed.to_string()
    } else {
        value
    }
}

fn mantissa(value: f64) -> String {
    if value < 10.0 {
        trim_decimal(format!("{:.2}", value))
    } else if value < 100.0 {
        trim_decimal(format!("{:.1}", value))
    } else {
        format!("{:.0}", value)
    }
}

fn plain(value: f64) -> String {
    if value < 0.01 {
        return "0".to_string();
    }
    if value < 1.0 {
        trim_decimal(format!("{:.3}", value))
    } else if value < 10.0 {
        trim_decimal(format!("{:.2}", value))
    } else if value < 100.0 {
        trim_decimal(format!("{:.1}", value))
    } else {
        format!("{:.0}", value)
    }
}

pub fn number(value: f64) -> String {
    if value.is_nan() {
        return "0".to_string();
    }
    if value.is_infinite() {
        if value.is_sign_negative() {
            return "-INF".to_string();
        }
        return "INF".to_string();
    }
    let negative = value.is_sign_negative() && value != 0.0;
    let magnitude = value.abs();
    if magnitude < 999.5 {
        let text = plain(magnitude);
        return if negative { format!("-{}", text) } else { text };
    }
    let tier = ((magnitude.log10() / 3.0).floor() as usize).min(SUFFIXES.len());
    let mut tier = tier;
    if tier >= SUFFIXES.len() {
        let exponent = magnitude.log10().floor() as i32;
        let coefficient = magnitude / 10f64.powi(exponent);
        let text = format!(
            "{}e{}",
            trim_decimal(format!("{:.2}", coefficient)),
            exponent
        );
        return if negative { format!("-{}", text) } else { text };
    }
    let mut scaled = magnitude / 10f64.powi((tier * 3) as i32);
    if scaled >= 999.5 && tier + 1 < SUFFIXES.len() {
        tier += 1;
        scaled = magnitude / 10f64.powi((tier * 3) as i32);
    }
    let text = format!("{}{}", mantissa(scaled), SUFFIXES[tier]);
    if negative {
        format!("-{}", text)
    } else {
        text
    }
}

pub fn rate(value: f64) -> String {
    format!("{}/s", number(value))
}

pub fn percent(fraction: f64) -> String {
    format!("{:.0}%", (fraction * 100.0).clamp(0.0, 999.0))
}

pub fn duration(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    if total >= 86400 {
        let days = total / 86400;
        let hours = (total % 86400) / 3600;
        format!("{}d {}h", days, hours)
    } else if total >= 3600 {
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    } else if total >= 60 {
        let minutes = total / 60;
        let seconds = total % 60;
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", total)
    }
}

pub fn clock(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let secs = total % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_numbers_use_compact_suffixes() {
        assert_eq!(number(999.0), "999");
        assert_eq!(number(1000.0), "1K");
        assert_eq!(number(1234.0), "1.23K");
        assert_eq!(number(3_400_000.0), "3.4M");
        assert_eq!(number(5_600_000_000.0), "5.6B");
        assert!(number(1.0e96).contains('e'));
        assert!(number(f64::INFINITY).contains("INF"));
    }

    #[test]
    fn durations_and_percentages_read_cleanly() {
        assert_eq!(duration(45.0), "45s");
        assert_eq!(duration(125.0), "2m 5s");
        assert_eq!(duration(3_725.0), "1h 2m");
        assert_eq!(percent(0.75), "75%");
        assert_eq!(clock(3661.0), "01:01:01");
    }
}
