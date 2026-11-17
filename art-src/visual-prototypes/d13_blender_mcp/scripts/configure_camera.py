"""Create/update D13 orthographic side camera for sprite export."""
from __future__ import annotations

import math

import bpy
from mathutils import Euler

CAM = "D13_cam_ortho_side"


def main() -> None:
    if CAM in bpy.data.objects:
        cam_obj = bpy.data.objects[CAM]
    else:
        data = bpy.data.cameras.new(CAM)
        cam_obj = bpy.data.objects.new(CAM, data)
        bpy.context.scene.collection.objects.link(cam_obj)
    cam_obj.data.type = "ORTHO"
    cam_obj.data.ortho_scale = 2.05
    cam_obj.location = (0.0, -4.2, 0.95)
    cam_obj.rotation_euler = Euler((math.radians(90), 0, 0), "XYZ")
    bpy.context.scene.camera = cam_obj
    scene = bpy.context.scene
    scene.render.resolution_x = 512
    scene.render.resolution_y = 640
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = "RGBA"
    scene.render.film_transparent = True
    print("OK camera", CAM, cam_obj.data.ortho_scale)


if __name__ == "__main__":
    main()
