#!/usr/bin/env bash
set -euo pipefail

APP_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
BACKEND_DIR="$APP_DIR/backend"

if ! command -v docker >/dev/null 2>&1; then
  echo "Docker belum terpasang atau tidak ada di PATH." >&2
  exit 1
fi

if ! docker info >/dev/null 2>&1; then
  echo "Docker belum berjalan atau akun ini belum punya akses ke Docker." >&2
  exit 1
fi

for container in pkl-redisinsight toko-laptop-postgres; do
  if ! docker container inspect "$container" >/dev/null 2>&1; then
    echo "Container '$container' tidak ditemukan. Buat container itu terlebih dahulu." >&2
    exit 1
  fi
done

# Redis dikelola dengan Compose; layanan lain sudah dibuat sebelumnya.
(cd "$BACKEND_DIR" && docker compose up -d redis)
docker start pkl-redisinsight toko-laptop-postgres >/dev/null

echo
echo "Container aktif:"
for container in backend-redis-1 toko-laptop-postgres pkl-redisinsight; do
  docker inspect --format '{{.Name}}: {{.State.Status}}' "$container"
done

echo
echo "RedisInsight: http://localhost:5540"
