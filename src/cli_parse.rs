use crate::program_data::ProgramData;
use std::{env, vec};

pub fn parse() -> ProgramData {
    let mut program_data: ProgramData = ProgramData::new();

    if env::args().len() > 1 {
        // parse args bruh
        program_data.nums = vec![0];
        program_data.words = vec!["todo"];
    }

    program_data
}
