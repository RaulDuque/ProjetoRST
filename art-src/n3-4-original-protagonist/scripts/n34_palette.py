"""N3.4 base-protagonist palette: one character per colour in the text grids.

The colours were picked by eye from the right-side view of
`Ficha_Modular_do_Protagonista_Gotico.png` (hair, skin, grey shirt, dark
brown trousers, leather) and then adjusted so neighbouring materials stay
distinct at 106 px. `.` is transparent.
"""

from __future__ import annotations

PALETTE: dict[str, tuple[int, int, int]] = {
    # outline / deepest shadow
    "K": (26, 20, 26),
    # hair: silver-white with cool shadows
    "W": (238, 238, 244),
    "w": (204, 206, 218),
    "h": (156, 160, 180),
    "H": (104, 108, 130),
    # skin
    "S": (236, 200, 172),
    "s": (204, 154, 126),
    "z": (146, 98, 86),
    # eye / brow
    "e": (58, 52, 64),
    # grey shirt
    "G": (138, 134, 140),
    "g": (104, 100, 108),
    "q": (72, 68, 78),
    # dark brown trousers
    "P": (88, 70, 70),
    "p": (62, 48, 50),
    "o": (44, 34, 38),
    # leather (belt, bracer, boots)
    "L": (104, 84, 74),
    "l": (70, 54, 50),
    "d": (46, 36, 36),
    # metal (buckle, boot buckles)
    "M": (176, 172, 168),
}

TRANSPARENT = "."
