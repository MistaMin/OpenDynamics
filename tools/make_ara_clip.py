#!/usr/bin/env python3
"""Writes the 10 s stereo test clip the ARA TestHost loads (16-bit, 48 kHz): phrases of filtered
noise and a 110 Hz tone at -32, -18 and -26 dBFS with a quiet gap. It carries an iXML chunk:
the WAV reader in the ARA SDK examples searches for one and does not stop at the end of a file
without it. Usage: make_ara_clip.py OUT.wav"""
import math, random, struct, sys

sr, n = 48000, 480000
rng = random.Random(3)
levels = [10 ** (-32 / 20), 10 ** (-18 / 20), 10 ** (-26 / 20), 10 ** (-70 / 20)]
frames = bytearray()
hist = [[0.0] * 8, [0.0] * 8]
for i in range(n):
    t = i / sr
    level = levels[int(t // 2.5) % 4]
    env = (1 - ((t * 2) % 1)) ** 3 * 0.8 + 0.2
    tone = math.sin(2 * math.pi * 110 * t) * 0.6
    for c in range(2):
        hist[c] = hist[c][1:] + [rng.gauss(0, 1)]
        x = (sum(hist[c]) / 8 * 2.0 + tone) * level * env
        frames += struct.pack('<h', max(-32768, min(32767, int(x * 32767))))
xml = b'<?xml version="1.0" encoding="UTF-8"?><BWFXML><IXML_VERSION>1.5</IXML_VERSION></BWFXML>'
if len(xml) % 2:
    xml += b' '
fmt = struct.pack('<HHIIHH', 1, 2, sr, sr * 4, 4, 16)
body = b'WAVE' + b'fmt ' + struct.pack('<I', len(fmt)) + fmt + b'data' + struct.pack('<I', len(frames)) + bytes(frames)
body += b'iXML' + struct.pack('<I', len(xml)) + xml
open(sys.argv[1], 'wb').write(b'RIFF' + struct.pack('<I', len(body)) + body)
