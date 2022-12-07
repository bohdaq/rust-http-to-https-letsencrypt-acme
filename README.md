# http-to-https-letsencrypt

**http-to-https-letsencrypt** is an application web-server for making permanent redirects from http to https. 

It has built in support for Let'sEncrypt HTTP verification. This means that requests starting from
./well-known/acme-challenge will be served as static content and not redirected to https.

So basically you download, install and run http-to-https-letsencrypt server, and then run [certbot](https://certbot.eff.org/) alongside.
