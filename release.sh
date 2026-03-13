#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:?Usage: ./release.sh <version>   e.g. ./release.sh 0.2.0}"
REPO="DIvkov575/sequences-rs"
TAP_REPO="DIvkov575/homebrew-sequences-rs"
URL="https://github.com/$REPO/archive/refs/tags/v$VERSION.tar.gz"

# tag and push the source repo so the tarball exists on GitHub
git tag "v$VERSION"
git push origin "v$VERSION"

echo "Fetching tarball for v$VERSION..."
curl -sL "$URL" -o /tmp/sequences-rs-"$VERSION".tar.gz

SHA256=$(shasum -a 256 /tmp/sequences-rs-"$VERSION".tar.gz | awk '{print $1}')
echo "sha256: $SHA256"

TAP_DIR=$(mktemp -d)
git clone "https://github.com/$TAP_REPO.git" "$TAP_DIR"

mkdir -p "$TAP_DIR/Formula"
sed "s/{{version}}/$VERSION/g; s/{{sha256}}/$SHA256/g" \
    sequences_template.rb > "$TAP_DIR/Formula/sequences-rs.rb"

git -C "$TAP_DIR" add Formula/sequences-rs.rb
git -C "$TAP_DIR" commit -m "sequences-rs $VERSION"
git -C "$TAP_DIR" push

rm -rf "$TAP_DIR"
echo "Done. Tap updated to v$VERSION."
