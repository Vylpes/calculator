# Maintainer: Ethan Lane <ethan@vylpes.com>
pkgname=calculator-app
pkgver=${GITHUB_REF_NAME#v} # Remove 'v' prefix from tag
pkgrel=1
pkgdesc="A simple calculator app with both GUI and CLI modes"
arch=('x86_64')
url="https://git.vylpes.xyz/RabbitLabs/calculator"
license=('MIT')

package() {
    install -Dm755 "${srcdir}/../artifact/calculator-app" "${pkgdir}/usr/bin/calculator-app"
}
