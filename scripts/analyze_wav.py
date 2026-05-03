#!/usr/bin/env python3
"""analyze_wav.py <wav_path> [--samples N] [--channel N] -- print readable WAV metadata and export JSON."""

import argparse
import json
import math
import os
import struct
import sys
import wave


def decode_samples(raw, sampwidth, total_samples):
    if sampwidth == 1:
        return [((sample - 128) / 128.0) for sample in raw]
    if sampwidth == 2:
        return [sample / 32768.0 for sample in struct.unpack(f"<{total_samples}h", raw)]
    if sampwidth == 3:
        samples = []
        for i in range(0, len(raw), 3):
            chunk = raw[i : i + 3]
            value = struct.unpack(
                "<i", chunk + (b"\x00" if chunk[2] < 128 else b"\xff")
            )[0]
            samples.append(value / 8388608.0)
        return samples
    if sampwidth == 4:
        try:
            return list(struct.unpack(f"<{total_samples}f", raw))
        except struct.error:
            return [sample / 2147483648.0 for sample in struct.unpack(f"<{total_samples}i", raw)]

    raise ValueError(f"unsupported sample width: {sampwidth} bytes")


def interleaved_to_channels(samples, n_channels):
    channels = [[] for _ in range(n_channels)]
    for index, sample in enumerate(samples):
        channels[index % n_channels].append(sample)
    return channels


def summarize_channel(samples):
    if not samples:
        return {
            "peak": 0.0,
            "rms": 0.0,
            "mean": 0.0,
            "min": 0.0,
            "max": 0.0,
            "zero_crossings": 0,
            "nan_count": 0,
        }

    finite_samples = [sample for sample in samples if math.isfinite(sample)]
    nan_count = len(samples) - len(finite_samples)
    working = finite_samples if finite_samples else [0.0]

    zero_crossings = 0
    for previous, current in zip(working, working[1:]):
        if (previous < 0.0 <= current) or (previous > 0.0 >= current):
            zero_crossings += 1

    return {
        "peak": max(abs(sample) for sample in working),
        "rms": math.sqrt(sum(sample * sample for sample in working) / len(working)),
        "mean": sum(working) / len(working),
        "min": min(working),
        "max": max(working),
        "zero_crossings": zero_crossings,
        "nan_count": nan_count,
    }


def format_preview(samples, limit):
    preview = samples[:limit]
    return "[" + ", ".join(f"{sample:.6f}" for sample in preview) + "]"


def build_analysis(path, preview_samples, channel_index):
    with wave.open(path, "rb") as wav_file:
        n_channels = wav_file.getnchannels()
        sampwidth = wav_file.getsampwidth()
        framerate = wav_file.getframerate()
        n_frames = wav_file.getnframes()
        comptype = wav_file.getcomptype()
        compname = wav_file.getcompname()
        raw = wav_file.readframes(n_frames)

    total_samples = n_frames * n_channels
    samples = decode_samples(raw, sampwidth, total_samples)
    channels = interleaved_to_channels(samples, n_channels)

    if channel_index is not None and not (0 <= channel_index < n_channels):
        raise ValueError(f"channel {channel_index} out of range for {n_channels} channels")

    duration_seconds = (n_frames / framerate) if framerate else 0.0

    channel_indices = [channel_index] if channel_index is not None else list(range(n_channels))
    channel_data = {}
    for index in channel_indices:
        stats = summarize_channel(channels[index])
        channel_data[f"channel_{index}"] = {
            "peak": stats["peak"],
            "rms": stats["rms"],
            "mean": stats["mean"],
            "min": stats["min"],
            "max": stats["max"],
            "zero_crossings": stats["zero_crossings"],
            "nan_or_inf_samples": stats["nan_count"],
            f"first_{preview_samples}_samples": channels[index][:preview_samples],
        }

    return {
        "path": path,
        "sample_rate": framerate,
        "channels": n_channels,
        "sample_width_bytes": sampwidth,
        "bit_depth": sampwidth * 8,
        "frames": n_frames,
        "duration_seconds": duration_seconds,
        "compression_type": comptype,
        "compression_name": compname,
        "total_samples": total_samples,
        "channel_data": channel_data,
        "interleaved_preview": {f"first_{preview_samples}_samples": samples[:preview_samples]},
    }


def print_analysis(analysis, preview_samples):
    print("WAV Analysis")
    print("============")
    print(f"path: {analysis['path']}")
    print(f"sample_rate: {analysis['sample_rate']} Hz")
    print(f"channels: {analysis['channels']}")
    print(
        f"sample_width: {analysis['sample_width_bytes']} bytes ({analysis['bit_depth']}-bit)"
    )
    print(f"frames: {analysis['frames']}")
    print(f"duration: {analysis['duration_seconds']:.6f} s")
    print(
        f"compression: {analysis['compression_type']} ({analysis['compression_name']})"
    )
    print(f"total_samples: {analysis['total_samples']}")
    print()

    for channel_name, channel_stats in analysis["channel_data"].items():
        print(f"{channel_name}:")
        print(f"  peak: {channel_stats['peak']:.6f}")
        print(f"  rms: {channel_stats['rms']:.6f}")
        print(f"  mean: {channel_stats['mean']:.6f}")
        print(f"  min: {channel_stats['min']:.6f}")
        print(f"  max: {channel_stats['max']:.6f}")
        print(f"  zero_crossings: {channel_stats['zero_crossings']}")
        print(f"  nan_or_inf_samples: {channel_stats['nan_or_inf_samples']}")
        print(
            f"  first_{preview_samples}_samples: "
            f"{format_preview(channel_stats[f'first_{preview_samples}_samples'], preview_samples)}"
        )
        print()

    print("interleaved_preview:")
    print(
        f"  first_{preview_samples}_samples: "
        f"{format_preview(analysis['interleaved_preview'][f'first_{preview_samples}_samples'], preview_samples)}"
    )


def write_json_output(analysis, wav_path, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    wav_name = os.path.splitext(os.path.basename(wav_path))[0]
    output_path = os.path.join(output_dir, f"{wav_name}.json")
    with open(output_path, "w", encoding="utf-8") as output_file:
        json.dump(analysis, output_file, indent=2)
    return output_path


def main(argv):
    parser = argparse.ArgumentParser(
        description="Print readable metadata and sample previews for a WAV file."
    )
    parser.add_argument("wav_path", help="Path to the WAV file")
    parser.add_argument(
        "--samples",
        type=int,
        default=16,
        help="Number of samples to show in each preview (default: 16)",
    )
    parser.add_argument(
        "--channel",
        type=int,
        default=None,
        help="Only print stats for one zero-based channel",
    )
    parser.add_argument(
        "--output-dir",
        default="output/analyzed-wavs",
        help="Directory for exported JSON analysis (default: output/analyzed-wavs)",
    )
    args = parser.parse_args(argv)

    if args.samples <= 0:
        print("ERROR: --samples must be > 0")
        return 1

    try:
        analysis = build_analysis(args.wav_path, args.samples, args.channel)
        print_analysis(analysis, args.samples)
        output_path = write_json_output(analysis, args.wav_path, args.output_dir)
        print()
        print(f"json_output: {output_path}")
    except Exception as exc:
        print(f"ERROR: {exc}")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
