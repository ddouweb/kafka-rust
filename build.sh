#!/bin/bash

#cargo build --release --target x86_64-unknown-linux-musl

# 获取支持的平台列表
PLATFORMS=$(docker buildx inspect --bootstrap | grep "Platforms:" | cut -d: -f2 | tr -d ' ')

# 检查是否获取到平台列表
if [ -z "$PLATFORMS" ]; then
  echo "无法获取支持的平台列表。"
  exit 1
fi

echo "构建支持的平台: $PLATFORMS"

# 构建镜像
docker buildx build \
  --platform $PLATFORMS \
  -t app .

  #test  cargo test -p storage -- --nocapture