from textdraw import Group, Pixel, TextPath, render

lr_barriers = Group([Pixel("X", (-1, 0)), Pixel("X", (1, 0))], style="blink red")

barriers_a = lr_barriers.at((4, 5))
barriers_b = lr_barriers.at((7, 3))

others = Group([Pixel("O", (3, 2), weight=4), Pixel("O", (4, 2), weight=3)], style="blue")

others2 = Group([Pixel("O", (7, 2), style="green"), Pixel("O", (8, 10))], style="on white", weight=5)
others2[1].style += "bold"


path_a = TextPath(
    (0, 0),
    (5, 5),
    style="blue",
    end_style="up arrow blink red on black",
    environment=[others, others2],
    barriers=[barriers_a],
)
path_b = TextPath(
    (0, 0),
    (6, 5),
    style="magenta",
    end_style="up arrow blink red on black",
    environment=[others, others2],
    barriers=[barriers_b],
    paths=[path_a],
)

print(render(barriers_a, barriers_b, others, others2, path_a, path_b))
