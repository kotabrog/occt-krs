// tests/integration_test.rs

use std::fs::File;
use std::io::Read;

use occt_krs::Vector3; // ここは実際のクレート名に合わせて変更してください
use serde::Deserialize;

/// Python 側で出力された JSON の各フィールドに対応する構造体
#[derive(Debug, Deserialize)]
struct PythonResults {
    #[serde(rename = "v1")]
    v1: Vector3,
    #[serde(rename = "v2")]
    v2: Vector3,
    #[serde(rename = "v1 + v2")]
    v1_add: Vector3,
    #[serde(rename = "v1 - v2")]
    v1_sub: Vector3,
    #[serde(rename = "v1 dot v2")]
    v1_dot: f64,
    #[serde(rename = "v1 cross v2")]
    v1_cross: Vector3,
    #[serde(rename = "v1 magnitude")]
    v1_magnitude: f64,
    #[serde(rename = "v1 normalized")]
    v1_normalized: Vector3,
}

/// 浮動小数点数の比較（許容誤差 tol 以内なら同一とみなす）
fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() < tol
}

#[test]
fn test_vector_operations_against_python_results() {
    // プロジェクトのルートディレクトリから result ディレクトリのパスを生成する
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let result_dir = std::path::Path::new(&manifest_dir).join("../python_test/result");
    let json_path = result_dir.join("python_vector_results.json");

    let mut file = File::open(&json_path)
        .expect("Could not open result/python_vector_results.json. Ensure the file exists in the result directory.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read result/python_vector_results.json");

    let python_results: PythonResults =
        serde_json::from_str(&contents).expect("JSON does not match expected format");

    // Rust 側での計算
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);
    let v_add = v1 + v2;
    let v_sub = v1 - v2;
    let dot = v1.dot(v2);
    let v_cross = v1.cross(v2);
    let v1_magnitude = v1.length();
    let v1_normalized = v1.normalized();

    let tol = 1e-10;

    // v1 の比較
    assert!(approx_eq(v1.x, python_results.v1.x, tol));
    assert!(approx_eq(v1.y, python_results.v1.y, tol));
    assert!(approx_eq(v1.z, python_results.v1.z, tol));

    // v2 の比較
    assert!(approx_eq(v2.x, python_results.v2.x, tol));
    assert!(approx_eq(v2.y, python_results.v2.y, tol));
    assert!(approx_eq(v2.z, python_results.v2.z, tol));

    // v1 + v2 の比較
    assert!(approx_eq(v_add.x, python_results.v1_add.x, tol));
    assert!(approx_eq(v_add.y, python_results.v1_add.y, tol));
    assert!(approx_eq(v_add.z, python_results.v1_add.z, tol));

    // v1 - v2 の比較
    assert!(approx_eq(v_sub.x, python_results.v1_sub.x, tol));
    assert!(approx_eq(v_sub.y, python_results.v1_sub.y, tol));
    assert!(approx_eq(v_sub.z, python_results.v1_sub.z, tol));

    // 内積の比較
    assert!(approx_eq(dot, python_results.v1_dot, tol));

    // 外積の比較
    assert!(approx_eq(v_cross.x, python_results.v1_cross.x, tol));
    assert!(approx_eq(v_cross.y, python_results.v1_cross.y, tol));
    assert!(approx_eq(v_cross.z, python_results.v1_cross.z, tol));

    // 大きさ（ノルム）の比較
    assert!(approx_eq(v1_magnitude, python_results.v1_magnitude, tol));

    // 正規化の比較
    assert!(approx_eq(v1_normalized.x, python_results.v1_normalized.x, tol));
    assert!(approx_eq(v1_normalized.y, python_results.v1_normalized.y, tol));
    assert!(approx_eq(v1_normalized.z, python_results.v1_normalized.z, tol));
}
