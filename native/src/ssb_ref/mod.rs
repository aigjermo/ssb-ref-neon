use qstring::QString;
use regex::Regex;
use std::str;
use urlencoding::decode;

pub enum Ref<'a> {
    Feed(&'a str),
    Blob((&'a str, Option<&'a str>)),
    Message((&'a str, Option<&'a str>)),
    CloakedMessage((&'a str, Option<&'a str>)),
}

fn split_query(id: &str) -> (&str, Option<&str>) {
    match id.find('?') {
        Some(ix) => (&id[..ix], Some(&id[ix..])),
        None => (id, None),
    }
}

impl Ref<'_> {
    pub fn from(id: &str) -> Result<Ref, &str> {
        let (id, q) = split_query(id);
        match id.get(0..1) {
            Some("@") => Ok(Ref::Feed(validate_key(id, "@", ".ed25519")?)),
            Some("&") => Ok(Ref::Blob((validate_key(id, "&", ".sha256")?, q))),
            Some("%") if id.ends_with(".sha256") => {
                Ok(Ref::Message((validate_key(id, "%", ".sha256")?, q)))
            }
            Some("%") if id.ends_with(".cloaked") => {
                Ok(Ref::CloakedMessage((validate_key(id, "%", ".cloaked")?, q)))
            }
            _ => Err("Invalid or unknown reference format"),
        }
    }

    pub fn id(&self) -> String {
        match self {
            Ref::Feed(id) => id.to_string(),
            Ref::Blob((id, _)) => id.to_string(),
            Ref::Message((id, _)) => id.to_string(),
            Ref::CloakedMessage((id, _)) => id.to_string(),
        }
    }

    pub fn has_query(&self) -> bool {
        match self {
            Ref::Message((_, Some(_))) => true,
            Ref::Blob((_, Some(_))) => true,
            _ => false,
        }
    }

    pub fn query(&self) -> Option<QString> {
        match self {
            Ref::Feed(_) => None,
            Ref::Blob((_, q)) => q.map(|q| QString::from(q)),
            Ref::Message((_, q)) => q.map(|q| QString::from(q)),
            Ref::CloakedMessage((_, q)) => q.map(|q| QString::from(q)),
        }
    }

    pub fn type_str(&self) -> &str {
        match self {
            Ref::Feed(_) => "feed",
            Ref::Blob(_) => "blob",
            Ref::Message(_) => "msg",
            Ref::CloakedMessage(_) => "cloaked_message",
        }
    }
}

/// Valid keys must contain a canonical base64 encoded 32 byte value.
fn validate_key<'a>(key: &'a str, prefix: &str, suffix: &str) -> Result<&'a str, &'a str> {
    let re_key = Regex::new(r"^[A-Za-z0-9/+]{42}[AEIMQUYcgkosw048]=$").unwrap();

    match key
        .strip_prefix(prefix)
        .and_then(|k| k.strip_suffix(suffix))
        .and_then(|k| Some(re_key.is_match(k)))
    {
        Some(true) => Ok(key),
        _ => Err("not a valid base64 encoded key"),
    }
}

/// Check that a link is valid and if there is a query part, parse it.
pub fn parse_query(id: &str) -> Option<(String, Option<QString>)> {
    match Ref::from(id).map(|x| (x.id(), x.query())) {
        Ok(res) => Some(res),
        Err(_) => None,
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
