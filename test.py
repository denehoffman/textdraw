from textdraw import Pixel, Group, render

# Pixels with different background and foreground styles
pixels = [
    Pixel('A', (0, 0), style='red'),
    Pixel('B', (1, 0), style='on blue'),
    Pixel('C', (2, 0), style='green on yellow'),
]

# Apply different group-level style overrides
groups = [
    Group([p.at((i, 1)) for i, p in enumerate(pixels)], style='on white'),  # override bg only
    Group([p.at((i, 2)) for i, p in enumerate(pixels)], style='blink'),  # add effect
    Group([p.at((i, 3)) for i, p in enumerate(pixels)], style='red'),  # override fg only
]

# Render top row without group override, and subsequent rows with them
print(render(*pixels, *groups))
