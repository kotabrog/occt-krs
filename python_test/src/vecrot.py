#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
このスクリプトは、OCPパッケージを利用してOCCT APIのgp_Vecクラスを操作し、
以下の3次元ベクトル演算を実施して、その結果をJSON形式で出力します。

操作内容:
    - v1 = (1, 2, 3) および v2 = (4, 5, 6) の生成
    - 加算 (v1 + v2)
    - 減算 (v1 - v2)
    - 内積 (v1 ・ v2)
    - 外積 (v1 x v2)
    - v1の大きさ（ノルム）
    - v1の正規化

出力結果は "python_vector_results.json" に保存されます。
"""
import os
import json
from OCP.gp import gp_Vec

def vector_to_dict(vec: gp_Vec) -> dict:
    """gp_Vecの各成分を辞書形式に変換する"""
    return {
        "x": vec.X(),
        "y": vec.Y(),
        "z": vec.Z()
    }

def main():
    # 例として、v1 = (1, 2, 3) および v2 = (4, 5, 6) のベクトルを生成
    v1 = gp_Vec(1.0, 2.0, 3.0)
    v2 = gp_Vec(4.0, 5.0, 6.0)
    
    # 加算: gp_VecはPython上で直接の演算子オーバーロードがないため、各成分ごとに計算
    v_add = gp_Vec(v1.X() + v2.X(), v1.Y() + v2.Y(), v1.Z() + v2.Z())
    
    # 減算
    v_sub = gp_Vec(v1.X() - v2.X(), v1.Y() - v2.Y(), v1.Z() - v2.Z())
    
    # 内積
    dot = v1.Dot(v2)
    
    # 外積
    cross = v1.Crossed(v2)
    
    # v1の大きさ（ノルム）
    magnitude = v1.Magnitude()
    
    # 正規化: Normalized() メソッドにより正規化済みの新たなgp_Vecを取得
    v1_normalized = v1.Normalized()
    
    # 結果を辞書にまとめる
    output = {
        "v1": vector_to_dict(v1),
        "v2": vector_to_dict(v2),
        "v1 + v2": vector_to_dict(v_add),
        "v1 - v2": vector_to_dict(v_sub),
        "v1 dot v2": dot,
        "v1 cross v2": vector_to_dict(cross),
        "v1 magnitude": magnitude,
        "v1 normalized": vector_to_dict(v1_normalized)
    }

    # 結果出力先のディレクトリを指定 (result)
    result_dir = "../result"
    os.makedirs(result_dir, exist_ok=True)
    output_path = os.path.join(result_dir, "python_vector_results.json")

    # JSONファイルに出力
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(output, f, indent=4, ensure_ascii=False)
    
    print("結果を python_vector_results.json に出力しました。")

if __name__ == "__main__":
    main()
