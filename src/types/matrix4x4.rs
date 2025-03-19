#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn new() -> Self {
        Matrix4x4 {
            data: [[0.0; 4]; 4],
        }
    }

    pub fn identity() -> Self {
        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn multiply(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = (0..4).map(|k| self.data[i][k] * other.data[k][j]).sum();
            }
        }
        result
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }
}
