@echo off
REM Deploy script for Cloudflare Workers
REM Solves the "cargo: not found" error in deployment environments

echo 🚀 Starting Cloudflare Workers deployment...

REM Check if wrangler is installed
where wrangler >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ Error: wrangler is not installed
    echo 💡 Install it with: npm install -g @cloudflare/wrangler
    exit /b 1
)

REM Check if cargo is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ❌ Error: cargo (Rust) is not installed
    echo 💡 Install it from: https://rustup.rs/
    exit /b 1
)

REM Check if worker-build is installed
where worker-build >nul 2>nul
if %errorlevel% neq 0 (
    echo 📦 Installing worker-build...
    cargo install worker-build
)

echo 🔨 Building Worker...
worker-build --release

if %errorlevel% equ 0 (
    echo ✅ Build successful
    echo 🚀 Deploying to Cloudflare...
    wrangler deploy
    
    if %errorlevel% equ 0 (
        echo 🎉 Deployment successful!
        echo 🌐 Your Worker is available at: https://where-am-i.your-subdomain.workers.dev
    ) else (
        echo ❌ Deployment failed
        exit /b 1
    )
) else (
    echo ❌ Build failed
    exit /b 1
)
