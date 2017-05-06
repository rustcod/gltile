// taken from vecmath & cam (MIT)
// https://github.com/PistonDevelopers/vecmath
// https://github.com/PistonDevelopers/cam

pub type Vector4 = [f32; 4];
pub type Matrix4 = [[f32; 4]; 4];

pub fn model_view_projection(model: Matrix4, view: Matrix4, projection: Matrix4) -> Matrix4 {
    col_mat4_mul(col_mat4_mul(projection, view), model)
}

fn col_mat4_mul(a: Matrix4, b: Matrix4) -> Matrix4 {
    [
        col_mat4_mul_col(a, b, 0),
        col_mat4_mul_col(a, b, 1),
        col_mat4_mul_col(a, b, 2),
        col_mat4_mul_col(a, b, 3),
    ]
}

fn col_mat4_mul_col(a: Matrix4, b: Matrix4, i: usize) -> Vector4 {
    [
        vec4_dot(col_mat4_row(a, 0), b[i]),
        vec4_dot(col_mat4_row(a, 1), b[i]),
        vec4_dot(col_mat4_row(a, 2), b[i]),
        vec4_dot(col_mat4_row(a, 3), b[i]),
    ]
}

fn vec4_dot(a: Vector4, b: Vector4) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
}

fn col_mat4_row(a: Matrix4, i: usize) -> Vector4 {
    row_mat4_col(a, i)
}

fn row_mat4_col(a: Matrix4, i: usize) -> Vector4 {
    [a[0][i], a[1][i], a[2][i], a[3][i]]
}
