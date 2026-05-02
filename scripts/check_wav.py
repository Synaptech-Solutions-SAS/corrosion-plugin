#!/usr/bin/env python3
"""check_wav.py <wav_path> -- exits 0 if WAV is non-silent, non-clipping, non-NaN, long enough."""
import sys, wave, struct, math

def check_wav(path):
    try:
        with wave.open(path, 'rb') as wf:
            n_channels = wf.getnchannels()
            sampwidth  = wf.getsampwidth()
            framerate  = wf.getframerate()
            n_frames   = wf.getnframes()
            raw        = wf.readframes(n_frames)
    except Exception as e:
        print(f"ERROR opening {path}: {e}")
        return False

    duration_s = n_frames / framerate
    if duration_s < 0.1:
        print(f"FAIL: too short ({duration_s:.3f}s < 0.1s)")
        return False

    total_samples = n_frames * n_channels
    if sampwidth == 2:
        fmt = f"<{total_samples}h"
        samples = [s / 32768.0 for s in struct.unpack(fmt, raw)]
    elif sampwidth == 3:
        samples = []
        for i in range(0, len(raw), 3):
            val = struct.unpack('<i', raw[i:i+3] + (b'\x00' if raw[i+2] < 128 else b'\xff'))[0]
            samples.append(val / 8388608.0)
    elif sampwidth == 4:
        fmt = f"<{total_samples}f"
        samples = list(struct.unpack(fmt, raw))
    else:
        print(f"FAIL: unsupported sample width {sampwidth}")
        return False

    nan_count = sum(1 for s in samples if math.isnan(s) or math.isinf(s))
    peak      = max(abs(s) for s in samples)
    rms       = math.sqrt(sum(s*s for s in samples) / len(samples)) if samples else 0.0

    print(f"peak={peak:.6f} rms={rms:.6f} nan_count={nan_count} frames={n_frames} sr={framerate}")

    if nan_count > 0:
        print(f"FAIL: {nan_count} NaN/inf samples")
        return False
    if peak < 0.01:
        print(f"FAIL: peak {peak:.6f} < 0.01 (silent)")
        return False
    if peak > 0.9999:
        print(f"FAIL: peak {peak:.6f} > 0.9999 (hard clipping)")
        return False
    return True

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: check_wav.py <wav_path>")
        sys.exit(1)
    sys.exit(0 if check_wav(sys.argv[1]) else 1)
