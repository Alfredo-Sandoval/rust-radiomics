"""Draw the README figure of the IBSI digital phantom and one of its voxel maps.

Run from the repository root with the project Python:

    crates/radiomics-py/.venv/bin/python docs/assets/phantom.py

Writes phantom-light.svg and phantom-dark.svg next to this file.
"""

from pathlib import Path

import nibabel as nib
import numpy as np
from rustradiomics import Settings, extract_files

ROOT = Path(__file__).resolve().parents[2]
DATA = ROOT / "crates/radiomics/tests/data/w12"
IMAGE = DATA / "digital_phantom_image.nii.gz"
MASK = DATA / "digital_phantom_mask.nii.gz"
OUT = Path(__file__).resolve().parent

FEATURE = "glcm_JointEntropy"
MAP_MAX = 3.0

EDGE = 24.0  # cube edge in pixels
CX = EDGE * np.cos(np.pi / 6)
CY = EDGE * 0.5
SLAB_GAP = 26.0
ROW_GAP = 46.0
MARGIN = 12.0
LABEL_HEIGHT = 26.0
COLORBAR_HEIGHT = 34.0
FONT = "-apple-system,BlinkMacSystemFont,'Segoe UI',Helvetica,Arial,sans-serif"

# Viridis at 11 evenly spaced points.
VIRIDIS = (
    "#440154", "#482475", "#414487", "#355f8d", "#2a788e", "#21918c",
    "#22a884", "#44bf70", "#7ad151", "#bddf26", "#fde725",
)  # fmt: skip

THEMES = {
    "light": {"background": "#ffffff", "text": "#1f2328", "muted": "#6e7781", "outside": "#a8cdf0"},
    "dark": {"background": "#0d1117", "text": "#e6edf3", "muted": "#8d96a0", "outside": "#7fb2e5"},
}


def load():
    """Return the image, mask and voxel map as (z, y, x) arrays."""
    image = np.asarray(nib.load(IMAGE).dataobj).transpose(2, 1, 0)
    mask = np.asarray(nib.load(MASK).dataobj).transpose(2, 1, 0) > 0
    settings = Settings(bin_width=1, features=(FEATURE,))
    result = extract_files(IMAGE, MASK, mode="voxel", settings=settings)
    voxel_map = np.asarray(result.features[f"original_{FEATURE}"])
    if voxel_map.shape != image.shape:
        raise ValueError(f"map shape {voxel_map.shape} does not match image {image.shape}")
    return image, mask, voxel_map


def tag(name, content="", **attributes):
    """Return an SVG element. Attribute names use _ for -, and class_ for class."""
    opening = " ".join(
        [name]
        + [f'{key.rstrip("_").replace("_", "-")}="{value}"' for key, value in attributes.items()]
    )
    return f"<{opening}>{content}</{name}>" if content else f"<{opening}/>"


def hex_to_rgb(color):
    return np.array([int(color[i : i + 2], 16) for i in (1, 3, 5)], dtype=float)


def rgb_to_hex(rgb):
    return "#" + "".join(f"{int(round(c)):02x}" for c in np.clip(rgb, 0, 255))


def viridis(t):
    t = float(np.clip(t, 0.0, 1.0)) * (len(VIRIDIS) - 1)
    low = int(np.floor(t))
    high = min(low + 1, len(VIRIDIS) - 1)
    weight = t - low
    return rgb_to_hex(hex_to_rgb(VIRIDIS[low]) * (1 - weight) + hex_to_rgb(VIRIDIS[high]) * weight)


def gray(value):
    """Intensity 1 to 9 as a gray, brighter for higher values as on a CT."""
    level = 0.28 + 0.66 * (value - 1) / 8
    return rgb_to_hex(np.full(3, 255 * level))


def shade(color, factor):
    return rgb_to_hex(hex_to_rgb(color) * factor)


def ink(color):
    r, g, b = hex_to_rgb(color) / 255
    return "#111111" if 0.2126 * r + 0.7152 * g + 0.0722 * b > 0.45 else "#ffffff"


def slab_size(nx, ny):
    return (nx + ny) * CX, (nx + ny) * CY + EDGE


