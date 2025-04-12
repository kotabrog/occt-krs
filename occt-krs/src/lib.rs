use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Sub, Mul};

/// 3次元ベクトルを表す構造体
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// 新しいベクトルを生成する
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// 内積を計算する
    pub fn dot(self, other: Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// 外積を計算する
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// ベクトルの長さ（ノルム）を計算する
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// 正規化（単位ベクトル化）する  
    /// ※長さがゼロの場合はpanicするので注意
    pub fn normalized(self) -> Vector3 {
        let len = self.length();
        if len == 0.0 {
            panic!("ゼロ長ベクトルは正規化できません");
        }
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

/// Vector3同士の加算の実装
impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// Vector3同士の減算の実装
impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// Vector3のスカラー倍の実装 (ベクトル * スカラー)
impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

/// スカラー倍の右側にベクトルを許容するための実装 (スカラー * ベクトル)
impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

/// Vector3をJSON形式でファイルに出力する関数  
/// テストなどで、OpenCascade側の出力との比較に利用できます。
pub fn output_vector_as_json(vector: &Vector3, filename: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(vector)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        // 直交するので内積は0
        assert!((a.dot(b) - 0.0).abs() < 1e-10);

        let c = Vector3::new(1.0, 2.0, 3.0);
        let d = Vector3::new(4.0, -5.0, 6.0);
        let dot = c.dot(d);
        // 計算結果：1*4 + 2*(-5) + 3*6 = 4 - 10 + 18 = 12
        assert!((dot - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_cross_product() {
        let a = Vector3::new(1.0, 0.0, 0.0);
        let b = Vector3::new(0.0, 1.0, 0.0);
        let cross = a.cross(b);
        // 右手系で外積は (0, 0, 1)
        let expected = Vector3::new(0.0, 0.0, 1.0);
        assert!((cross.x - expected.x).abs() < 1e-10);
        assert!((cross.y - expected.y).abs() < 1e-10);
        assert!((cross.z - expected.z).abs() < 1e-10);
    }

    #[test]
    fn test_length_and_normalization() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let len = v.length();
        // 3-4-5の三角形なので長さは5
        assert!((len - 5.0).abs() < 1e-10);

        let normalized = v.normalized();
        // 正規化後は長さが1
        assert!((normalized.length() - 1.0).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "ゼロ長ベクトルは正規化できません")]
    fn test_normalize_zero_vector() {
        let zero = Vector3::new(0.0, 0.0, 0.0);
        // ゼロベクトルの正規化はpanicするのでテストが成功するはず
        let _ = zero.normalized();
    }
}
