from typing import Self

class BoundingBox:
    top: int
    right: int
    bottom: int
    left: int
    def __init__(self, top: int, right: int, bottom: int, left: int): ...
    def __add__(self, other: tuple[int, int] | Self) -> Self: ...
    def __contains__(self, other: tuple[int, int] | Self) -> bool: ...

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

def render(*args: Group | Pixel) -> str: ...

__all__ = ["BoundingBox", "Group", "Pixel", "Style", "render"]
