use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum LineKind {
    Code,
    Blank,
    Comment,
    BlockStart,
    BlockEnd,
}

impl LineKind {
    pub fn from_line(line: &[u8], lang: Language, in_block: bool) -> Self {
        if line.trim_ascii().is_empty() && !in_block {
            return Self::Blank;
        }

        if lang.is_block_start(line) {
            return Self::BlockStart;
        }

        if lang.is_block_end(line) {
            return Self::BlockEnd;
        }

        if lang.is_lc(line) || in_block {
            return LineKind::Comment;
        }

        LineKind::Code
    }
}

#[derive(Copy, Clone, Debug)]
struct BlockComment {
    start: &'static [u8],
    end: &'static [u8],
}

#[derive(Copy, Clone, Debug)]
pub struct Language {
    pub name: &'static str,
    line: Option<&'static [u8]>,
    block: Option<BlockComment>,
    ext: &'static [&'static str],
}

impl Language {
    fn check_line(check: Option<&[u8]>, line: &[u8]) -> bool {
        if check.is_none() {
            return false;
        }

        let c = check.unwrap();
        let len = c.len();

        let content = line.trim_ascii_start();

        if content.len() < len {
            return *content == c[..content.len()];
        }

        content[..len] == *c
    }

    pub fn is_file(&self, ext: &str) -> bool {
        self.ext.contains(&ext)
    }

    pub fn is_lc(&self, line: &[u8]) -> bool {
        return Self::check_line(self.line, line);
    }

    pub fn is_block_start(&self, line: &[u8]) -> bool {
        return Self::check_line(self.block.map(|b| b.start), line);
    }

    pub fn is_block_end(&self, line: &[u8]) -> bool {
        return Self::check_line(self.block.map(|b| b.end), line);
    }
}

pub const TEXT: Language = Language {
    name: "plaintext",
    line: None,
    block: None,
    ext: &[""],
};

pub const RUST: Language = Language {
    name: "rust",
    line: Some(b"//"),
    block: Some(BlockComment {
        start: b"/*",
        end: b"*/",
    }),
    ext: &["rs"],
};

pub const TOML: Language = Language {
    name: "toml",
    line: Some(b"#"),
    block: None,
    ext: &["toml"],
};

pub const LANGUAGES: &'static [Language] = &[RUST, TOML];

pub struct Count {
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

impl Count {
    pub fn new() -> Self {
        Self {
            blank: 0,
            comment: 0,
            code: 0,
        }
    }

    pub fn from_reader<R: Read>(reader: &mut R, lang: Language) -> std::io::Result<Self> {
        let mut ret = Self::new();

        let mut buf = [0u8; 4096];

        let mut in_block = false;

        loop {
            let n = reader.read(&mut buf)?;

            if n == 0 {
                break;
            }

            buf[..n].split(|b| *b == b'\n').for_each(|l| {
                let kind = LineKind::from_line(l, lang, in_block);

                match kind {
                    LineKind::Code => {
                        ret.code += 1;
                    }
                    LineKind::Blank => {
                        ret.blank += 1;
                    }
                    LineKind::Comment => {
                        ret.comment += 1;
                    }
                    LineKind::BlockStart => {
                        if !in_block {
                            in_block = true
                        }
                        ret.comment += 1
                    }
                    LineKind::BlockEnd => {
                        if in_block {
                            in_block = false
                        }
                    }
                }
            });
        }

        Ok(ret)
    }
}
