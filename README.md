# http-to-https-letsencrypt

# Repository moved to [bohdaq/http-to-https-letsencrypt](https://github.com/bohdaq/http-to-https-letsencrypt)

#
#######

**http-to-https-letsencrypt** is an application web-server for making permanent redirects from http to https. 

It has built in support for Let'sEncrypt HTTP verification. This means that requests starting from
./well-known/acme-challenge will be served as static content and not redirected to https.

So basically you download, install and run http-to-https-letsencrypt server, 
and then run [certbot](https://certbot.eff.org/) in _--webroot_ mode alongside.

## Download
[Download binary](https://github.com/bohdaq/rust-http-to-https-letsencrypt-acme/releases) from releases page.  There is a mirror for downloads on [Google Drive](https://drive.google.com/drive/folders/1wize_LrCwGgs4MSvldhbpqDx4UTD3uEk?usp=share_link).

## Installation
Open [INSTALL](INSTALL.md) for details.

## Development
Open [DEVELOPER](DEVELOPER.md) for details.

## Configuration
Open [CONFIGURE](CONFIGURE.md) for details.

## Frequently Asked Questions
Open [FAQ](FAQ.md) for details.

## Community
Use GitHub discussions, issues and pull requests.

There is Rust Web Server [Discord](https://discord.gg/zaErjtr5Dm) where you can ask questions and share ideas.

Follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Donations
If you appreciate my work and want to support it, feel free to do it via [PayPal](https://www.paypal.com/donate/?hosted_button_id=7J69SYZWSP6HJ).

## Links
1. [Rust TLS Server](https://github.com/bohdaq/rust-tls-server)
1. [Rust Web Server](https://github.com/bohdaq/rust-web-server)
1. [Rust Web Framework](https://github.com/bohdaq/rust-web-framework/)
1. [Create Debian Package](https://github.com/bohdaq/rws-create-deb)
1. [Create RPM Package](https://github.com/bohdaq/rws-rpm-builder)
1. [Homebrew Formula](https://github.com/bohdaq/homebrew-rust-web-server)
1. [crypto-ext](https://github.com/bohdaq/crypto-ext/)
1. [file-ext](https://github.com/bohdaq/file-ext/)
