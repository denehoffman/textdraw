from textdraw import Box, TextPath, render


boxes = {
    'A': (0, 0),
    'B': (30, 0),
    'C': (0, -8),
    'D': (30, -8),
    'E': (15, -4),
    'F': (15, -12),
}
objs = []
coords = {}
for label, (x, y) in boxes.items():
    box = Box(label, (x, y), border_style='bold white', style='bold', line_style='thick')
    objs.append(box)
    coords[label] = (
        box.bbox.left + box.bbox.width // 2,
        box.bbox.bottom + box.bbox.height // 2,
    )

paths = [
    ('A', 'B', 'red'),
    ('A', 'C', 'green'),
    ('B', 'D', 'blue'),
    ('C', 'D', 'magenta'),
    ('A', 'E', 'yellow'),
    ('F', 'E', 'cyan'),
    ('E', 'D', 'bright_blue'),
]

for start, end, color in paths:
    path = TextPath(coords[start], coords[end], style=color, bend_penalty=0)
    objs.append(path)

print(render(*reversed(objs)))
