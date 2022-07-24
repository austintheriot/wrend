use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Matrix4x4(pub [f64; 16]);

impl Matrix4x4 {
    pub fn identity_matrix() -> Matrix4x4 {
        Matrix4x4([
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ])
    }

    pub fn translation_matrix(tx: f64, ty: f64, tz: f64) -> Matrix4x4 {
        Matrix4x4([
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., tx, ty, tz, 1.,
        ])
    }

    pub fn scaling_matrix(sx: f64, sy: f64, sz: f64) -> Matrix4x4 {
        Matrix4x4([
            sx, 0., 0., 0., 0., sy, 0., 0., 0., 0., sz, 0., 0., 0., 0., 1.,
        ])
    }

    pub fn x_rotation_matrix(angle_in_radians: f64) -> Matrix4x4 {
        let c = angle_in_radians.cos();
        let s = angle_in_radians.sin();

        Matrix4x4([1., 0., 0., 0., 0., c, s, 0., 0., -s, c, 0., 0., 0., 0., 1.])
    }

    pub fn y_rotation_matrix(angle_in_radians: f64) -> Matrix4x4 {
        let c = angle_in_radians.cos();
        let s = angle_in_radians.sin();

        Matrix4x4([c, 0., -s, 0., 0., 1., 0., 0., s, 0., c, 0., 0., 0., 0., 1.])
    }

    pub fn z_rotation_matrix(angle_in_radians: f64) -> Matrix4x4 {
        let c = angle_in_radians.cos();
        let s = angle_in_radians.sin();

        Matrix4x4([c, s, 0., 0., -s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.])
    }

    pub fn multiply(self: Matrix4x4, b: Matrix4x4) -> Matrix4x4 {
        let matrix_size: usize = 4;

        // matrix coordinates
        let b00 = b.0[0 * matrix_size + 0];
        let b01 = b.0[0 * matrix_size + 1];
        let b02 = b.0[0 * matrix_size + 2];
        let b03 = b.0[0 * matrix_size + 3];
        let b10 = b.0[1 * matrix_size + 0];
        let b11 = b.0[1 * matrix_size + 1];
        let b12 = b.0[1 * matrix_size + 2];
        let b13 = b.0[1 * matrix_size + 3];
        let b20 = b.0[2 * matrix_size + 0];
        let b21 = b.0[2 * matrix_size + 1];
        let b22 = b.0[2 * matrix_size + 2];
        let b23 = b.0[2 * matrix_size + 3];
        let b30 = b.0[3 * matrix_size + 0];
        let b31 = b.0[3 * matrix_size + 1];
        let b32 = b.0[3 * matrix_size + 2];
        let b33 = b.0[3 * matrix_size + 3];
        let a00 = self.0[0 * matrix_size + 0];
        let a01 = self.0[0 * matrix_size + 1];
        let a02 = self.0[0 * matrix_size + 2];
        let a03 = self.0[0 * matrix_size + 3];
        let a10 = self.0[1 * matrix_size + 0];
        let a11 = self.0[1 * matrix_size + 1];
        let a12 = self.0[1 * matrix_size + 2];
        let a13 = self.0[1 * matrix_size + 3];
        let a20 = self.0[2 * matrix_size + 0];
        let a21 = self.0[2 * matrix_size + 1];
        let a22 = self.0[2 * matrix_size + 2];
        let a23 = self.0[2 * matrix_size + 3];
        let a30 = self.0[3 * matrix_size + 0];
        let a31 = self.0[3 * matrix_size + 1];
        let a32 = self.0[3 * matrix_size + 2];
        let a33 = self.0[3 * matrix_size + 3];

        Matrix4x4([
            b00 * a00 + b01 * a10 + b02 * a20 + b03 * a30,
            b00 * a01 + b01 * a11 + b02 * a21 + b03 * a31,
            b00 * a02 + b01 * a12 + b02 * a22 + b03 * a32,
            b00 * a03 + b01 * a13 + b02 * a23 + b03 * a33,
            b10 * a00 + b11 * a10 + b12 * a20 + b13 * a30,
            b10 * a01 + b11 * a11 + b12 * a21 + b13 * a31,
            b10 * a02 + b11 * a12 + b12 * a22 + b13 * a32,
            b10 * a03 + b11 * a13 + b12 * a23 + b13 * a33,
            b20 * a00 + b21 * a10 + b22 * a20 + b23 * a30,
            b20 * a01 + b21 * a11 + b22 * a21 + b23 * a31,
            b20 * a02 + b21 * a12 + b22 * a22 + b23 * a32,
            b20 * a03 + b21 * a13 + b22 * a23 + b23 * a33,
            b30 * a00 + b31 * a10 + b32 * a20 + b33 * a30,
            b30 * a01 + b31 * a11 + b32 * a21 + b33 * a31,
            b30 * a02 + b31 * a12 + b32 * a22 + b33 * a32,
            b30 * a03 + b31 * a13 + b32 * a23 + b33 * a33,
        ])
    }

    pub fn translate(self, tx: f64, ty: f64, tz: f64) -> Matrix4x4 {
        self.multiply(Matrix4x4::translation_matrix(tx, ty, tz))
    }

    pub fn scale(self: Matrix4x4, sx: f64, sy: f64, sz: f64) -> Matrix4x4 {
        self.multiply(Matrix4x4::scaling_matrix(sx, sy, sz))
    }

    pub fn rotate_x(self: Matrix4x4, angle_in_radians: f64) -> Matrix4x4 {
        self.multiply(Matrix4x4::x_rotation_matrix(angle_in_radians))
    }

    pub fn rotate_y(self: Matrix4x4, angle_in_radians: f64) -> Matrix4x4 {
        self.multiply(Matrix4x4::y_rotation_matrix(angle_in_radians))
    }

    pub fn rotate_z(self: Matrix4x4, angle_in_radians: f64) -> Matrix4x4 {
        self.multiply(Matrix4x4::z_rotation_matrix(angle_in_radians))
    }
}

impl From<Vec3> for Matrix4x4 {
    fn from(vec3: Vec3) -> Self {
        Matrix4x4([
            1.,
            0.,
            0.,
            vec3.x(),
            0.,
            1.,
            0.,
            vec3.y(),
            0.,
            0.,
            1.,
            vec3.z(),
            0.,
            0.,
            0.,
            1.,
        ])
    }
}

impl From<&Vec3> for Matrix4x4 {
    fn from(vec3: &Vec3) -> Self {
        (*vec3).into()
    }
}
