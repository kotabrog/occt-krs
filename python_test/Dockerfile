FROM ubuntu:22.04

# ロケールなど基本設定
ENV DEBIAN_FRONTEND=noninteractive \
    LANG=C.UTF-8 \
    LC_ALL=C.UTF-8

# CQ-editorで必要なOpenGL / X関連ライブラリをインストール
# ※ GUI はlibgl1, libxcb-xinerama0, libxkbcommon-x11-0 などが必要
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    libgl1-mesa-dev \
    libxkbcommon-x11-0 \
    libxcb-xinerama0 \
    && rm -rf /var/lib/apt/lists/*

# CQ-editor のインストール先ディレクトリ
ENV CQE_INSTALL_DIR=/root/cq-editor

# CQ-editor のインストーラをダウンロードし、インストールして削除
RUN set -ex && \
    curl -LO https://github.com/CadQuery/CQ-editor/releases/download/nightly/CQ-editor-master-Linux-x86_64.sh && \
    sh CQ-editor-master-Linux-x86_64.sh -b -p "$CQE_INSTALL_DIR" && \
    rm CQ-editor-master-Linux-x86_64.sh

# インストールした cq-editor の bin ディレクトリにパスを通す
ENV PATH=$CQE_INSTALL_DIR/bin:$PATH

# Miniconda を /opt/conda にインストール
ENV CONDA_DIR=/opt/conda
RUN curl -LO https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh && \
    bash Miniconda3-latest-Linux-x86_64.sh -b -p $CONDA_DIR && \
    rm Miniconda3-latest-Linux-x86_64.sh

# conda にパスを通す
ENV PATH=$CONDA_DIR/bin:$PATH

# conda本体をアップデート & ベースの設定
RUN conda update -n base -c defaults conda
RUN conda init bash

ARG ENV_NAME=cadquery-env

# environment.yml をコンテナにコピー
COPY environment.yml /tmp/environment.yml
RUN conda env update -n ${ENV_NAME} -f /tmp/environment.yml

# 起動時に${ENV_NAME}を自動有効化する設定
RUN echo "conda activate ${ENV_NAME}" >> /root/.bashrc

# デフォルトはログインシェルで起動し、.bashrcを読み込ませる
CMD ["/bin/bash", "--login"]
