use byteorder::{LittleEndian, WriteBytesExt};
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::i16::MAX;

// Constants
const SAMPLE_RATE: f32 = 44100.0;
const AMPLITUDE: f32 = MAX as f32;
const DEFAULT_DURATION: f32 = 2.0;
const DEFAULT_FREQUENCY: f32 = 440.0;
const DEFAULT_WAVE_TYPE: WaveType = WaveType::Sine;

// Defining wave types
enum WaveType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Parse duration argument, or use default if not provided
    let duration = args
        .get(1)
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(DEFAULT_DURATION);

    // Parse frequency argument, or use default if not provided
    let frequency = if let Some(s) = args.get(2) {
        match s.as_str() {
            "random" => random_frequency(),
            _ => s.parse::<f32>().unwrap_or(DEFAULT_FREQUENCY),
        }
    } else {
        DEFAULT_FREQUENCY
    };

    // Parse wave type argument, or use default if not provided
    let wave_type = args
        .get(3)
        .and_then(|s| match s.as_str() {
            "square" => Some(WaveType::Square),
            "sawtooth" => Some(WaveType::Sawtooth),
            "triangle" => Some(WaveType::Triangle),
            _ => None,
        })
        .unwrap_or(DEFAULT_WAVE_TYPE);

    let binding = String::from("output.wav");
    let output_file_name = args.get(4).unwrap_or(&binding);
        
    let mut phase = 0.0 as f32;

    let num_samples = (SAMPLE_RATE * duration) as usize;
    let mut buffer = Vec::with_capacity(num_samples * 2);

    let phase_inc = (frequency * 2.0 * std::f32::consts::PI) / SAMPLE_RATE;

    // Generate the audio samples based on the selected wave type
    for i in 0..num_samples {
        let sample = match wave_type {
            WaveType::Sine => (AMPLITUDE * (phase.sin() as f32)) as i16,
            WaveType::Square => {
                if (phase.sin() as f32) > 0.0 {
                    (AMPLITUDE) as i16
                } else {
                    (-AMPLITUDE) as i16
                }
            }
            WaveType::Sawtooth => (AMPLITUDE * (2.0 * (phase - 0.25).fract() - 1.0) as f32) as i16,
            WaveType::Triangle => {
                let phase_mod = (phase / std::f32::consts::PI).fract();
                if phase_mod < 0.5 {
                    (AMPLITUDE * (4.0 * phase_mod - 1.0) as f32) as i16
                } else {
                    (AMPLITUDE * (3.0 - 4.0 * phase_mod) as f32) as i16
                }
            }
        };
    
        buffer.write_i16::<LittleEndian>(sample).unwrap();
        phase += phase_inc;
    }
    
    let file = File::create(output_file_name).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    write_wav_header(&mut writer, num_samples as u32);
    writer.write_all(&buffer).expect("Unable to write to file");
}

// Function that writes a WAV header to a given writer
fn write_wav_header<W: Write>(writer: &mut BufWriter<W>, num_samples: u32) {
    let bytes_per_sample = 2;
    let bytes_per_second = (SAMPLE_RATE as u32) * (bytes_per_sample as u32);
    let header_size = 44;
    let data_size = num_samples * (bytes_per_sample as u32);

// Write RIFF header
writer.write_all(b"RIFF").unwrap();
writer.write_u32::<LittleEndian>(header_size + data_size).unwrap();
writer.write_all(b"WAVE").unwrap();

// Write format subchunk
writer.write_all(b"fmt ").unwrap();
writer.write_u32::<LittleEndian>(16).unwrap();
writer.write_u16::<LittleEndian>(1).unwrap();
writer.write_u16::<LittleEndian>(1).unwrap();
writer.write_u32::<LittleEndian>(SAMPLE_RATE as u32).unwrap();
writer.write_u32::<LittleEndian>(bytes_per_second).unwrap();
writer.write_u16::<LittleEndian>(bytes_per_sample).unwrap();
writer.write_u16::<LittleEndian>(8 * bytes_per_sample).unwrap();

// Write data subchunk
writer.write_all(b"data").unwrap();
writer.write_u32::<LittleEndian>(data_size).unwrap();
} fn random_frequency() -> f32 {
let mut rng = rand::thread_rng();
rng.gen_range(100.0..=1000.0)
}