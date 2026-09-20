#!/bin/bash
set -e

cargo build --release

rm -rf AppDir
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps
mkdir -p AppDir/src

cp target/release/cat_clicker AppDir/usr/bin/cat-clicker
cp src/cat.png AppDir/src/cat.png
cp src/meow.mp3 AppDir/src/meow.mp3
cp src/cat.png AppDir/usr/share/icons/hicolor/256x256/apps/cat-clicker.png
cp src/cat.png AppDir/cat-clicker.png

cat <<EOF >AppDir/cat-clicker.desktop
[Desktop Entry]
Name=Cat Clicker
Exec=cat-clicker
Icon=cat-clicker
Type=Application
Categories=Game;
EOF

cat <<'EOF' >AppDir/AppRun
#!/bin/sh
HERE="$(dirname "$(readlink -f "${0}")")"
export PATH="${HERE}/usr/bin:${PATH}"
cd "${HERE}"
exec "${HERE}/usr/bin/cat-clicker" "$@"
EOF

chmod +x AppDir/AppRun

if [ ! -f "appimagetool-x86_64.AppImage" ]; then
  curl -LO https://github.com/AppImage/AppImageKit/releases/download/13/appimagetool-x86_64.AppImage
  chmod +x appimagetool-x86_64.AppImage
fi

ARCH=x86_64 ./appimagetool-x86_64.AppImage AppDir Cat_Clicker-x86_64.AppImage
