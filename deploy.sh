#!/bin/bash

# Deploy script for Cloudflare Workers
# Solves the "cargo: not found" error in deployment environments

echo "🚀 Starting Cloudflare Workers deployment..."

# Check if wrangler is installed
if ! command -v wrangler &> /dev/null; then
    echo "❌ Error: wrangler is not installed"
    echo "💡 Install it with: npm install -g @cloudflare/wrangler"
    exit 1
fi

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo (Rust) is not installed"
    echo "💡 Install it from: https://rustup.rs/"
    exit 1
fi

# Check if worker-build is installed
if ! command -v worker-build &> /dev/null; then
    echo "📦 Installing worker-build..."
    cargo install worker-build
fi

echo "🔨 Building Worker..."
worker-build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful"
    echo "🚀 Deploying to Cloudflare..."
    wrangler deploy
    
    if [ $? -eq 0 ]; then
        echo "🎉 Deployment successful!"
        echo "🌐 Your Worker is available at: https://where-am-i.your-subdomain.workers.dev"
    else
        echo "❌ Deployment failed"
        exit 1
    fi
else
    echo "❌ Build failed"
    exit 1
fi
