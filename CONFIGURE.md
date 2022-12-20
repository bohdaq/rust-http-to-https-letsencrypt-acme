[Read Me](README.md) > Configuration

# Configuration Info
The http-to-https-letsencrypt can be started without any configuration. The following is the default config - the server will bind to IP 127.0.0.1 and port 7878, will spawn 200 threads, CORS requests are allowed.

The http-to-https-letsencrypt will try to read configuration from [system environment variables](https://github.com/bohdaq/rust-http-to-https-letsencrypt-acme/blob/main/rws.variables) first, then it will override configuration
by reading it from file named [rws.config.toml](https://github.com/bohdaq/rust-http-to-https-letsencrypt-acme/blob/main/rws.config.toml) placed in the same directory where you execute http-to-https-letsencrypt, at last it will
apply config provided via [command-line arguments](https://github.com/bohdaq/rust-http-to-https-letsencrypt-acme/blob/main/rws.command_line).


