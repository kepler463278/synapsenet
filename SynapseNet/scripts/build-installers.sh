#!/bin/bash
# SynapseNet v0.4 Cross-Platform Installer Build Script

set -e

VERSION="0.4.0"
APP_NAME="SynapseNet"
BUILD_DIR="target/release"
DIST_DIR="dist"

echo "🚀 Building SynapseNet v${VERSION} installers..."

# Create dist directory
mkdir -p "$DIST_DIR"

# Function to build for a specific platform
build_platform() {
    local platform=$1
    
    echo "📦 Building for $platform..."
    
    cd crates/tauri-app
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        echo "📥 Installing npm dependencies..."
        npm install
    fi
    
    # Build for platform
    case "$platform" in
        "macos")
            cargo tauri build --target universal-apple-darwin
            ;;
        "windows")
            cargo tauri build --target x86_64-pc-windows-msvc
            ;;
        "linux")
            cargo tauri build --target x86_64-unknown-linux-gnu
            ;;
    esac
    
    cd ../..
}

# Function to create checksums
create_checksums() {
    echo "🔐 Creating checksums..."
    cd "$DIST_DIR"
    
    # Create SHA256 checksums for all files
    for file in *; do
        if [ -f "$file" ] && [ "$file" != "checksums.txt" ]; then
            shasum -a 256 "$file" >> checksums.txt
        fi
    done
    
    echo "✅ Checksums created in checksums.txt"
    cd ..
}

# Function to sign macOS app (requires developer certificate)
sign_macos() {
    if [ "$SKIP_SIGNING" != "true" ] && [ -n "$APPLE_DEVELOPER_ID" ]; then
        echo "🔏 Signing macOS application..."
        
        local app_path="crates/tauri-app/target/universal-apple-darwin/release/bundle/macos/${APP_NAME}.app"
        
        if [ -d "$app_path" ]; then
            codesign --force --deep --sign "$APPLE_DEVELOPER_ID" "$app_path"
            codesign --verify --verbose "$app_path"
            echo "✅ macOS app signed successfully"
        else
            echo "⚠️  App bundle not found at $app_path"
        fi
    else
        echo "⚠️  Skipping macOS signing (set APPLE_DEVELOPER_ID to enable)"
    fi
}

# Function to notarize macOS app (requires Apple ID)
notarize_macos() {
    if [ "$SKIP_NOTARIZATION" != "true" ] && [ -n "$APPLE_ID" ] && [ -n "$APPLE_PASSWORD" ]; then
        echo "📋 Notarizing macOS application..."
        
        local app_path="crates/tauri-app/target/universal-apple-darwin/release/bundle/macos/${APP_NAME}.app"
        
        if [ -d "$app_path" ]; then
            cd "$(dirname "$app_path")"
            zip -r "${APP_NAME}.zip" "${APP_NAME}.app"
            
            xcrun notarytool submit "${APP_NAME}.zip" \
                --apple-id "$APPLE_ID" \
                --password "$APPLE_PASSWORD" \
                --team-id "$APPLE_TEAM_ID" \
                --wait
            
            xcrun stapler staple "${APP_NAME}.app"
            
            cd -
            echo "✅ macOS app notarized successfully"
        fi
    else
        echo "⚠️  Skipping macOS notarization (set APPLE_ID and APPLE_PASSWORD to enable)"
    fi
}

