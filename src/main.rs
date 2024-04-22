use std::{fs::OpenOptions, io::{self, Read, Seek, SeekFrom, Write}};
use std::mem;
use std::{fs::File};

#[repr(C)]
#[derive(Debug)]
struct WavHeader {
    chunk_id: String,
    chunk_size: u32,
    format: [u8; 4],
    fmt_subchunk_id: [u8; 4],
    fmt_subchunk_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    data_subchunk_id: [u8; 4],
    data_subchunk_size: u32,
}

fn parse_wav_header(buffer: &[u8]) -> io::Result<WavHeader> {
    if buffer.len() < 44 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Buffer too small"));
    }
    Ok(WavHeader {
        chunk_id: String::from_utf8_lossy(&buffer[0..4]).to_string(),
        chunk_size: u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]),
        format: [buffer[8], buffer[9], buffer[10], buffer[11]],
        fmt_subchunk_id: [buffer[12], buffer[13], buffer[14], buffer[15]],
        fmt_subchunk_size: u32::from_le_bytes([buffer[16], buffer[17], buffer[18], buffer[19]]),
        audio_format: u16::from_le_bytes([buffer[20], buffer[21]]),
        num_channels: u16::from_le_bytes([buffer[22], buffer[23]]),
        sample_rate: u32::from_le_bytes([buffer[24], buffer[25], buffer[26], buffer[27]]),
        byte_rate: u32::from_le_bytes([buffer[28], buffer[29], buffer[30], buffer[31]]),
        block_align: u16::from_le_bytes([buffer[32], buffer[33]]),
        bits_per_sample: u16::from_le_bytes([buffer[34], buffer[35]]),
        data_subchunk_id: [buffer[36], buffer[37], buffer[38], buffer[39]],
        data_subchunk_size: u32::from_le_bytes([buffer[40], buffer[41], buffer[42], buffer[43]]),
    })
}

fn find_data_chunk(mut file: &File) -> io::Result<u32> {
    let mut buffer = [0; 8];

    file.seek(SeekFrom::Start(44))?;

    loop {
        if file.read_exact(&mut buffer).is_err() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Reached EOF without finding subchunk"));
        }
        let subchunk_id = String::from_utf8_lossy(&buffer[0..4]);
        if subchunk_id == "data" {
            return Ok(u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]));
        } else {
            let subchunk_size = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
            file.seek(SeekFrom::Current(subchunk_size as i64))?;
        }
    }
}

fn main() {
    println!("Starting slowverber!");

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("./song.wav")
        .expect("Failed to open file");

    let mut header_buff: [u8; 44] = [0; 44];
    file.read_exact(&mut header_buff).expect("Failed to read the WAV header");

    let wav_header = parse_wav_header(&header_buff).expect("Failed to parse WAV header");
    println!("WAV Header: {:?}", wav_header);

    match find_data_chunk(&file) {
        Ok(size) => println!("Size of 'data' subchunk: {} bytes", size),
        Err(e) => println!("Error finding 'data' subchunk: {}", e),
    }

    file.seek(SeekFrom::Start(0)).expect("Failed to seek to start");
    let fmt = "YESS";

    // file.seek(SeekFrom::Start(25));
    // let sample_rate
    // file.write_all(fmt.as_bytes()).expect("Failed to write data");
    // file.flush().expect("Failed to flush");

    println!("Ending slowverber!");
}
