use std::ops::{Index, IndexMut, Mul};
use crate::vector4::Vector4;

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Matrix4x4 {
    data: [[f32; 4]; 4]
}

impl Matrix4x4 {
    pub fn new(data: [[f32; 4]; 4]) -> Self {
        Self { data }
    }

    pub fn zero() -> Self {
        Self::new([[0.0; 4]; 4])
    }

    pub fn identity() -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn set(&mut self, row: usize, column: usize, value: f32) {
        self.data[row][column] = value
    }

    pub fn multiply(self, m: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * m.data[k][j]
                }
            }
        }
        result
    }

    pub fn multiply_by_vector(self, v: Vector4) -> Vector4 {
        let rhs = [v.x, v.y, v.z, v.w];
        let mut result = [0.0f32; 4];
        for r in 0..4 {
            for c in 0..4 {
                result[r] += self.data[r][c]*rhs[c]
            }
        }
        Vector4::new(result[0], result[1], result[2], result[3])
    }
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = [f32; 4];

    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Mul<Vector4> for Matrix4x4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        self.multiply_by_vector(rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::matrix4x4::Matrix4x4;

    #[test]
    fn multiply_matrices() {
        let m0 = Matrix4x4::identity();
        let m1 = Matrix4x4::zero();
        assert_eq!(m0 * m1, Matrix4x4::zero());
    }
}
