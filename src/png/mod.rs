use crate::util;
use std::fs::File;
use std::io::Read;
use std::prelude::*;

pub struct IHDR {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
}

impl IHDR {
    pub fn new(
        width: u32,
        height: u32,
        bit_depth: u8,
        color_type: u8,
        compression_method: u8,
        filter_method: u8,
        interlace_method: u8,
    ) -> IHDR {
        IHDR {
            width,
            height,
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        }
    }
}

pub struct Png {
    ihdr: IHDR,
    filepath: String,
}

impl Png {
    pub fn new(filepath: &str) -> Png {
        println!("Parsing png at {}", filepath);
        let mut file = File::open(filepath).expect("Error reading file");

        let mut signature_buffer = [0; 8];
        file.read(&mut signature_buffer);

        println!("Png signature: ");
        util::print_buffer_as_hex(&signature_buffer);
        println!("");

        let mut ihdr_chunk_length_buffer = [0; 4];
        file.read(&mut ihdr_chunk_length_buffer);
        println!("IHDR length buffer: ");
        util::print_buffer_as_hex(&ihdr_chunk_length_buffer);
        println!("");

        let ihdr_length = util::get_decimal_sum_from_buffer(&ihdr_chunk_length_buffer);
        let mut ihdr_type_buffer = [0; 4];
        file.read(&mut ihdr_type_buffer);
        println!("IHDR type buffer: ");
        util::print_buffer_as_hex(&ihdr_type_buffer);
        println!("");

        let mut ihdr_chunk_data_buffer = vec![0; ihdr_length as usize];
        file.read(&mut ihdr_chunk_data_buffer);
        println!("IHDR data buffer: ");
        util::print_buffer_as_hex(&ihdr_chunk_data_buffer);
        util::print_buffer(&ihdr_chunk_data_buffer);

        let width = util::get_decimal_sum_from_buffer(&ihdr_chunk_data_buffer[0..4]);
        println!("Width: ");
        println!("{} px", width);

        let height = util::get_decimal_sum_from_buffer(&ihdr_chunk_data_buffer[4..8]);
        println!("Height: ");
        println!("{} px", height);

        let bit_depth = util::get_decimal_sum_from_buffer(&ihdr_chunk_data_buffer[8..9]);
        println!("Bit depth: ");
        println!("{}", bit_depth);

        let color_type = util::get_decimal_sum_from_buffer(&ihdr_chunk_data_buffer[9..10]);
        println!("Color type: ");
        println!("{}", color_type);

        let compression_method = util::get_decimal_sum_from_buffer(&ihdr_chunk_data_buffer[10..11]);
        println!("Compression method: ");
        println!("{}", compression_method);

        let filter_method = util::get_decimal_sum_from_buffer(&ihdr_chunk_data_buffer[11..12]);
        println!("Filter method: ");
        println!("{}", filter_method);

        let interlace_method = util::get_decimal_sum_from_buffer(&ihdr_chunk_data_buffer[12..13]);
        println!("Interlace method: ");
        println!("{}", interlace_method);

        Png {
            ihdr: IHDR {
                width: width as u32,
                height: height as u32,
                bit_depth: bit_depth as u8,
                color_type: color_type as u8,
                compression_method: compression_method as u8,
                filter_method: filter_method as u8,
                interlace_method: interlace_method as u8,
            },
            filepath: String::from(filepath),
        }
    }
}
