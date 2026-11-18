"""N3.7: evidence sheet of the reusable pipeline.

Compares the pipeline's rebuild of the N3.6 run with the approved grids
(texel by texel) and stacks the preview of both configs into
`docs/development/n3_7/evidence/pipeline_output.png`.

    python3 -B art-src/n3-7-run-sync/scripts/pipeline_evidence.py
"""

from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[3]
HERE = ROOT / "art-src/n3-7-run-sync"
APPROVED = ROOT / "art-src/n3-6-animation-proof/source"
EVIDENCE = ROOT / "docs/development/n3_7/evidence"
INK = (40, 32, 40)
LIGHT = (226, 224, 228, 255)


def main() -> None:
    rebuilt = sorted((HERE / "pipeline/run_n36/source").glob("video_*.txt"))
    same = sum((APPROVED / path.name).read_bytes() == path.read_bytes() for path in rebuilt)
    rows = [
        (f"run_n36.json: reconstrucao da corrida N3.6 pelo pipeline - {same}/{len(rebuilt)} grades "
         "identicas texel a texel as aprovadas", Image.open(HERE / "pipeline/run_n36/preview/frames.png")),
        ("run_start.json: partida real do video (quadros 0-13, mesma escala e deslocamento); "
         "run_start = 3, 5, 7, 9, 11, 13", Image.open(HERE / "pipeline/run_start/preview/frames.png")),
    ]
    width = max(image.width for _, image in rows)
    height = sum(image.height + 24 for _, image in rows) + 30
    out = Image.new("RGBA", (width, height), LIGHT)
    draw = ImageDraw.Draw(out)
    y = 0
    for title, image in rows:
        draw.text((8, y + 6), title, fill=INK)
        out.alpha_composite(image.convert("RGBA"), (0, y + 24))
        y += image.height + 24
    draw.text((8, y + 8), "caixa azul = caixa opaca do quadro; traco vermelho = pivo (56, 118); "
              "nenhum quadro editado a mao ou gerado pelo agente", fill=INK)
    out.save(EVIDENCE / "pipeline_output.png")
    print(f"{same}/{len(rebuilt)} identical")


if __name__ == "__main__":
    main()
