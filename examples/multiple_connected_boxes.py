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
        box.bounding_box.left + box.bounding_box.width // 2,
        box.bounding_box.bottom + box.bounding_box.height // 2,
    )

paths = [
    ('A', 'B', 'red'),
    ('A', 'C', 'green'),
    ('B', 'D', 'blue'),
    ('C', 'D', 'magenta'),
    ('A', 'E', 'yellow'),
    ('F', 'E', 'cyan'),
    ('E', 'D', 'bold blue'),
]

for start, end, color in paths:
    path = TextPath(coords[start], coords[end], style=color)
    objs.append(path)

print(render(*reversed(objs)))
