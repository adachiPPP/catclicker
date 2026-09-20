pkgname=cat-clicker
pkgver=0.1.0
pkgrel=1
pkgdesc="A dark cat clicker application written in Rust using egui and rodio"
arch=('x86_64')
url="https://localhost/cat-clicker"
license=('MIT')
depends=('alsa-lib' 'libxkbcommon' 'gcc-libs')
makedepends=('cargo' 'pkg-config')
source=()

build() {
  cd "$startdir"
  cargo build --release --locked
}

package() {
  cd "$startdir"

  install -Dm755 "target/release/cat_clicker" "$pkgdir/usr/bin/cat-clicker"

  install -d "$pkgdir/usr/share/cat-clicker/src"
  install -m644 "src/cat.png" "$pkgdir/usr/share/cat-clicker/src/cat.png"
  install -m644 "src/meow.mp3" "$pkgdir/usr/share/cat-clicker/src/meow.mp3"

  install -d "$pkgdir/usr/share/applications"
  cat <<EOF >"$pkgdir/usr/share/applications/cat-clicker.desktop"
[Desktop Entry]
Name=Cat Clicker
Comment=meow
Exec=cat-clicker
Icon=cat-clicker
Terminal=false
Type=Application
Categories=Game;Utility;
Path=/usr/share/cat-clicker
EOF

  install -Dm644 "src/cat.png" "$pkgdir/usr/share/icons/hicolor/256x256/apps/cat-clicker.png"
}
