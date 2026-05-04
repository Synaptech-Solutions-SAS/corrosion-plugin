use clap::Parser;
use corrotion::offline::RenderConfig;
use corrotion::presets::Preset;
use corrotion::voice::Voice;
use corrotion::Object;
use std::fs;
use std::path::Path;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    preset: String,
    #[arg(long)]
    out: String,
    #[arg(long, default_value = "60")]
    note: u8,
    #[arg(long, default_value = "2")]
    duration: f32,
}

fn float_sample_to_pcm_i16(sample: f32) -> i16 {
    let clamped = sample.clamp(-1.0, 1.0);
    (clamped * i16::MAX as f32) as i16
}

fn write_wav_i16(path: &Path, samples: &[f32], sample_rate: u32) -> std::io::Result<()> {
    let data_bytes: Vec<u8> = samples
        .iter()
        .flat_map(|sample| {
            let pcm = float_sample_to_pcm_i16(*sample);
            [(pcm & 0xFF) as u8, ((pcm >> 8) & 0xFF) as u8]
        })
        .collect();

    let data_len = data_bytes.len() as u32;
    let chunk_size = 36 + data_len;
    let byte_rate = sample_rate * 2;

    let mut wav = Vec::with_capacity(44 + data_bytes.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&chunk_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    wav.extend_from_slice(&data_bytes);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, wav)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let preset = Preset::load(&args.preset)?;

    let sample_rate = RenderConfig::default().sample_rate;
    let frame_count = (sample_rate as f32 * args.duration) as usize;
    let mut voice = Voice::new();
    let profile = match preset.object {
        Object::Pipe => corrotion::dsp::ModalProfileId::Pipe,
        Object::Plate => corrotion::dsp::ModalProfileId::Plate,
        Object::Tank => corrotion::dsp::ModalProfileId::Tank,
        Object::Chain => corrotion::dsp::ModalProfileId::Chain,
    };

    voice.note_on(
        args.note,
        127.0,
        profile,
        0,
        preset.size,
        preset.rust,
        preset.damage,
        0,
    );

    let mut output = Vec::with_capacity(frame_count);
    for _ in 0..frame_count {
        let mut sample = voice.process_sample(sample_rate);
        sample = (sample * (1.0 + preset.drive * 3.0)).tanh();
        sample *= preset.output;
        output.push(sample.clamp(-1.0, 1.0));
    }

    write_wav_i16(Path::new(&args.out), &output, sample_rate)?;
    println!("Wrote {}", args.out);
    Ok(())
}
