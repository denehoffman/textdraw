from textdraw import Box, Pixel, TextPath, render

if __name__ == '__main__':
    a = Box('A', (-20, 10), border_style='green')
    b = Box('B', (0, 0), border_style='red')
    print(a.bounding_box)
    start_node = Pixel('', (a.bounding_box.right + 1, a.bounding_box.bottom), style='red')
    end_node = Pixel('◼', (b.bounding_box.left - 1, b.bounding_box.top), style='green')
    path = TextPath(
        (a.bounding_box.right + 1, a.bounding_box.bottom - 1),
        (b.bounding_box.left - 2, b.bounding_box.top),
        style='dimmed',
        start_direction='down',
        end_direction='right',
        bend_penalty=20,
    )
    print(render(a, b, start_node, end_node, path))
