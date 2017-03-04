#version 140

in vec2 position;
in vec2 screen_point;
in vec2 tileset_coords;

in vec3 foreground_color;
in vec3 background_color;

out vec2 v_tileset_coords;

out vec3 v_foreground_color;
out vec3 v_background_color;

uniform mat4 mvp;

void main() {
    // 16.0 fixed "scale"
    gl_Position = mvp * vec4(position + 16.0 * screen_point, 0.0, 1.0);
    v_tileset_coords = tileset_coords;
    v_foreground_color = foreground_color;
    v_background_color = background_color;
}
