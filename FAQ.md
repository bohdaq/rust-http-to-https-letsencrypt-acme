[Read Me](README.md) > FAQ

# Frequently Asked Questions

## #1 
I'm getting following error:
> unable to set up TCP listener: Permission denied (os error 13)

Try to run http-to-https-letsencrypt with admin privileges.

## #2 
I'm getting following error:
> unable to set up TCP listener: Address already in use (os error 48)

Some application is already using port 80. 
Find out PID and stop it.

> sudo fuser 80/tcp
> 
> sudo lsof -i :80