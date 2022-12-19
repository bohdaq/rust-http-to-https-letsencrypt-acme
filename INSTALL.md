[Read Me](README.md) > Install

## Install
[Download binary](https://github.com/bohdaq/rust-web-server/releases) for you platform from releases page.
There is a mirror for downloads on [Google Drive](https://drive.google.com/drive/folders/13iSR3VxmfFvZgOZ0LddP_EJp7GJ-lQd8?usp=sharing).
### x86 64-bit Apple macOS
> sudo cp http-to-https-letsencrypt /usr/local/bin
>
> sudo chmod ug+rwx,o+r /usr/local/bin/http-to-https-letsencrypt
#### x86 64-bit Homebrew macOS
> brew tap bohdaq/rust-web-server
>
> brew install http-to-https-letsencrypt

### x86 64-bit Linux
> sudo cp http-to-https-letsencrypt /usr/local/bin
>
> sudo chmod ug+rwx,o+r /usr/local/bin/http-to-https-letsencrypt
#### x86 64-bit Debian
> sudo dpkg -i --force-overwrite http-to-https-letsencrypt.deb
#### x86 64-bit RPM
Replace _YOUR_VERSION_ with version you downloaded.
> sudo rpm -i --force http-to-https-letsencrypt-_YOUR_VERSION_.rpm

### ARM 64-bit Linux
> sudo cp http-to-https-letsencrypt /usr/local/bin
>
> sudo chmod ug+rwx,o+r /usr/local/bin/http-to-https-letsencrypt
#### ARM 64-bit Debian
> sudo dpkg -i --force-overwrite http-to-https-letsencrypt.deb

### x86 64-bit Windows
Copy executable to _C:\WINDOWS\system32_ folder.


### Testing installation
To check installation execute the following code in the terminal:

> $ http-to-https-letsencrypt

You will see similar output:

> http-to-https-letsencrypt
>
> Version:       YOUR_VERSION
>
> Authors:       Bohdan Tsap <bohdan.tsap@tutanota.com>
>
> Repository:    https://github.com/bohdaq/rust-web-server
>
> Desciption:    http-to-https-letsencrypt is an application web-server for making permanent redirects from http to https.
>
> Rust Version:  RUST_VERSION
> 
> ...
> Server is up and running: http://127.0.0.1:80


Open browser, go to http://127.0.0.1:80, you'll see default page.

Go back to terminal, press Ctrl + C (or CMD + C) to stop server.