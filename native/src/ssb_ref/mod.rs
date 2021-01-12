use qstring::QString;
use regex::Regex;
use ssb_multiformats::multihash::Multihash;
use std::str;
use urlencoding::decode;

/// Check that the argument is a valid reference to a blob, with no query or
/// other trailing data.
pub fn is_blob(blob: &str) -> bool {
    match Multihash::from_legacy(blob.as_bytes()) {
        Ok((Multihash::Blob(_), tail)) => tail.len() == 0,
        _ => false,
    }
}

/// Check that a link is valid and if there is a query part, parse it.
pub fn parse_link(str: &str) -> Option<(String, Option<QString>)> {
    match Multihash::from_legacy(str.as_bytes()) {
        Err(_) => None,
        Ok((link, tail)) => Some((
            link.to_legacy_string(),
            match str::from_utf8(&tail) {
                Err(_) => None,
                Ok(q) => {
                    if q.starts_with('?') {
                        Some(QString::from(q))
                    } else {
                        None
                    }
                }
            },
        )),
    }
}

/// Strip everything except from a valid reference from a string and return it.
pub fn extract_link(str: &str) -> Option<String> {
    let re_link = Regex::new(r"[@%&][A-Za-z0-9/+]{43}=\.[\w\d]+").unwrap();
    let re_amp = Regex::new(r"&amp;").unwrap();

    let try_match = |str| {
        re_link
            .captures(str)
            .map(|captures| captures.get(0).unwrap().as_str().to_string())
    };

    // try string as is first
    match try_match(str) {
        Some(result) => Some(result),
        _ => {
            // urldecode and remove &amp;
            let mut str = str.to_string();
            if let Ok(decoded) = decode(&str) {
                str = decoded.to_string();
            }
            let str = re_amp.replace(&str, "&");

            try_match(&str)
        }
    }
}

/// Strip unwanted characters and lowercase a channel name
pub fn normalize_channel_name(str: &str) -> Option<String> {
    let re_filter = Regex::new(r#"\s|[,.?!<>()\[\]#"]"#).unwrap();
    let name = str.to_string().to_lowercase();
    let name = re_filter.replace_all(&name, "").to_string();

    match name.len() {
        0 => None,
        n if n > 30 => Some(name[0..30].to_string()),
        _ => Some(name),
    }
}
