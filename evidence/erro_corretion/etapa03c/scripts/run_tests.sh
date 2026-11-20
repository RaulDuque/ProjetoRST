#!/usr/bin/env bash
# Etapa 03C (Erro-Corretion): reproduce the Agent Sprite Forge and PixelFX tests.
#
# Everything is offline and local: no image/video generation, no API call. Inputs are the
# approved N3.5/N3.6 sprites and the owner's original run video (read only). Raw outputs go
# to etapa03c/work/ (ignored by git); build_evidence.py turns them into etapa03c/out/.
#
#   bash evidence/erro_corretion/etapa03c/scripts/run_tests.sh
#
# Tools (installed in etapa 03C, see RELATORIO.md):
#   ASF      Claude Code plugin agent-sprite-forge@agent-sprite-forge 0.6.0 (commit 25203d0)
#   PIXELFX  Claude Code plugin pixelfx@pixelfx 0.3.1 (commit 5294518)
#   VENV     isolated Python 3.11 venv with numpy, Pillow, scipy and a static ffmpeg/ffprobe
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../.." && pwd)"
ASF="${ASF:-$HOME/.claude/plugins/cache/agent-sprite-forge/agent-sprite-forge/0.6.0/skills}"
PIXELFX="${PIXELFX:-$HOME/.claude/plugins/cache/pixelfx/pixelfx/0.3.1}"
VENV="${VENV:-$HOME/.local/share/dc-art-tools/asf-venv}"
export PATH="$VENV/bin:$PATH"
STAGE="$ROOT/evidence/erro_corretion/etapa03c"
WORK="$STAGE/work"
VIDEO="$ROOT/Md-usario/assets-user/prota/base/trimmed_fd4c76cf-0416-4387-b451-da4cc90a2061.mp4"
IDLE="$ROOT/art-src/n3-5-fidelity/generated/candidate_106.png"
RUN6="$ROOT/art-src/n3-6-animation-proof/generated/run6.png"
V2D="$ASF/video2dsprite/scripts"

if [ -e "$WORK" ]; then
  echo "work folder exists: $WORK (move it away first; the tools refuse existing outputs)" >&2
  exit 1
fi
mkdir -p "$WORK"
cd "$WORK"

echo "== doctors"
python "$V2D/video2dsprite.py" doctor > doctor_video.json
PYTHONUTF8=1 python "$ASF/generate2dmedia/scripts/forge_doctor.py" --host-tools none --save doctor_routes.json > doctor_routes.txt || true
(cd "$PIXELFX" && node tools/check.mjs melee arcane shadow) > pixelfx_check.txt

echo "== A1 video2dsprite: the original run video -> keyed frames -> loop -> pixel / hibit finish"
python "$V2D/video2dsprite.py" triage --video "$VIDEO" --output-dir triage
python "$V2D/video2dsprite.py" process --video "$VIDEO" --output-dir proc --name run --fps 0 \
  --start 1.5 --duration 6.3 --key green --workers 3
python "$V2D/gait_loop.py" select --frames-dir proc/frames-clean --fps 24 --output-dir loop
python - <<'EOF'
import json, os, shutil
sel = json.load(open("loop/selection.json"))
names = sorted(os.listdir("proc/frames-clean"))
os.makedirs("loopframes")
for name in names[sel["start"]:sel["endExclusive"]]:
    shutil.copy(f"proc/frames-clean/{name}", f"loopframes/{name}")
EOF
# 105.2 px rest-frame height = scale 0.2367, the N3.6 video scale (0.23661): same size as run6.
for mode in pixel hibit; do
  python "$V2D/finish_frames.py" "$mode" --frames loopframes --output-dir "fin_$mode" --target-height 105.2 \
    --loop-policy cycle --clip "run_asf_$mode" --character protagonist
done

echo "== A2 pixel_motion: breath idle from the approved idle sprite"
python "$V2D/pixel_motion.py" breath --sprite "$IDLE" --name idle_breath --output-dir breath

echo "== A3 pixel_motion: round trip of the approved run6 through a video (no generation)"
python - "$RUN6" <<'EOF'
import sys
from PIL import Image
run = Image.open(sys.argv[1]).convert("RGBA")
for i in range(run.width // 112):
    run.crop((i * 112, 0, i * 112 + 112, 128)).save(f"run_{i}.png")
EOF
python "$V2D/pixel_motion.py" prepare --sprite run_0.png --action run --facing right \
  --subject "silver-haired swordsman in grey shirt" --output-dir job
python - <<'EOF'
import json, os
from PIL import Image
job = json.load(open("job/registration_job.json"))
scale, (ox, oy) = job["referenceScale"], job["referenceOffset"]
base = Image.open("job/input.png").convert("RGB")
key = base.getpixel((2, 2))
os.makedirs("synth")
n = 0
for _ in range(5):          # 5 cycles, every drawing held 2 video frames (12 fps drawing at 24 fps)
    for i in range(6):
        sprite = Image.open(f"run_{i}.png").convert("RGBA")
        sprite = sprite.resize((sprite.width * scale, sprite.height * scale), Image.Resampling.NEAREST)
        frame = Image.new("RGB", base.size, key)
        frame.paste(sprite, (ox, oy), sprite)
        for _ in range(2):
            frame.save(f"synth/f_{n:04d}.png")
            n += 1
EOF
ffmpeg -v error -y -framerate 24 -i synth/f_%04d.png -c:v libx264 -pix_fmt yuv420p -crf 20 synth_run.mp4
python "$V2D/pixel_motion.py" frames --clip synth_run.mp4 --job job --output-dir pm_frames
python "$V2D/pixel_motion.py" finish --frames-dir pm_frames --name run_roundtrip --output-dir pm_fin \
  --mode cycle --count 6 --ms 83

echo "== A4 pixel_look master: the approved idle read back from a blurred, enlarged JPEG"
python - "$IDLE" <<'EOF'
import sys
from PIL import Image, ImageFilter
sprite = Image.open(sys.argv[1]).convert("RGBA")
sprite = sprite.crop(sprite.getbbox())
big = sprite.resize((sprite.width * 11, sprite.height * 11), Image.Resampling.NEAREST)
picture = Image.new("RGB", (big.width + 200, big.height + 200), (40, 170, 60))
picture.paste(big, (100, 100), big)
picture = picture.resize((int(picture.width * 1.07), int(picture.height * 1.07)), Image.Resampling.BICUBIC)
picture.filter(ImageFilter.GaussianBlur(1.6)).save("picture.jpg", quality=82)
EOF
python "$ASF/generate2dsprite/scripts/pixel_look.py" master --image picture.jpg --output-dir look

echo "== B pixelfx: gothic sword slash and spell, sprite sheets + JSON"
BLADE='["#120a1c","#2a1a44","#4a2e74","#7c62aa","#b6a8d6","#e8e2f6","#ffffff"]'
SPELL='["#10061a","#2e1048","#5a1c80","#8e34b4","#c070d8","#e8b860","#fff0c0"]'
(cd "$PIXELFX" && node tools/render.mjs melee/sword-slash --sheet --gif --scale 4 --bg dark --out "$WORK/pixelfx" \
  --opts "{\"palette\":$BLADE,\"radius\":64}")
(cd "$PIXELFX" && node tools/render.mjs arcane/magic-circle-cast --sheet --gif --scale 3 --bg dark --out "$WORK/pixelfx" \
  --opts "{\"palette\":$SPELL}")
echo "done: $WORK"
