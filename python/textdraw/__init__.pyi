from typing import Literal
from typing import Self


class BoundingBox:
    top: int
    right: int
    bottom: int
    left: int

    def __init__(self, top: int, right: int, bottom: int, left: int): ...
    def __add__(self, other: tuple[int, int] | Self) -> Self: ...
    def __contains__(self, other: tuple[int, int] | Self) -> bool: ...
    @property
    def width(self) -> int: ...
    @property
    def height(self) -> int: ...


class Group:
    pixels: list[Pixel]
    position: tuple[int, int]
    style: Style
    weight: int

    def __init__(
        self,
        pixels: list[Pixel],
        position: tuple[int, int] | None = None,
        style: str | None = None,
        weight: int | None = None,
    ): ...
    def at(self, position: tuple[int, int]) -> Self: ...
    def __getitem__(self, index: int) -> Pixel: ...
    def __setitem__(self, index: int, pixel: Pixel) -> None: ...


class Pixel:
    character: str
    position: tuple[int, int]
    style: Style
    weight: int

    def __init__(
        self,
        character: str,
        position: tuple[int, int] | None = None,
        style: str | None = None,
        *,
        weight: int | None = None,
    ): ...
    def at(self, position: tuple[int, int]) -> Self: ...


class Style:
    def __init__(self, style: str): ...
    def __add__(self, other: str | Self) -> Self: ...
    def __call__(self, text: str) -> str: ...


def render(*args: Group | Pixel | TextPath | Box) -> str: ...


class TextPath:
    style: Style
    line_style: str
    weight: int | None
    start_direction: str | None
    end_direction: str | None

    def __init__(
        self,
        start: tuple[int, int],
        end: tuple[int, int],
        style: str | None = None,
        *,
        line_style: Literal['regular', 'double', 'thick'] = 'regular',
        weight: int | None = None,
        start_direction: str | None = None,
        end_direction: str | None = None,
        bend_penalty: int = 1,
        environment: list[Group | Pixel | TextPath | Box] | tuple[Group | Pixel | TextPath | Box] | None = None,
        barriers: list[Group | Pixel | TextPath | Box] | tuple[Group | Pixel | TextPath | Box] | None = None,
        paths: list[Group | Pixel | TextPath | Box] | tuple[Group | Pixel | TextPath | Box] | None = None,
    ) -> Self: ...


def arrow(kind: str) -> str: ...


class Box:
    text: str
    position: tuple[int, int]
    width: int
    height: int
    style: Style
    border_style: Style
    line_style: Literal['regular', 'double', 'thick'] | None
    weight: int | None
    padding: tuple[int, int, int, int] | None
    padding_style: Style
    align: Literal['top', 'center', 'bottom']
    justify: Literal['right', 'center', 'left']
    truncate_string: str | None
    transparent: bool
    transparent_padding: bool

    def __init__(
        self,
        text: str = '',
        position: tuple[int, int] = (0, 0),
        width: int = 0,
        height: int = 0,
        style: str | None = None,
        border_style: str | None = None,
        line_style: Literal['regular', 'double', 'thick'] | None = 'regular',
        weight: int | None = 1,
        padding: tuple[int, int, int, int] | None = None,
        padding_style: str | None = None,
        align: Literal['top', 'center', 'bottom'] = 'top',
        justify: Literal['right', 'center', 'left'] = 'left',
        truncate_string: str | None = None,
        transparent: bool = False,
        transparent_padding: bool = False,
    ) -> Self: ...


__all__ = ['BoundingBox', 'Box', 'Group', 'Pixel', 'Style', 'TextPath', 'arrow', 'render']
