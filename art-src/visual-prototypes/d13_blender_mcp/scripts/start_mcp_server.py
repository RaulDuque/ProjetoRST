"""Enable MCP for Blender addon and start the socket server (lab only)."""
from __future__ import annotations

import bpy


def main() -> None:
    # Prefer the installed module name.
    module = "blender_mcp"
    try:
        bpy.ops.preferences.addon_enable(module=module)
        print(f"enabled addon {module}")
    except Exception as e:
        print(f"addon_enable failed: {e}")

    scene = bpy.context.scene
    if hasattr(scene, "blendermcp_port"):
        scene.blendermcp_port = 9876
    if hasattr(scene, "blendermcp_auto_start_server"):
        scene.blendermcp_auto_start_server = True

    # Start server via operator if available.
    try:
        bpy.ops.blendermcp.start_server()
        print("started via blendermcp.start_server")
    except Exception as e:
        print(f"start_server op failed: {e}")
        # Fallback: call ensure helper if present
        try:
            import blender_mcp as m

            if hasattr(m, "_blendermcp_ensure_server_running"):
                m._blendermcp_ensure_server_running()
                print("started via _blendermcp_ensure_server_running")
        except Exception as e2:
            print(f"fallback failed: {e2}")

    print(
        "mcp_status",
        getattr(scene, "blendermcp_server_running", None),
        getattr(scene, "blendermcp_port", None),
    )


if __name__ == "__main__":
    main()
