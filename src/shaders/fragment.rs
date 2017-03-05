pub static FRAGMENT: &'static str = r##"
  #version 140

  in vec2 v_tileset_coords;
  in vec3 v_foreground_color;
  in vec3 v_background_color;

  out vec4 color;

  uniform sampler2D tileset;

  void main() {
    vec4 tileset_color = texture(tileset, v_tileset_coords);
    color = mix(vec4(v_background_color, 1.0), vec4(v_foreground_color, 1.0) * tileset_color, tileset_color.a);
  }
"##;
