#!/usr/bin/env python3
"""Write the optional full-block pack's connected textures in OptiFine's `ctm_compact` form.

The original pack (`optional_fullblock_ctm`) gives each illuminant block a CTM-mod
`.png.mcmeta` (`"type": "ctm"`, version 1) and a 2x2 sheet: top-left with no border,
top-right with its left and right borders, bottom-left with its top and bottom ones,
bottom-right with only the corners. Sunrise reads OptiFine's `optifine/ctm/**.properties`,
whose `ctm_compact` takes the same five pictures as tiles 0-4: 0 every border (the block's
own texture), 1 none, 2 left and right, 3 top and bottom, 4 only the inner corners.

So for each block this splits the sheet into tiles 1-4, copies the block's texture as tile 0,
writes `<block>.properties`, and removes the CTM-mod files. Run from the repository root:
`python3 tools/gen_ctm_pack.py`. It reads the sheets from the upstream checkout given as
`--upstream` (default `../../upstream/simplylight`), so it can be run again.
"""

import argparse
import json
import os
import shutil
import struct
import zlib

PACK = "assets/simplylight/resourcepacks/optional_fullblock_ctm"
UP_PACK = "src/main/resources/optional_fullblock_ctm"
BLOCKS = "assets/simplylight/textures/block/full_block"
# The sheet's quarters (x, y) in tile order 1-4.
QUARTERS = [(0, 0), (1, 0), (0, 1), (1, 1)]


def read_png(path):
    data = open(path, "rb").read()
    pos, idat, palette, alpha = 8, b"", None, b""
    while pos < len(data):
        (size,) = struct.unpack(">I", data[pos : pos + 4])
        kind, body = data[pos + 4 : pos + 8], data[pos + 8 : pos + 8 + size]
        pos += 12 + size
        if kind == b"IHDR":
            width, height, depth, colour, _, _, interlace = struct.unpack(">IIBBBBB", body)
        elif kind == b"PLTE":
            palette = [tuple(body[i : i + 3]) for i in range(0, len(body), 3)]
        elif kind == b"tRNS":
            alpha = body
        elif kind == b"IDAT":
            idat += body
    if interlace or (depth != 8 and colour not in (0, 3)):
        raise SystemExit(f"{path}: interlaced or 16-bit PNGs are not handled")
    channels = {6: 4, 2: 3, 3: 1, 0: 1, 4: 2}[colour]
    # Filters work on whole bytes: `channels` bytes back, or one byte below 8 bits.
    raw, stride = zlib.decompress(idat), (width * channels * depth + 7) // 8
    channels = max(1, channels * depth // 8)
    rows, prev, at = [], bytearray(stride), 0
    for _ in range(height):
        kind, line = raw[at], bytearray(raw[at + 1 : at + 1 + stride])
        at += 1 + stride
        for x in range(stride):
            a = line[x - channels] if x >= channels else 0
            b = prev[x]
            c = prev[x - channels] if x >= channels else 0
            if kind == 1:
                line[x] = (line[x] + a) & 255
            elif kind == 2:
                line[x] = (line[x] + b) & 255
            elif kind == 3:
                line[x] = (line[x] + (a + b) // 2) & 255
            elif kind == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[x] = (line[x] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line))
        prev = line

    def sample(row, x):
        if depth == 8:
            return row[x]
        per = 8 // depth
        return (row[x // per] >> (8 - depth * (x % per + 1))) & ((1 << depth) - 1)

    def pixel(row, x):
        if colour == 6:
            return tuple(row[x * 4 : x * 4 + 4])
        if colour == 2:
            return tuple(row[x * 3 : x * 3 + 3]) + (255,)
        if colour == 3:
            i = sample(row, x)
            return palette[i] + (alpha[i] if i < len(alpha) else 255,)
        if colour == 4:
            return (row[x * 2],) * 3 + (row[x * 2 + 1],)
        return (sample(row, x) * 255 // ((1 << depth) - 1),) * 3 + (255,)

    return width, height, [[pixel(r, x) for x in range(width)] for r in rows]


def write_png(path, pixels):
    height, width = len(pixels), len(pixels[0])
    raw = b"".join(b"\0" + bytes(c for p in row for c in p) for row in pixels)

    def chunk(kind, body):
        return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body))

    with open(path, "wb") as out:
        out.write(b"\x89PNG\r\n\x1a\n")
        out.write(chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)))
        out.write(chunk(b"IDAT", zlib.compress(raw, 9)))
        out.write(chunk(b"IEND", b""))


def main():
    args = argparse.ArgumentParser()
    args.add_argument("--upstream", default="../../upstream/simplylight")
    upstream = os.path.join(args.parse_args().upstream, UP_PACK)
    metas = sorted(
        f
        for f in os.listdir(os.path.join(upstream, BLOCKS))
        if f.endswith(".png.mcmeta")
    )
    shutil.rmtree(os.path.join(PACK, "assets/simplylight/optifine"), ignore_errors=True)
    for meta in metas:
        block = meta.removesuffix(".png.mcmeta")
        ctm = json.load(open(os.path.join(upstream, BLOCKS, meta)))["ctm"]
        if (ctm["ctm_version"], ctm["type"]) != (1, "ctm") or len(ctm["textures"]) != 1:
            raise SystemExit(f"{meta}: not a version-1 compact sheet")
        sheet = ctm["textures"][0].split(":", 1)[1]
        width, _, pixels = read_png(os.path.join(upstream, "assets/simplylight/textures", sheet + ".png"))
        tile = width // 2
        out = os.path.join(PACK, "assets/simplylight/optifine/ctm", block)
        os.makedirs(out, exist_ok=True)
        shutil.copy(os.path.join(BLOCKS, block + ".png"), os.path.join(out, "0.png"))
        for n, (qx, qy) in enumerate(QUARTERS, start=1):
            rows = [row[qx * tile : (qx + 1) * tile] for row in pixels[qy * tile : (qy + 1) * tile]]
            write_png(os.path.join(out, f"{n}.png"), rows)
        with open(os.path.join(out, block + ".properties"), "w") as props:
            props.write(
                f"matchTiles=simplylight:block/full_block/{block}\n"
                "method=ctm_compact\n"
                "tiles=0-4\n"
            )
    shutil.rmtree(os.path.join(PACK, "assets/simplylight/textures"), ignore_errors=True)
    print(f"{len(metas)} blocks")


if __name__ == "__main__":
    main()
