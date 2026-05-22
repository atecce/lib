use std::error::Error;
use std::io::{BufRead, BufReader};

pub struct Reader<R> {
    r: BufReader<R>,
}

pub fn new_reader() -> Reader<&'static [u8]> {
    Reader {
        r: BufReader::new(&include_bytes!("../../gutenberg/cache/epub/100/pg100.txt")[..]),
    }
}

impl<R: std::io::Read> Reader<R> {
    pub fn read_until_sonnets(self) -> Vec<Vec<String>> {
        let mut lines = self.r.lines()
            .skip_while(|l| l.as_ref().map_or(true, |s| !s.contains("THE SONNETS")))
            .skip(1)
            .skip_while(|l| l.as_ref().map_or(true, |s| !s.contains("THE SONNETS")))
            .skip(1);

        (0..154)
            .map(|_| {
                lines.by_ref()
                    .take(18)
                    .map(|l| l.map(|s| s.trim().to_string()))
                    .filter(|l| l.as_ref().map_or(true, |s| !s.is_empty()))
                    .skip(1)
                    .collect::<Result<Vec<String>, _>>()
            })
            .collect::<Result<_, _>>().unwrap()
    }

    // TODO(atec): obviously get whole play instead
    pub fn read_until_alls_well_that_ends_well_act_i(self) -> Vec<String> {
        self.r.lines()
            .map_while(Result::ok)
            .skip_while(|l| !l.contains("ACT I"))
            .skip(1)
            .skip_while(|l| !l.contains("ACT I"))
            .skip(1)
            .take_while(|l| !l.contains("[_Exeunt._]"))
            .filter(|l| !l.trim().is_empty())
            .collect()
    }
}
