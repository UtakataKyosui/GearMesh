#!/bin/bash
set -e

echo "🔧 Building GearMesh example..."
echo ""

# Build backend
echo "📦 Building backend..."
cd backend
cargo build
echo "✅ Backend built"
echo ""

# Generate TypeScript types
echo "🔄 Generating TypeScript types..."
(cd ../../../crates/gear-mesh-cli && cargo run -- generate --input ../../examples/axum-react/backend/src --output ../../examples/axum-react/frontend/src/types)
echo "✅ Types generated"
echo ""

# Install frontend dependencies
echo "📦 Installing frontend dependencies..."
cd ../frontend
npm install
echo "✅ Dependencies installed"
echo ""

echo "✨ Build complete!"
echo ""
echo "To run the example:"
echo "  1. Start backend:  cd backend && cargo run"
echo "  2. Start frontend: cd frontend && npm run dev"
