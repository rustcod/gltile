pub static VERTEX: &'static str = r##"
  #version 140

  in vec2 position;
  in vec2 screen_loc;
  in vec2 tileset_coords;

  in vec3 foreground_color;
  in vec3 background_color;

  out vec2 v_tileset_coords;

  out vec3 v_foreground_color;
  out vec3 v_background_color;

  uniform mat4 mvp;

  void main() {
      gl_Position = mvp * vec4(position[0] + {width} * screen_loc[0], position[1] + {height} * screen_loc[1], 0.0, 1.0);
      v_tileset_coords = tileset_coords;
      v_foreground_color = foreground_color;
      v_background_color = background_color;
  }
"##;
