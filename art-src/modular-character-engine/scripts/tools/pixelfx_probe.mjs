// pixelfx probe (offline tool, not part of the game).
//
// Runs pixelfx's own rig (src/core/rig.js, MIT, Copyright (c) 2026 Patch,
// Hugo Duprez) to show the weapon model the Dark Castle grip profiles follow:
// with the hand target fixed, changing the weapon angle `sa` turns the blade
// but not the forearm; moving the hand moves the forearm while the blade
// keeps `sa`. Prints JSON (forearm angle, blade angle, hand position).
//
//   node art-src/modular-character-engine/scripts/tools/pixelfx_probe.mjs <pixelfx dir>
import { pathToFileURL } from "node:url";
import { join } from "node:path";

const dir = process.argv[2];
const { skeleton } = await import(pathToFileURL(join(dir, "src/core/rig.js")).href);
const { READY } = await import(pathToFileURL(join(dir, "src/core/poses.js")).href);

const deg = (r) => Math.round((r * 180) / Math.PI * 10) / 10;
const angle = (a, b) => Math.atan2(-(b[1] - a[1]), b[0] - a[0]); // y-down -> CCW

function probe(label, pose) {
  const s = skeleton(pose);
  const J = s.joints;
  return {
    label,
    sa: pose.sa,
    hand: [Math.round(J.handN[0] * 10) / 10, Math.round(J.handN[1] * 10) / 10],
    forearm_deg: deg(angle(J.elbowN, J.handN)),
    // pixelfx: blade axis = [cos sa, -sin sa] in screen terms (0 forward, negative = up)
    blade_deg: deg(-pose.sa),
  };
}

const base = { ...READY, weapon: "sword", fx: 12, fy: -21 };
const rows = [];
for (const sa of [-2.65, -1.45, -0.85, 0.62]) rows.push(probe(`sa ${sa}, hand fixed`, { ...base, sa }));
for (const [fx, fy] of [[6, -16], [12, -21], [16, -30]]) rows.push(probe(`hand ${fx},${fy}, sa fixed`, { ...base, fx, fy, sa: -0.85 }));
const fixedHand = rows.slice(0, 4);
const fixedBlade = rows.slice(4);
const spread = (xs) => Math.round((Math.max(...xs) - Math.min(...xs)) * 10) / 10;
console.log(JSON.stringify({
  tool: "pixelfx 0.3.1 src/core/rig.js skeleton()",
  rows,
  forearm_spread_when_only_sa_changes_deg: spread(fixedHand.map((r) => r.forearm_deg)),
  blade_spread_when_only_the_hand_moves_deg: spread(fixedBlade.map((r) => r.blade_deg)),
}, null, 1));
