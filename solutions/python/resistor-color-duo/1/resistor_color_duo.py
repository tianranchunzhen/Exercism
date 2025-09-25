def value(colors):
    color_num = {
        "black": 0,
        "brown": 1,
        "red": 2,
        "orange": 3,
        "yellow": 4,
        "green": 5,
        "blue": 6,
        "violet": 7,
        "grey": 8,
        "white": 9,
    }
    a, b = [color_num[color] for color in colors][:2]
    return a * 10 + b