from textdraw import Box, Pixel, TextPath, render

if __name__ == '__main__':
    a = Box('A', (-20, 10), border_style='green')
    b = Box('B', (0, 0), border_style='red')
    print(a.bbox)
    start_node = Pixel('', (a.bbox.right + 1, a.bbox.bottom), style='red')
    end_node = Pixel('◼', (b.bbox.left - 1, b.bbox.top), style='green')
    path = TextPath(
        (a.bbox.right + 1, a.bbox.bottom - 1),
        (b.bbox.left - 2, b.bbox.top),
        style='dimmed',
        start_direction='down',
        end_direction='right',
        bend_penalty=20,
    )
    print(render(a, b, start_node, end_node, path))
