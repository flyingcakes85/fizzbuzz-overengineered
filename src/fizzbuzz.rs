use core::panic;

use crate::program_data::ProgramData;

pub fn fizzbuzz(program_data: ProgramData, lower_limit: Option<usize>, upper_limit: Option<usize>) {
    if program_data.nums.len() != program_data.words.len() {
        panic!("Wrong argument sizes");
    }

    if lower_limit > upper_limit {
        panic!("lower limit greater than upper limit");
    }

    let data_length = program_data.nums.len();

    let lower_limit = lower_limit.unwrap();
    let upper_limit = upper_limit.unwrap();

    // TODO
    // for num in lower_limit..upper_limit {
    //     for data_item in 0..data_length {}
    // }
}
