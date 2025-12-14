#!/bin/bash
# Docker E2Eテスト実行スクリプト

set -e

echo "Building Docker image for gear-mesh E2E test..."
docker build -f Dockerfile.test -t gear-mesh-test .

echo ""
echo "Running E2E tests in Docker container..."
docker run --rm gear-mesh-test /workspace/gear-mesh/test-e2e.sh
