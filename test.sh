#!/bin/sh -e
binpathfix="${CARGO_TARGET_DIR:+${CARGO_TARGET_DIR}/debug/}binpathfix"
tmp="$(mktemp -u)"
## Create binary compiled in "/src/project".
cat > "$tmp.1" <<-EOF
kaaisuas sjadhjashdsd djashdghjasd hghghgasd hgasdjhgasdjhb
asjdkasdj /src/project/main.c asnja ajsb /src/project/lib.c a.
EOF

## Perform replace.
cp "$tmp.1" "$tmp.2"
"${binpathfix}" -v -o "/src/project" "$tmp.2"

## Show result.
echo ""
cat "$tmp.1"
echo "                     |"
echo "                     V"
cat "$tmp.2"
echo ""
