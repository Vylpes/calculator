# Maintainer: Ethan Lane <ethan@vylpes.com>
pkgname=vyxcalc
pkgver=${GITHUB_REF_NAME#v} # Remove 'v' prefix from tag
pkgrel=1
pkgdesc="A simple calculator app with both GUI and CLI modes"
arch=('x86_64')
url="https://git.vylpes.xyz/RabbitLabs/calculator"
license=('MIT')

package() {
    install -Dm755 "${srcdir}/../artifact/vyxcalc" "${pkgdir}/usr/bin/vyxcalc"
}
