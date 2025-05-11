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
