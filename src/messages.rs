use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    problems::PROBLEMS,
    report,
};

lazy_static! {
    pub static ref TYPE_RE: Regex =
        Regex::new(r"^([a-zA-Z\-_\s/]+)(\([a-zA-Z\-_\s/]+\))?(!)?(\s*):").unwrap();
}

#[derive(Debug)]
pub struct Footer<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Default)]
pub struct Message<'a> {
    pub type_: Option<&'a str>,
    pub scope: Option<&'a str>,
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
    pub footers: Vec<Footer<'a>>,
    pub is_breaking_change: bool,
}

#[derive(Default)]
struct Header<'a> {
    type_: &'a str,
    scope: &'a str,
    title: &'a str,
    ex_mark: bool,
    whitespace_after_type: &'a str,
}

struct Body<'a> {
    body: &'a str,
    footers: Vec<Footer<'a>>,
    is_breaking_change: bool,
    newlines_before: usize,
    newlines_after: usize,
}

fn string_to_option<'a>(text: &'a str) -> Option<&'a str> {
    let text = text.trim();
    (!text.is_empty()).then_some(text)
}

fn parse_header<'a>(text: &'a str) -> Header<'a> {
    let Some(captures) = TYPE_RE.captures(text) else {
        return Header {
            title: text,
            ..Default::default()
        };
    };
    let type_ = captures.get(1).map(|m| m.as_str()).unwrap_or_default();
    let scope = captures.get(2).map(|m| m.as_str()).unwrap_or_default();
    let ex_mark = captures.get(3).is_some();
    let whitespace_after_type = captures.get(4).map(|m| m.as_str()).unwrap_or_default();
    let prefix = format!(
        "{}{}{}{}:",
        type_,
        scope,
        if ex_mark { "!" } else { "" },
        whitespace_after_type
    );
    let title = &text[prefix.len()..];
    Header {
        type_,
        scope,
        title,
        ex_mark,
        whitespace_after_type,
    }
}

fn find_problems_in_header(header: &Header) {
    if !header.type_.trim().is_empty() && header.type_.starts_with(' ') {
        report!("Whitespace before commit message type.");
    }
    if header.whitespace_after_type.len() > 0
        || (!header.type_.trim().is_empty() && header.type_.ends_with(' '))
    {
        report!("Whitespace after commit message type.");
    }
    if header.scope.starts_with("( ") {
        report!("Whitespace before commit message scope.");
    }
    if header.scope.ends_with(" )") {
        report!("Whitespace after commit message scope.");
    }
    if header.title.starts_with(' ') {
        if !header.title.trim().is_empty() && header.title[1..].starts_with(' ') {
            report!("Whitespace before commit message title.");
        }
    } else {
        report!("No space before commit message title.");
    }
    if !header.title.trim().is_empty() && header.title.ends_with(' ') {
        report!("Whitespace after commit message title.");
    }
}

fn parse_body<'a>(text: &'a str) -> Body<'a> {
    let newlines_before = text.chars().take_while(|c| *c == '\n').count();
    let mut footers = vec![];
    for line in text.lines().rev() {
        if line.is_empty() || !line.contains(':') {
            break;
        }
        footers.push(line);
    }
    let footer_length = footers
        .iter()
        .map(|f| f.len() + 1)
        .sum::<usize>()
        .saturating_sub(if text.ends_with('\n') { 0 } else { 1 });
    let mut body = &text[newlines_before..text.len() - footer_length];
    let newlines_after = body.chars().rev().take_while(|c| *c == '\n').count();
    if !body.is_empty() && newlines_after == 0 {
        body = &text[newlines_before..];
        footers.clear();
    }
    let body = &body[..body.len() - newlines_after];
    let footers: Vec<Footer> = footers
        .iter()
        .rev()
        .map(|footer| {
            let (key, value) = footer.split_once(':').unwrap();
            let value = value.strip_prefix(' ').unwrap_or(value);
            Footer { key, value }
        })
        .collect();
    let is_breaking_change = footers.iter().any(|f| f.key == "BREAKING CHANGE");
    Body {
        body,
        footers,
        newlines_before,
        newlines_after,
        is_breaking_change,
    }
}

fn find_problems_in_body(header: &Header, body: &Body) {
    if !body.body.is_empty() && body.newlines_before != 1 {
        report!("Commit message title and body must be separated by a single newline.");
    }
    if !body.body.is_empty() && body.newlines_after != 1 {
        report!("Commit message body and footer must be separated by a single newline.");
    }
    if header.ex_mark && !body.is_breaking_change {
        report!(
            "Breaking changes should be explained in a footer after the commit message. (example: `BREAKING CHANGE: ...`)"
        )
    }
    if body.is_breaking_change && !header.ex_mark {
        report!(
            "Breaking changes should be marked with `!` after the commit message type. (example: `feat!: ...`)"
        );
    }
}

pub fn parse_message<'a>(text: &'a str) -> Message<'a> {
    let first_line = text.split_once('\n').map(|(p, _)| p).unwrap_or(text);
    let header = parse_header(first_line);
    let body = parse_body(&text[(first_line.len() + 1).min(text.len())..]);
    find_problems_in_header(&header);
    find_problems_in_body(&header, &body);
    Message {
        type_: string_to_option(header.type_),
        scope: string_to_option(
            header
                .scope
                .strip_prefix('(')
                .unwrap_or_default()
                .strip_suffix(')')
                .unwrap_or_default(),
        ),
        title: string_to_option(header.title),
        body: (!body.body.is_empty()).then_some(body.body),
        footers: body.footers,
        is_breaking_change: header.ex_mark || body.is_breaking_change,
    }
}
