//! Tesla's known-answer tests: four 10-Qs, each read by the EDGAR reader
//! from its statement pages (the accession's folder here: FilingSummary.xml
//! and the R pages it opens) and checked line by line against the
//! statements transcribed by hand from the filing.

mod common;

#[path = "tsla/q1_2020.rs"]
mod q1_2020;
#[path = "tsla/q2_2020.rs"]
mod q2_2020;
#[path = "tsla/q3_2020.rs"]
mod q3_2020;
#[path = "tsla/q3_2025.rs"]
mod q3_2025;
