// https://fhug.org.uk/kb/link/gedcom-standard-release-5-5-and-5-5-1-3/
// https://edge.fscdn.org/assets/img/documents/ged551-5bac5e57fe88dd37df0e153d9c515335.pdf

use std::str::FromStr;
use std::{iter::Peekable, str::Chars};

use crate::tag::Tag;

// TODO
#[derive(Debug)]
pub enum TokenizerError {
    ParseIntError,
}

// A gedcom_line has the following syntax:
// gedcom_line:= level + delim + [optional_xref_ID] + tag + [optional_line_value] + terminator
#[derive(Debug, Clone)]
pub struct Line {
    pub level: u8,
    pub tag: Tag,
    pub ref_id: Option<String>,
    pub pointer: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    content: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            chars: content.chars().peekable(),
        }
    }

    // tbd; wip
    pub fn next_line(&mut self) -> Option<Line> {
        if matches!(self.chars.peek(), None) {
            return None;
        }

        // Remove any spaces before the level (if there are any)
        self.consume_delimiter();

        // Get the level and parse it from String into u8
        let level = self.get_level();

        self.consume_delimiter();

        let ref_id = self.get_ref_id();

        self.consume_delimiter();

        let tag = self.get_tag();

        let (pointer, value) = self.get_line_value();

        // Make sure we jump to the next line
        if matches!(self.chars.peek(), Some('\n')) {
            self.chars.next();
        }

        Some(Line {
            pointer,
            value,
            tag,
            ref_id,
            level,
        })
    }

    // Level numbers must be between 0 to 99 and must not contain leading zeroes.
    // Each new level number must be no higher than the previous line plus 1.
    //
    // grammar: level:= [digit | digit + digit ]
    // (Do not use non-significant leading zeroes such as 02.)
    pub(self) fn get_level(&mut self) -> u8 {
        let mut level_value = String::new();

        while let Some(c) = self.chars.peek() {
            let c = *c;
            if c.is_digit(10) {
                level_value.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        match level_value.parse::<u8>() {
            Ok(l) => Ok(l),
            Err(_) => Err(TokenizerError::ParseIntError), // todo
        }
        .unwrap() // todo, return result instead
    }

    // The cross-reference ID has a maximum of 22 characters, including the enclosing ‘at’ signs (@),
    // and it must be unique within the GEDCOM transmission.
    //
    // grammar: optional_xref_ID:= xref_ID + delim
    // xref_ID:= [pointer]
    //   where:
    //     pointer:= [(0x40) + alphanum + pointer_string + (0x40) ]
    //       where: (0x40)=@
    pub(self) fn get_ref_id(&mut self) -> Option<String> {
        if !matches!(self.chars.peek(), Some('\x40')) {
            return None;
        }

        let mut value = String::new();

        if matches!(self.chars.peek(), Some('\x40')) {
            value.push('\x40');
            self.chars.next();
        };

        while let Some(c) = self.chars.peek() {
            let c = *c;
            if !matches!(c, '\x20' | '\n') {
                value.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        Some(value)
    }

    // The length of the GEDCOM TAG is a maximum of 31 characters,
    // with the first 15 characters being unique.
    // grammar tag:= [alphanum | tag + alphanum ]
    pub(self) fn get_tag(&mut self) -> Tag {
        let mut tag_value = String::new();

        while let Some(c) = self.chars.peek() {
            let c = *c;
            if !matches!(c, '\x20' | '\n') {
                tag_value.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        if tag_value.starts_with('\x5f') {
            Tag::Custom(tag_value.to_string())
        } else {
            Tag::from_str(&tag_value).unwrap()
        }
    }

    // optional_line_value:= delim + line_value
    // line_value:= [ pointer | line_item ]
    // where:
    //   pointer:= [(0x40) + alphanum + pointer_string + (0x40) ]
    //     where: (0x40)=@
    //   line_item:= [any_char | escape | line_item + any_char | line_item + escape]
    pub(self) fn get_line_value(&mut self) -> (Option<String>, Option<String>) {
        if !matches!(self.chars.peek(), Some('\x20')) {
            return (None, None);
        }

        self.consume_delimiter();

        let mut pointer: Option<String> = None;
        let mut value: Option<String> = None;

        if matches!(self.chars.peek(), Some('\x40')) {
            pointer = self.get_ref_id();
        } else if !matches!(self.chars.peek(), Some('\x20') | Some('\n')) {
            let mut line_value = String::new();

            while let Some(c) = self.chars.peek() {
                let c = *c;
                if !matches!(c, '\n') {
                    line_value.push(c);
                    self.chars.next();
                } else {
                    break;
                }
            }
            value = Some(line_value);
        }

        (pointer, value)
    }

    // grammar: delim:= [(0x20) ]
    // where: (0x20)=space_character
    pub(self) fn consume_delimiter(&mut self) {
        while matches!(self.chars.peek(), Some('\x20')) {
            self.chars.next();
        }
    }
}
