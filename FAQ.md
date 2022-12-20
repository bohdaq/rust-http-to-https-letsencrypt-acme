[Read Me](README.md) > FAQ

# Frequently Asked Questions

## Problem #1 
I'm getting following error(**macOS**):
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

## Problem #3
I started http-to-https-letsencrypt on http://127.0.0.1:80, 
but unable to query it from local network.

### Solution
Server is running on loopback device. Find out ip address 
of you network device and restart http-to-https-letsencrypt
using provided ip.

> ifconfig # find ip address macOS or linux
> 
> ipconfig #windows


## Problem #4
I see the following error in the console:
> unable to parse request: invalid utf-8 sequence of _n_ bytes from index _m_

### Solution
Server received not properly encoded request in UTF-8 charset. Request may be sent from various software on your network. You can ignore this message.


## Problem #5
I see the following error in the console:
> unable to parse request: Unable to parse method, request uri and http version

### Solution
Server received not valid request, for example it may contain a typo or [ASCII invisible control characters](https://en.wikipedia.org/wiki/Control_character). Request may be sent from various software on your network. You can ignore this message.

## Problem #6
I see the following error in the console(**Linux**):
> unable to set up TCP listener: Cannot assign requested address (os error 99) 
> 

### Solution
Most probably you are trying to start server on port 80. To start server on port 80 try to run it as an administrator or user with admin privileges.
