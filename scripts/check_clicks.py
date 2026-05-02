#!/usr/bin/env python3
"""check_clicks.py <wav_path> -- exits 0 if no clicks (large sample-to-sample deltas) detected."""
import sys, wave, struct, math

def check_clicks(path, max_delta=0.5):
    try:
        with wave.open(path, 'rb') as wf:
            sampwidth  = wf.getsampwidth()
            n_channels = wf.getnchannels()
            n_frames   = wf.getnframes()
            raw        = wf.readframes(n_frames)
    except Exception as e:
        print(f"ERROR opening {path}: {e}")
        return False

    total_samples = n_frames * n_channels
    if sampwidth == 2:
        samples = [s / 32768.0 for s in struct.unpack(f"<{total_samples}h", raw)]
    elif sampwidth == 4:
        samples = list(struct.unpack(f"<{total_samples}f", raw))
    else:
        # For 3-byte, approximate
        samples = []
        for i in range(0, len(raw), 3):
            val = struct.unpack('<i', raw[i:i+3] + (b'\x00' if raw[i+2] < 128 else b'\xff'))[0]
            samples.append(val / 8388608.0)

    max_d = 0.0
    click_count = 0
    for i in range(1, len(samples)):
        d = abs(samples[i] - samples[i-1])
        if d > max_d:
            max_d = d
        if d > max_delta:
            click_count += 1

    print(f"max_delta={max_d:.6f} click_count={click_count}")
    if click_count > 0:
        print(f"FAIL: {click_count} clicks detected (delta > {max_delta})")
        return False
    return True

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: check_clicks.py <wav_path>")
        sys.exit(1)
    sys.exit(0 if check_clicks(sys.argv[1]) else 1)
