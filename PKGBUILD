# Maintainer: Ethan Lane <ethan@vylpes.com>
pkgname=vyxcalc
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple calculator app with both GUI and CLI modes"
arch=('x86_64')
url="https://git.vylpes.xyz/RabbitLabs/calculator"
license=('MIT')
depends=(
    'gtk4'
    'cairo'
    'pango'
    'glib2'
)
source=("vyxcalc::file://../artifact/vyxcalc")
sha256sums=('SKIP')

prepare() {
    chmod +x "$srcdir/vyxcalc"
}

package() {
    install -Dm755 "${srcdir}/vyxcalc" "${pkgdir}/usr/bin/vyxcalc"
    install -Dm644 "${startdir}/LICENSE" "${pkgdir}/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 "${startdir}/com.vylpes.vyxcalc.desktop" \
        "${pkgdir}/usr/share/applications/com.vylpes.vyxcalc.desktop"
    install -Dm644 "${startdir}/assets/icon.png" \
        "${pkgdir}/usr/share/icons/hicolor/128x128/apps/com.vylpes.vyxcalc.png"
}
