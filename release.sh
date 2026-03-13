#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:?Usage: ./release.sh <version>   e.g. ./release.sh 0.2.0}"
REPO="DIvkov575/sequences-rs"
URL="https://github.com/$REPO/archive/refs/tags/v$VERSION.tar.gz"

echo "Fetching tarball for v$VERSION..."
curl -sL "$URL" -o /tmp/sequences-rs-"$VERSION".tar.gz

SHA256=$(shasum -a 256 /tmp/sequences-rs-"$VERSION".tar.gz | awk '{print $1}')
echo "sha256: $SHA256"

sed "s/{{version}}/$VERSION/g; s/{{sha256}}/$SHA256/g" \
    sequences_template.rb > sequences-rs.rb

echo "Formula written to sequences-rs.rb"
echo ""
echo "Install:   brew install --formula ./sequences-rs.rb"
echo "Reinstall: brew reinstall --formula ./sequences-rs.rb"
