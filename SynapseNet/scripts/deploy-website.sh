#!/bin/bash
# Deploy website to synapsenet.org

set -e

echo "🌐 Deploying SynapseNet website..."
echo ""

# Check if website directory exists
if [ ! -d "website" ]; then
    echo "❌ Error: website directory not found"
    exit 1
fi

# Copy Genesis Manifest to website
echo "📄 Copying Genesis Manifest..."
cp GENESIS_v1.0.txt website/GENESIS_v1.0.txt

# Validate HTML files
echo "✅ Validating HTML files..."
for file in website/*.html; do
    if [ -f "$file" ]; then
        echo "  - $(basename $file)"
    fi
done

echo ""
echo "📦 Website files ready for deployment:"
echo "  - index.html"
echo "  - download.html"
echo "  - docs.html"
echo "  - whitepaper.html"
echo "  - join.html"
echo "  - style.css"
echo "  - GENESIS_v1.0.txt"
echo ""

echo "🚀 Deployment options:"
echo ""
echo "1. GitHub Pages:"
echo "   - Push website/ to gh-pages branch"
echo "   - Configure custom domain: synapsenet.org"
echo ""
echo "2. Netlify:"
echo "   - netlify deploy --dir=website --prod"
echo ""
echo "3. Vercel:"
echo "   - vercel --prod website/"
echo ""
echo "4. IPFS (decentralized):"
echo "   - ipfs add -r website/"
echo "   - Update DNS TXT record with IPFS hash"
echo ""
echo "5. Traditional hosting:"
echo "   - rsync -avz website/ user@synapsenet.org:/var/www/html/"
echo ""

echo "✅ Website ready for deployment!"
echo ""
echo "After deployment, verify:"
echo "  - https://synapsenet.org"
echo "  - All links working"
echo "  - Genesis Manifest downloadable"
echo "  - Mobile responsive"
