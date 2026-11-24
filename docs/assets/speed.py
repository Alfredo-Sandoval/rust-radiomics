"""Draw the README speed figure from the package comparison record.

Run from the repository root with any Python 3.13 or later:

    python3 docs/assets/speed.py

Reads benchmarks/results/pyradiomics.json and writes speed-light.svg and speed-dark.svg next to
this file.
"""

import json
from html import escape
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RECORD = ROOT / "benchmarks/results/pyradiomics.json"
OUT = Path(__file__).resolve().parent

FONT = "-apple-system,BlinkMacSystemFont,'Segoe UI',Helvetica,Arial,sans-serif"
WIDTH = 850
LEFT = 180  # start of the bars
RIGHT = 700  # end of the axis; value labels run past it
TOP = 58
ROW = 46
BAR = 18
RADIUS = 4
AXIS_MAX = 800.0
TICKS = (0, 200, 400, 600, 800)

THEMES = {
    "light": {
        "background": "#ffffff", "text": "#1f2328", "muted": "#6e7781", "grid": "#d8dee4",
        "ours": "#2a78d6", "other": "#8c959f",
    },
    "dark": {
        "background": "#0d1117", "text": "#e6edf3", "muted": "#8d96a0", "grid": "#30363d",
        "ours": "#3987e5", "other": "#6e7681",
    },
}  # fmt: skip


def bar_path(x, y, width, height, radius):
    """Return a bar anchored at x with only its right end rounded."""
    radius = min(radius, width, height / 2)
    return (
        f"M{x:.1f},{y:.1f}h{width - radius:.1f}"
        f"a{radius},{radius} 0 0 1 {radius},{radius}v{height - 2 * radius:.1f}"
        f"a{radius},{radius} 0 0 1 -{radius},{radius}h-{width - radius:.1f}z"
    )


def milliseconds(value):
    """Format a time with three significant figures."""
    return f"{value:.3g} ms" if value < 100 else f"{value:.0f} ms"


def draw(record, theme):
    """Return the SVG text for one color theme."""
    rows = record["variants"]
    baseline = rows[0]["warm_ms"]
    height = TOP + ROW * len(rows) + 44
    scale = (RIGHT - LEFT) / AXIS_MAX
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {WIDTH} {height}" '
        f'width="{WIDTH}" height="{height}" role="img">',
        f"<title>{escape(record['title'])}</title>",
        f"<style>text{{font-family:{FONT}}}"
        f".label{{font-size:14px;font-weight:600;fill:{theme['text']}}}"
        f".note{{font-size:12px;fill:{theme['muted']}}}"
        f".value{{font-size:13px;fill:{theme['text']};font-variant-numeric:tabular-nums;"
        f"paint-order:stroke;stroke:{theme['background']};stroke-width:5px;stroke-linejoin:round}}"
        f".tick{{font-size:12px;fill:{theme['muted']};text-anchor:middle;"
        f"font-variant-numeric:tabular-nums}}</style>",
        f'<rect width="{WIDTH}" height="{height}" fill="{theme["background"]}"/>',
        f'<text class="label" x="12" y="24">{escape(record["title"])}</text>',
        f'<text class="note" x="12" y="43">{escape(record["subtitle"])}</text>',
    ]
    plot_bottom = TOP + ROW * len(rows)
    for tick in TICKS:
        x = LEFT + tick * scale
        parts.append(
            f'<line x1="{x:.1f}" y1="{TOP - 4}" x2="{x:.1f}" y2="{plot_bottom}" '
            f'stroke="{theme["grid"]}" stroke-width="1"/>'
        )
        parts.append(f'<text class="tick" x="{x:.1f}" y="{plot_bottom + 18}">{tick}</text>')
    parts.append(
        f'<text class="tick" x="{(LEFT + RIGHT) / 2:.1f}" y="{plot_bottom + 36}">'
        "Warm call time (ms), lower is faster</text>"
    )
    for index, row in enumerate(rows):
        top = TOP + index * ROW
        y = top + (ROW - BAR) / 2
        width = max(row["warm_ms"] * scale, RADIUS)
        fill = theme["ours"] if row["ours"] else theme["other"]
        parts.append(f'<text class="label" x="12" y="{top + 20}">{escape(row["label"])}</text>')
        parts.append(f'<text class="note" x="12" y="{top + 37}">{escape(row["detail"])}</text>')
        parts.append(f'<path d="{bar_path(LEFT, y, width, BAR, RADIUS)}" fill="{fill}"/>')
        value = milliseconds(row["warm_ms"])
        if index > 0:
            value += f" · {baseline / row['warm_ms']:.0f}× faster than {rows[0]['label']}"
        parts.append(
            f'<text class="value" x="{LEFT + width + 8:.1f}" y="{y + BAR / 2 + 4.5:.1f}">'
            f"{escape(value)}</text>"
        )
    parts.append("</svg>")
    return "\n".join(parts) + "\n"


def main():
    record = json.loads(RECORD.read_text())
    if record["variants"][0]["warm_ms"] > AXIS_MAX:
        raise ValueError(f"the slowest warm time exceeds the {AXIS_MAX:.0f} ms axis")
    for name, theme in THEMES.items():
        (OUT / f"speed-{name}.svg").write_text(draw(record, theme))


if __name__ == "__main__":
    main()
