use std::{
    fs::File,
    io::{BufReader, Read},
};

pub struct Count {
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

fn is_line(text: &[u8]) -> bool {
    false
}

/// for simplicity and to avoid accidental duplicate io checks, this function
/// assumes the file exists and is a file
pub fn count_file(path: &std::path::PathBuf) -> std::io::Result<Count> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buf = [0u8; 4096];

    let mut lc = 0;

    loop {
        let n = reader.read(&mut buf)?;

        if n == 0 {
            break;
        }

        lc += buf[..n].iter().filter(|&&b| b == b'\n').count();
    }

    Ok(Count {
        blank: 0,
        code: lc,
        comment: 0,
    })
}
