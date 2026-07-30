#!/usr/bin/env python3
"""
Crops a run.py screenshot around the character.

The temporal-shot scenario returns the character's internal-resolution box and
the canvas geometry; this maps that box onto the page screenshot and cuts a
padded, upscaled crop so a report shows the character rather than a whole dark
room.

    python3 tools/headless/crop_shot.py shot.png result.json crop.png --pad 24 --scale 3
"""
import argparse
import json

from PIL import Image


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("shot")
    ap.add_argument("result", help="run.py --json output; its scenario blob has box/canvas")
    ap.add_argument("out")
    ap.add_argument("--pad", type=int, default=24, help="padding in internal pixels")
    ap.add_argument("--scale", type=int, default=3, help="nearest-neighbour upscale")
    args = ap.parse_args()

    sc = json.load(open(args.result))["scenario"]
    box, ren, cv = sc["box"], sc["render"], sc["canvas"]
    sx, sy = cv["cssW"] / ren["w"], cv["cssH"] / ren["h"]
    x0 = cv["cssX"] + (box["x0"] - args.pad) * sx
    y0 = cv["cssY"] + (box["y0"] - args.pad) * sy
    x1 = cv["cssX"] + (box["x1"] + args.pad) * sx
    y1 = cv["cssY"] + (box["y1"] + args.pad) * sy
    img = Image.open(args.shot)
    x0, y0 = max(0, int(x0)), max(0, int(y0))
    x1, y1 = min(img.width, int(x1)), min(img.height, int(y1))
    crop = img.crop((x0, y0, x1, y1))
    if args.scale > 1:
        crop = crop.resize((crop.width * args.scale, crop.height * args.scale), Image.NEAREST)
    crop.save(args.out)
    print(f"{args.out}: {crop.width}x{crop.height} from ({x0},{y0})-({x1},{y1})")


if __name__ == "__main__":
    main()
