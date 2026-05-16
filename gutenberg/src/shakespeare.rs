use std::error::Error;
use std::io::{BufRead, BufReader};

//                   Contents
//
//    ALL’S WELL THAT ENDS WELL
//    THE TRAGEDY OF ANTONY AND CLEOPATRA
//    AS YOU LIKE IT
//    THE COMEDY OF ERRORS
//    THE TRAGEDY OF CORIOLANUS
//    CYMBELINE
//    THE TRAGEDY OF HAMLET, PRINCE OF DENMARK
//    THE FIRST PART OF KING HENRY THE FOURTH
//    THE SECOND PART OF KING HENRY THE FOURTH
//    THE LIFE OF KING HENRY THE FIFTH
//    THE FIRST PART OF HENRY THE SIXTH
//    THE SECOND PART OF KING HENRY THE SIXTH
//    THE THIRD PART OF KING HENRY THE SIXTH
//    KING HENRY THE EIGHTH
//    THE LIFE AND DEATH OF KING JOHN
//    THE TRAGEDY OF JULIUS CAESAR
//    THE TRAGEDY OF KING LEAR
//    LOVE’S LABOUR’S LOST
//    THE TRAGEDY OF MACBETH
//    MEASURE FOR MEASURE
//    THE MERCHANT OF VENICE
//    THE MERRY WIVES OF WINDSOR
//    A MIDSUMMER NIGHT’S DREAM
//    MUCH ADO ABOUT NOTHING
//    THE TRAGEDY OF OTHELLO, THE MOOR OF VENICE
//    PERICLES, PRINCE OF TYRE
//    KING RICHARD THE SECOND
//    KING RICHARD THE THIRD
//    THE TRAGEDY OF ROMEO AND JULIET
//    THE TAMING OF THE SHREW
//    THE TEMPEST
//    THE LIFE OF TIMON OF ATHENS
//    THE TRAGEDY OF TITUS ANDRONICUS
//    TROILUS AND CRESSIDA
//    TWELFTH NIGHT; OR, WHAT YOU WILL
//    THE TWO GENTLEMEN OF VERONA
//    THE TWO NOBLE KINSMEN
//    THE WINTER’S TALE
//    A LOVER’S COMPLAINT
//    THE PASSIONATE PILGRIM
//    THE PHOENIX AND THE TURTLE
//    THE RAPE OF LUCRECE
//    VENUS AND ADONIS
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
