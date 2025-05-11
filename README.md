<!-- markdownlint-disable MD033 MD041 -->
<p align="center">
  <h1 align="center">textdraw</h1>
</p>
<p align="center">
    <img alt="GitHub Release" src="https://img.shields.io/github/v/release/denehoffman/textdraw?style=for-the-badge&logo=github"></a>
  <a href="https://github.com/denehoffman/textdraw/commits/main/" alt="Latest Commits">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/denehoffman/textdraw?style=for-the-badge&logo=github"></a>
  <a href="LICENSE-APACHE" alt="License">
    <img alt="GitHub License" src="https://img.shields.io/github/license/denehoffman/textdraw?style=for-the-badge"></a>
  <a href="https://pypi.org/project/textdraw/" alt="View project on PyPI">
  <img alt="PyPI - Version" src="https://img.shields.io/pypi/v/textdraw?style=for-the-badge&logo=python&logoColor=yellow&labelColor=blue"></a>
</p>

`textdraw` is a Python library for drawing styled Unicode boxes and diagrams
using [`rich`](https://github.com/Textualize/rich). Paths between points
can be generated via an A* path-finding algorithm, and text objects can be
composed to create complex layouts.

<!--toc:start-->
- [Features](#features)
- [Installation](#installation)
- [Examples](#examples)
  - [Boxed Hello World](#boxed-hello-world)
  - [Connecting boxes](#connecting-boxes)
  - [Multiple connected boxes](#multiple-connected-boxes)
  - [A Complex Example](#a-complex-example)
- [Future Plans](#future-plans)
- [Contributing](#contributing)
<!--toc:end-->

## Features

- Unicode box-drawing with `light`, `heavy`, and `double` borders
- Styled text rendering via `rich`
- Layout composition using `TextPanel`s
- Automatic path-finding with bend and group penalties
- Flexible padding and justification for text boxes
- Support for cleanly merging path intersections

## Installation

```shell
pip install textdraw
```

Or with `uv`:

```shell
uv pip install textdraw
```

## Examples

### Boxed Hello World

```python
from textdraw import TextBox, BorderType
from rich import print

box = TextBox.from_string(
    "Hello, world!",
    border_type=BorderType.DOUBLE,
    border_style="bold blue",
    style="italic",
    padding=(1, 2, 1, 2),
)

print(box)
```

<p align="center">
  <img
    width="300"
    src="media/hello-world.png"
    alt="Boxed Hello World result"
  />
</p>

### Connecting boxes

```python
from textdraw import TextPanel, TextBox, BorderType
from rich import print

panel = TextPanel()

a = TextBox.from_string("A", border_style="green")
panel.add_object(a, 0, 0)
b = TextBox.from_string("B", border_style="red")
panel.add_object(b, 20, 10)

# Automatically route a connecting line
path = panel.connect(
    (a.width, a.height - 1),
    (19, 10),
    border_type=BorderType.LIGHT,
    style="dim",
    start_char="",
    start_style="red",
    end_char="◼",
    end_style="green",
)
panel.add_object(path, 0, 0)

print(panel)
```

<p align="center">
  <img
    width="300"
    src="media/connected-boxes.png"
    alt="Connecting boxes result"
  />
</p>

### Multiple connected boxes

```python
from textdraw import TextPanel, TextBox, BorderType
from rich import print

panel = TextPanel()

boxes = {
    "A": (0, 0),
    "B": (30, 0),
    "C": (0, 8),
    "D": (30, 8),
    "E": (15, 4),
    "F": (15, 12),
}

coords = {}
for label, (x, y) in boxes.items():
    box = TextBox.from_string(
        label,
        border_type=BorderType.HEAVY,
        border_style="bold white",
        style="bold",
        padding=(0, 1, 0, 1),
    )
    panel.add_object(box, x, y)
    coords[label] = (x + box.width // 2, y + box.height // 2)

paths = [
    ("A", "B", "red"),
    ("A", "C", "green"),
    ("B", "D", "blue"),
    ("C", "D", "magenta"),
    ("A", "E", "yellow"),
    ("F", "E", "cyan"),
    ("E", "D", "bright_blue"),
]

new_panel = TextPanel()
for start, end, color in paths:
    path = panel.connect(
        coords[start],
        coords[end],
        border_type=BorderType.LIGHT,
        style=color,
    )
    panel.add_object(path, 0, 0)
    new_panel.add_object(path, 0, 0)

# We use the new panel to draw the boxes over the paths
for label, (x, y) in boxes.items():
    box = TextBox.from_string(
        label,
        border_type=BorderType.HEAVY,
        border_style="bold white",
        style="bold",
        padding=(0, 1, 0, 1),
    )
    new_panel.add_object(box, x, y)

print(new_panel)
```

<p align="center">
  <img
    width="300"
    src="media/multiple-connected-boxes.png"
    alt="Multiple connecting boxes result"
  />
</p>

### A Complex Example

```python
from typing import override
from textdraw import (
    AbstractTextObject,
    BorderType,
    StyledChar,
    TextObject,
    TextPanel,
    TextBox,
)
from rich import print


class LetterBox(TextObject):
    def __init__(self, letter: str, x: int, y: int):
        super().__init__(penalty_group="letterbox")
        self.box = TextBox.from_string(letter)
        self.x = x
        self.y = y
        self.c_left = (x - 1, y + self.box.height // 2)
        self.c_right = (x + self.box.width, y + self.box.height // 2)
        self.c_top = (x + self.box.width // 2, y - 1)
        self.c_bottom = (x + self.box.width // 2, y + self.box.height)

    @property
    @override
    def chars(self) -> list[StyledChar]:
        panel = TextPanel([(self.box, self.x, self.y)])
        barrier = TextObject.from_string(" ")
        panel.add_object(barrier, self.c_left[0], self.c_left[1] - 1)
        panel.add_object(barrier, self.c_left[0], self.c_left[1] + 1)
        panel.add_object(barrier, self.c_right[0], self.c_right[1] - 1)
        panel.add_object(barrier, self.c_right[0], self.c_right[1] + 1)
        panel.add_object(barrier, self.c_top[0] - 1, self.c_top[1])
        panel.add_object(barrier, self.c_top[0] + 1, self.c_top[1])
        panel.add_object(barrier, self.c_bottom[0] - 1, self.c_bottom[1])
        panel.add_object(barrier, self.c_bottom[0] + 1, self.c_bottom[1])
        return panel.chars


a = LetterBox("a", 0, 0)
b = LetterBox("b", 20, 8)
c = LetterBox("c", 3, 10)

panel = TextPanel([a, b, c])
group_penalties = {"letterbox": 100, "line": 10}
panel.add_object(
    panel.connect(
        a.c_right,
        b.c_top,
        style="dim",
        group_penalties=group_penalties,
    ).with_penalty_group("line"),
)
panel.add_object(
    panel.connect(
        a.c_bottom,
        b.c_left,
        style="green",
        group_penalties=group_penalties,
    ).with_penalty_group("line"),
)
panel.add_object(
    panel.connect(
        a.c_left,
        c.c_top,
        style="blue",
        group_penalties=group_penalties,
    ).with_penalty_group("line"),
)
panel.add_object(
    panel.connect(
        b.c_bottom,
        c.c_left,
        style="red",
        border_type=BorderType.DOUBLE,
        group_penalties=group_penalties,
    ).with_penalty_group("line"),
)
panel.add_object(
    panel.connect_many(
        [c.c_bottom, b.c_left, a.c_top],
        [a.c_right, c.c_right, b.c_right],
        style="yellow",
        border_type=BorderType.HEAVY,
        start_char="",
        group_penalties=group_penalties,
    ).with_penalty_group("line"),
)
print(panel)
```

<p align="center">
  <img
    width="300"
    src="media/letterbox.png"
    alt="letterbox result"
  />
</p>

## Future Plans

This project was mostly a tool I wanted to create for a graph-drawing project.
However, there are some features that would be beneficial:

- The ability to move objects around inside a `TextPanel`
- A way to force the final direction of a path to bend in a certain way (right
  now, Unicode characters like `┌` cannot be inserted as the last character of
  a path unless they are added so manually)
- A simpler interface to add obstacles and intermediate points into the
  path-finding algorithm
- Proper calculation of the distance metric for diagonal paths (right now a
  diagonal path is an L-shape which takes three characters when the distance
  cost should evaluate to `sqrt(2)`)
- Combination characters like `╤` to combine different path styles or connect
  paths with boxes directly (the latter can be done but only manually)

## Contributing

I'm open to any contributions. Please create an issue and/or pull request,
I'll try to respond quickly.