# Copy artifacts to dist directory
copy_artifacts() {
    local platform=$1
    
    echo "📋 Copying artifacts for $platform..."
    
    case "$platform" in
        "macos")
            local dmg_path="crates/tauri-app/target/universal-apple-darwin/release/bundle/dmg/${APP_NAME}_${VERSION}_universal.dmg"
            if [ -f "$dmg_path" ]; then
                cp "$dmg_path" "$DIST_DIR/${APP_NAME}-${VERSION}-macos.dmg"
                echo "✅ Copied macOS DMG"
            else
                echo "⚠️  DMG not found at $dmg_path"
            fi
            ;;
            
        "windows")
            local msi_path="crates/tauri-app/target/x86_64-pc-windows-msvc/release/bundle/msi/${APP_NAME}_${VERSION}_x64_en-US.msi"
            if [ -f "$msi_path" ]; then
                cp "$msi_path" "$DIST_DIR/${APP_NAME}-${VERSION}-windows-x64.msi"
                echo "✅ Copied Windows MSI"
            else
                echo "⚠️  MSI not found at $msi_path"
            fi
            ;;
            
        "linux")
            local deb_path="crates/tauri-app/target/x86_64-unknown-linux-gnu/release/bundle/deb/synapsenet_${VERSION}_amd64.deb"
            local appimage_path="crates/tauri-app/target/x86_64-unknown-linux-gnu/release/bundle/appimage/${APP_NAME}_${VERSION}_amd64.AppImage"
            
            if [ -f "$deb_path" ]; then
                cp "$deb_path" "$DIST_DIR/synapsenet-${VERSION}-linux-amd64.deb"
                echo "✅ Copied Linux DEB"
            else
                echo "⚠️  DEB not found at $deb_path"
            fi
            
            if [ -f "$appimage_path" ]; then
                cp "$appimage_path" "$DIST_DIR/${APP_NAME}-${VERSION}-linux-x86_64.AppImage"
                chmod +x "$DIST_DIR/${APP_NAME}-${VERSION}-linux-x86_64.AppImage"
                echo "✅ Copied Linux AppImage"
            else
                echo "⚠️  AppImage not found at $appimage_path"
            fi
            ;;
    esac
}

# Main build logic
case "$1" in
    "macos")
        echo "🍎 Building for macOS..."
        build_platform "macos"
        sign_macos
        notarize_macos
        copy_artifacts "macos"
        echo "✅ macOS build complete!"
        ;;
        
    "windows")
        echo "🪟 Building for Windows..."
        build_platform "windows"
        copy_artifacts "windows"
        echo "✅ Windows build complete!"
        ;;
        
    "linux")
        echo "🐧 Building for Linux..."
        build_platform "linux"
        copy_artifacts "linux"
        echo "✅ Linux build complete!"
        ;;
        
    "all")
        echo "🌍 Building for all platforms..."
        
        # Detect current platform
        if [[ "$OSTYPE" == "darwin"* ]]; then
            $0 macos
        else
            echo "⚠️  Skipping macOS build (requires macOS)"
        fi
        
        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            $0 linux
        else
            echo "⚠️  Skipping Linux build (requires Linux or cross-compilation)"
        fi
        
        # Windows builds typically require Windows or complex cross-compilation
        echo "⚠️  Windows builds should be done on Windows or CI/CD"
        
        create_checksums
        echo "🎉 All available builds complete!"
        ;;
        
    "clean")
        echo "🧹 Cleaning build artifacts..."
        rm -rf "$DIST_DIR"
        rm -rf "crates/tauri-app/target"
        rm -rf "crates/tauri-app/dist"
        rm -rf "crates/tauri-app/node_modules"
        echo "✅ Clean complete"
        ;;
        
    *)
        echo "Usage: $0 {macos|windows|linux|all|clean}"
        echo ""
        echo "Environment variables:"
        echo "  APPLE_DEVELOPER_ID    - Apple Developer ID for code signing"
        echo "  APPLE_ID              - Apple ID for notarization"
        echo "  APPLE_PASSWORD        - App-specific password for notarization"
        echo "  APPLE_TEAM_ID         - Apple Team ID"
        echo "  SKIP_SIGNING=true     - Skip code signing"
        echo "  SKIP_NOTARIZATION=true - Skip notarization"
        echo ""
        echo "Examples:"
        echo "  $0 macos              # Build macOS DMG"
        echo "  $0 windows            # Build Windows MSI"
        echo "  $0 linux              # Build Linux DEB and AppImage"
        echo "  $0 all                # Build all available platforms"
        echo "  $0 clean              # Clean build artifacts"
        exit 1
        ;;
esac

echo "🎊 Build script completed successfully!"
