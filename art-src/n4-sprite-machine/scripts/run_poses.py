"""N4-1 (C1): skeletons of the three representative run frames.

Joints are canvas texels (112x128, pivot 56,118) measured on the enlarged
approved frames (N3.6 run6 #0, #2, #4 = video 48, 52, 56) and on the
high-resolution video frames. They are an interpretation: they guide which
limb a texel belongs to and where the hidden anatomy is drawn.

"near" = the limb on the camera side, "far" = the other one. The same
physical limb is near in every frame. The arms are read from the video
(near arm forward in #0 and #2, back in #4). The legs follow the running
gait (contralateral to the arms): near leg back in #0 and #2, forward in
#4. The occlusion at the hips is ambiguous in the reduced frames.

Radii are half-thicknesses in texels; ``boot_t`` is where the boot shaft
starts along the shin (0 = knee, 1 = ankle).
"""

from __future__ import annotations

FRAMES = {
    0: {  # video 48: contact
        "source": "video_048",
        "head": (66, 26), "head_r": 7.0, "face_x": 63,
        "neck": (63, 34), "shoulder_near": (61, 39), "shoulder_far": (57, 39),
        "hip": (58, 62),
        "torso": [(54, 36), (68, 36), (71, 44), (69, 58), (54, 61), (51, 48)],
        "near_arm": [(61, 39), (57, 55), (71, 56), (75, 55)],   # shoulder, elbow, wrist, fist
        "far_arm": [(57, 39), (55, 52), (64, 60), (67, 62)],
        "near_leg": [(56, 64), (50, 87), (33, 85), (23, 91)],  # hip, knee, ankle, toe
        "far_leg": [(60, 64), (60, 89), (59, 111), (72, 117)],
        "near_heel": (31, 79), "far_heel": (55, 117),
        "boot_t_near": 0.25, "boot_t_far": 0.35,
        "belt_y": 59,
    },
    2: {  # video 52: open stride
        "source": "video_052",
        "head": (66, 25), "head_r": 7.0, "face_x": 63,
        "neck": (63, 33), "shoulder_near": (61, 38), "shoulder_far": (56, 38),
        "hip": (58, 62),
        "torso": [(52, 35), (68, 35), (71, 44), (70, 58), (53, 61), (49, 46)],
        "near_arm": [(61, 38), (63, 51), (77, 42), (81, 39)],
        "far_arm": [(56, 38), (45, 45), (47, 55), (48, 58)],
        "near_leg": [(56, 64), (45, 82), (25, 88), (18, 93)],
        "far_leg": [(60, 64), (73, 80), (79, 106), (92, 111)],
        "near_heel": (26, 84), "far_heel": (76, 113),
        "boot_t_near": 0.45, "boot_t_far": 0.55,
        "belt_y": 59,
    },
    4: {  # video 56: recovery
        "source": "video_056",
        "head": (67, 24), "head_r": 7.0, "face_x": 64,
        "neck": (64, 33), "shoulder_near": (57, 38), "shoulder_far": (63, 38),
        "hip": (58, 62),
        "torso": [(51, 34), (69, 34), (72, 44), (70, 58), (53, 61), (48, 46)],
        "near_arm": [(57, 38), (44, 46), (54, 55), (57, 58)],
        "far_arm": [(63, 38), (70, 50), (78, 44), (82, 40)],
        "near_leg": [(60, 64), (77, 79), (57, 100), (61, 109)],
        "far_leg": [(56, 64), (48, 87), (35, 109), (42, 117)],
        "near_heel": (53, 104), "far_heel": (30, 114),
        "boot_t_near": 0.3, "boot_t_far": 0.5,
        "belt_y": 59,
    },
}

RADII = {"upper_arm": 2.6, "forearm": 2.2, "fist": 2.4, "thigh": 4.2, "shin": 3.0, "foot": 2.2}
