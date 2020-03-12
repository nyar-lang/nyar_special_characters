use crate::ucd::{XID_CONTINUE, XID_START};
use unic::{char::range::CharRange, ucd::Block};

pub fn xid_start_text() -> String {
    let (mut name, mut group) = ("", "Basic Latin");
    let mut chars = String::new();
    let mut text = String::from("# XID Start\n\n");
    for (s, e) in XID_START {
        name = Block::of(*s).unwrap().name;
        for c in CharRange::closed(*s, *e) {
            chars.push(c)
        }
        chars.push('\n');
        if name != group {
            text.push_str(&format!("## {}({})\n", group, chars.chars().count()));
            text.push_str(&chars);
            text.push('\n');
            group = name;
            chars = String::new()
        }
    }
    return text;
}

pub fn xid_continue_text() -> String {
    let (mut name, mut group) = ("", "Basic Latin");
    let mut chars = String::new();
    let mut text = String::from("# XID Continue\n\n");
    for (s, e) in XID_CONTINUE {
        name = Block::of(*s).unwrap().name;
        for c in CharRange::closed(*s, *e) {
            chars.push(c)
        }
        chars.push('\n');
        if name != group {
            text.push_str(&format!("## {}({})\n", group, chars.chars().count()));
            text.push_str(&chars);
            text.push('\n');
            group = name;
            chars = String::new()
        }
    }
    return text;
}