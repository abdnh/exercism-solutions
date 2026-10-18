use regex::{Captures, Regex};
use std::{borrow::Cow, sync::LazyLock};

static RULE1_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?<g1>([aeiou]|xr|yt).*)$"#).unwrap());
static RULE2_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?<g2>[^aeiou]+)(?<g1>.*)$"#).unwrap());
static RULE3_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?<g2>[^aeiou]*?qu)(?<g1>.*)$"#).unwrap());
static RULE4_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^(?<g2>[^aeiou]+?)(?<g1>y.*)$"#).unwrap());

pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            for regex in [&RULE1_RE, &RULE3_RE, &RULE4_RE, &RULE2_RE] {
                let replaced = regex.replace_all(word, |caps: &Captures| {
                    let s = &["g1", "g2"]
                        .iter()
                        .map(|g| caps.name(g).map(|m| m.as_str()).unwrap_or_default())
                        .collect::<String>();
                    format!("{s}ay")
                });
                if let Cow::Owned(s) = replaced {
                    return s;
                }
            }

            word.into()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