def draw_slab(values, colors, inside, origin, theme, show_values):
    """Draw one slice as a 1-voxel-thick slab of cubes; origin is the slab's top corner.

    With show_values, voxels outside the mask are drawn in the theme's outside color with
    their intensity. Without it, they are left out, because the map has no value there.
    """
    ox, oy = origin
    ny, nx = values.shape

    def point(x, y, z):
        return ox + (x - y) * CX, oy + (x + y) * CY - z * EDGE + EDGE

    def face(*corners):
        return " ".join("{:.1f},{:.1f}".format(*point(*corner)) for corner in corners)

    parts = []
    for j, i in sorted(np.ndindex(ny, nx), key=lambda ji: ji[0] + ji[1]):
        if not inside[j, i] and not show_values:
            continue
        color = colors[j, i] if inside[j, i] else theme["outside"]
        top = face((i, j, 1), (i + 1, j, 1), (i + 1, j + 1, 1), (i, j + 1, 1))
        left = face((i, j + 1, 1), (i + 1, j + 1, 1), (i + 1, j + 1, 0), (i, j + 1, 0))
        right = face((i + 1, j, 1), (i + 1, j + 1, 1), (i + 1, j + 1, 0), (i + 1, j, 0))
        parts.append(tag("polygon", class_="cube", points=top, fill=color))
        parts.append(tag("polygon", class_="cube", points=left, fill=shade(color, 0.82)))
        parts.append(tag("polygon", class_="cube", points=right, fill=shade(color, 0.66)))
        if show_values:
            x, y = point(i + 0.5, j + 0.5, 1)
            text = f"{values[j, i]:g}"
            parts.append(
                tag("text", text, class_="value", x=f"{x:.1f}", y=f"{y + 4:.1f}", fill=ink(color))
            )
    return parts


def colorbar(right, bottom):
    width, height = 180.0, 8.0
    x, y = right - width, bottom - COLORBAR_HEIGHT + 6
    stops = "".join(
        tag("stop", offset=f"{i / (len(VIRIDIS) - 1):.2f}", stop_color=color)
        for i, color in enumerate(VIRIDIS)
    )
    parts = [
        f"<defs>{tag('linearGradient', stops, id='viridis')}</defs>",
        tag(
            "text",
            "Joint entropy (bits)",
            class_="note",
            x=f"{x - 10:.1f}",
            y=f"{y + 8:.1f}",
            text_anchor="end",
        ),
        tag(
            "rect",
            x=f"{x:.1f}",
            y=f"{y:.1f}",
            width=f"{width:.0f}",
            height=f"{height:.0f}",
            fill="url(#viridis)",
        ),
    ]
    for tick in range(int(MAP_MAX) + 1):
        tick_x = x + width * tick / MAP_MAX
        parts.append(
            tag(
                "text",
                str(tick),
                class_="note",
                x=f"{tick_x:.1f}",
                y=f"{y + height + 14:.1f}",
                text_anchor="middle",
            )
        )
    return parts


def figure(image, mask, voxel_map, theme):
    nz, ny, nx = image.shape
    slab_w, slab_h = slab_size(nx, ny)
    row_h = LABEL_HEIGHT + slab_h
    width = 2 * MARGIN + nz * slab_w + (nz - 1) * SLAB_GAP
    height = 2 * MARGIN + 2 * row_h + ROW_GAP + COLORBAR_HEIGHT

    gray_colors = np.vectorize(gray, otypes=[object])(image)
    map_colors = np.vectorize(lambda v: viridis(v / MAP_MAX), otypes=[object])(voxel_map)
    style = (
        f"text{{font-family:{FONT}}}"
        f".label{{font-size:14px;font-weight:600;fill:{theme['text']}}}"
        f".note{{font-size:12px;fill:{theme['muted']}}}"
        ".value{font-size:12px;text-anchor:middle;font-variant-numeric:tabular-nums}"
        f".cube{{stroke:{theme['background']};stroke-width:1;stroke-linejoin:round}}"
    )
    parts = [tag("style", style)]

    rows = (
        ("Image", "Blue voxels are outside the mask", image, gray_colors, True),
        ("Voxel map", "GLCM joint entropy in the 3 × 3 × 3 neighborhood of each voxel",
         voxel_map, map_colors, False),
    )  # fmt: skip
    for row, (label, note, values, colors, show_values) in enumerate(rows):
        top = MARGIN + row * (row_h + ROW_GAP)
        heading = label + tag("tspan", note, class_="note", dx="10")
        parts.append(tag("text", heading, class_="label", x=f"{MARGIN:.0f}", y=f"{top + 15:.0f}"))
        for k in range(nz):
            left = MARGIN + k * (slab_w + SLAB_GAP)
            origin = (left + ny * CX, top + LABEL_HEIGHT)
            parts.extend(draw_slab(values[k], colors[k], mask[k], origin, theme, show_values))
            if row == 0:
                x, y = left + slab_w / 2, top + row_h + 22
                parts.append(
                    tag(
                        "text",
                        f"Slice {k + 1}",
                        class_="note",
                        x=f"{x:.1f}",
                        y=f"{y:.1f}",
                        text_anchor="middle",
                    )
                )

    parts.extend(colorbar(width - MARGIN, height - MARGIN))
    size = {"width": f"{width:.0f}", "height": f"{height:.0f}"}
    viewbox = f"0 0 {size['width']} {size['height']}"
    return (
        tag(
            "svg",
            "\n" + "\n".join(parts) + "\n",
            xmlns="http://www.w3.org/2000/svg",
            viewBox=viewbox,
            **size,
        )
        + "\n"
    )


def main():
    image, mask, voxel_map = load()
    for name, theme in THEMES.items():
        (OUT / f"phantom-{name}.svg").write_text(figure(image, mask, voxel_map, theme))


if __name__ == "__main__":
    main()
