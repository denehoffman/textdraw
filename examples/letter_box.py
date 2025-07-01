from textdraw import Box, TextPath, render, Pixel, Group


class LetterBox:
    def __init__(self, letter: str, x: int, y: int):
        self.box = Box(letter, (x, y))
        self.c_left = (self.box.bounding_box.left - 1, self.box.bounding_box.bottom + self.box.bounding_box.height // 2)
        self.c_right = (
            self.box.bounding_box.right + 1,
            self.box.bounding_box.bottom + self.box.bounding_box.height // 2,
        )
        self.c_top = (self.box.bounding_box.left + self.box.bounding_box.width // 2, self.box.bounding_box.top + 1)
        self.c_bottom = (
            self.box.bounding_box.left + self.box.bounding_box.width // 2,
            self.box.bounding_box.bottom - 1,
        )
        marker = Pixel('⎚', style='green', weight=1)
        self.margin_markers = Group(
            [
                marker.at((self.c_left[0] - 2, self.c_left[1])),
                marker.at((self.c_right[0] + 2, self.c_right[1])),
                marker.at((self.c_top[0], self.c_top[1] + 2)),
                marker.at((self.c_bottom[0], self.c_bottom[1] - 2)),
            ]
        )
        barrier = Pixel('⎚', style='red', weight=None)
        self.barriers = Group(
            [
                barrier.at((self.c_left[0], self.c_left[1] - 1)),
                barrier.at((self.c_left[0], self.c_left[1] + 1)),
                barrier.at((self.c_right[0], self.c_right[1] - 1)),
                barrier.at((self.c_right[0], self.c_right[1] + 1)),
                barrier.at((self.c_top[0] - 1, self.c_top[1])),
                barrier.at((self.c_top[0] + 1, self.c_top[1])),
                barrier.at((self.c_bottom[0] - 1, self.c_bottom[1])),
                barrier.at((self.c_bottom[0] + 1, self.c_bottom[1])),
            ]
        )


if __name__ == '__main__':
    a = LetterBox('a', 0, 0)
    b = LetterBox('b', 20, -8)
    c = LetterBox('c', 3, -10)

    all_barriers = [a.barriers, b.barriers, c.barriers, a.box, b.box, c.box]
    all_markers = [a.margin_markers, b.margin_markers, c.margin_markers]
    paths = []
    paths.append(
        TextPath(
            a.c_right,
            b.c_top,
            style='dimmed',
            weight=20,
            bend_penalty=20,
            environment=[*all_markers, *paths],
            barriers=all_barriers,
        )
    )
    paths.append(
        TextPath(
            a.c_bottom,
            b.c_left,
            style='green',
            weight=20,
            bend_penalty=20,
            environment=[*all_markers, *paths],
            barriers=all_barriers,
        )
    )

    paths.append(
        TextPath(
            a.c_left,
            c.c_top,
            style='blue',
            weight=20,
            bend_penalty=20,
            environment=[*all_markers, *paths],
            barriers=all_barriers,
        )
    )

    paths.append(
        TextPath(
            b.c_bottom,
            c.c_left,
            style='red',
            line_style='double',
            weight=20,
            bend_penalty=20,
            environment=[*all_markers, *paths],
            barriers=all_barriers,
        )
    )
    shared_paths = [
        TextPath(
            c.c_bottom,
            a.c_right,
            style='yellow',
            line_style='thick',
            bend_penalty=20,
            environment=[*all_markers, *paths],
            barriers=all_barriers,
        )
    ]
    shared_paths.append(
        TextPath(
            b.c_left,
            c.c_right,
            style='yellow',
            line_style='thick',
            bend_penalty=20,
            paths=shared_paths,
            environment=[*all_markers, *paths],
            barriers=all_barriers,
        )
    )
    shared_paths.append(
        TextPath(
            a.c_top,
            b.c_right,
            style='yellow',
            line_style='thick',
            bend_penalty=20,
            paths=shared_paths,
            environment=[*all_markers, *paths],
            barriers=all_barriers,
        )
    )
    print(render(a.box, b.box, c.box, *paths, *shared_paths))

    blinking_shared_paths = []
    for path in shared_paths:
        path.style += 'blink'
        blinking_shared_paths.append(path)
    print(render(a.box, b.box, c.box, *paths, *blinking_shared_paths))
