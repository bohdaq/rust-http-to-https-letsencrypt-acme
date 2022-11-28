# Welcome to rust-http-to-https-letsencrypt-acme!

Hi, rust-http-to-https-letsencrypt-acme is an application web-server for making permanent redirects from http to https. 

It has built in support for Let'sEncrypt HTTP verification. This means that requests starting from
./well-known/acme-challenge will be served as static content and not redirected to https

