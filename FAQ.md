[Read Me](README.md) > FAQ

# Frequently Asked Questions

## Problem #1 
I'm getting following error:
> unable to set up TCP listener: Permission denied (os error 13)

### Solution
Try to run http-to-https-letsencrypt with admin privileges.

## Problem #2 
I'm getting following error:
> unable to set up TCP listener: Address already in use (os error 48)


### Solution
Some application is already using port 80. 
Find out PID and stop it.

> sudo fuser 80/tcp # works on linux
> 
> sudo lsof -i :80 # works on macOS as well as on linux

## #3
I started http-to-https-letsencrypt on http://127.0.0.1:80, 
but unable to query it from local network.

### Solution
Server is running on loopback device. Find out ip address 
of you network device and restart http-to-https-letsencrypt
using provided ip.

> ifconfig # find ip address macOS or linux
> 
> ipconfig #windows