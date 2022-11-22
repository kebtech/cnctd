use anyhow::anyhow;
use audiopus::{
    coder::{Decoder, Encoder},
    packet::Packet,
    Application, Bitrate, Channels, Error as OpusError, ErrorCode as OpusErrorCode, MutSignals,
    SampleRate,
};
use crate::recorder::AudioClip;
use itertools::interleave;

// pub fn encode(clip: &AudioClip) -> Result<(u32, Vec<u8>)> {
//     let sample_rate: i32 = clip.sample_rate.try_into()?;
//     let resampled: AudioClip;
//     let (samples_l, samples_r, sample_rate) = match SampleRate::try_from(sample_rate) {
//         Ok(sample_rate) => (&clip.samples, sample_rate),
//         Err(_) => {
//             resampled = clip.resample(48000);
//             (&resampled.samples_l, &resampled.samples_r)
//         }
//     };
//     let mut encoder = Encoder::new(sample_rate, Channels::Stereo, Application::Audio);
//     encoder.set_bitrate(Bitrate::BitsPerSecond(192000));

//     let frame_size = (sample_rate as i32 / 1000 * 20) as usize;

//     let mut output_l = vec![0u8; samples_l.len()];
//     let mut output_r = vec![0u8; samples_r.len()];
//     let mut samples_i_l = 0;
//     let mut samples_i_r = 0;
//     let mut output_i_l = 0;
//     let mut output_i_l = 0;
//     let mut end_buffer = vec![0f32; frame_size];

   
// }

pub fn encode(clip: &AudioClip) -> Result<(u32, Vec<u8>), anyhow::Error> {
    let sample_rate: i32 = clip.sample_rate.try_into()?;
    let resampled: AudioClip;
    let (samples, sample_rate) = match SampleRate::try_from(sample_rate) {
        Ok(sample_rate) => (&clip.samples, clip.sample_rate),
        Err(_) => {
            resampled = clip.resample(48000);
            (&resampled.samples, resampled.sample_rate)
        }
    };
    // let mut samples: Vec<f32> = vec![];
    // for (i, sample) in samples_l.iter().enumerate() {
    //     samples.push(*sample);
    //     if i == samples_l.len() - 1 { break }
    // }
    // for (i, sample) in samples_r.iter().enumerate() {
    //     samples.insert(i + 1, *sample);
    //     if i == samples_r.len() - 1 { break }
    // }

    let mut encoder = Encoder::new(SampleRate::Hz48000, Channels::Stereo, Application::Audio)?;
    encoder.set_bitrate(Bitrate::BitsPerSecond(192000))?;


    let frame_size = (sample_rate as i32 / 1000 * 20) as usize;

    let mut output = vec![0u8; samples.len().max(128)];
    let mut samples_i = 0;
    let mut output_i = 0;
    let mut end_buffer = vec![0f32; frame_size];

    // Store number of samples.
    {
        let samples: u32 = samples.len().try_into()?;
        let bytes = samples.to_be_bytes();
        output[..4].clone_from_slice(&bytes[..4]);
        output_i += 4;
    }

    while samples_i < samples.len() {
        match encoder.encode_float(
            if samples_i + frame_size < samples.len() {
                &samples[samples_i..(samples_i + frame_size)]
            } else {
                end_buffer[..(samples.len() - samples_i)].clone_from_slice(
                    &samples[samples_i..((samples.len() - samples_i) + samples_i)],
                );

                &end_buffer
            },
            &mut output[output_i + 2..],
        ) {
            Ok(pkt_len) => {
                samples_i += frame_size;
                let bytes = u16::try_from(pkt_len)?.to_be_bytes();
                output[output_i] = bytes[0];
                output[output_i + 1] = bytes[1];
                output_i += pkt_len + 2;
            }
            Err(OpusError::Opus(OpusErrorCode::BufferTooSmall)) => {
                println!(
                    "Needed to increase buffer size, opus is compressing less well than expected."
                );
                output.resize(output.len() * 2, 0u8);
            }
            Err(e) => {
                return Err(anyhow!(e));
            }
        }
    }

    output.truncate(output_i);

    Ok((sample_rate as i32 as u32, output))
}