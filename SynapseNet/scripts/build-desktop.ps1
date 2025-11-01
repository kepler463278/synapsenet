# Build script for SynapseNet Desktop v1.0 (Windows PowerShell)

Write-Host "🚀 Building SynapseNet Desktop v1.0..." -ForegroundColor Cyan
Write-Host ""

# Navigate to desktop app
Set-Location apps/desktop

Write-Host "📦 Installing dependencies..." -ForegroundColor Blue
npm install

Write-Host ""
Write-Host "🔨 Building application..." -ForegroundColor Blue
npm run tauri build

Write-Host ""
Write-Host "✅ Build complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Binaries location:"
Write-Host "  - Windows: src-tauri\target\release\bundle\nsis\"
Write-Host ""
Write-Host "🎉 SynapseNet Desktop v1.0 is ready!" -ForegroundColor Green
