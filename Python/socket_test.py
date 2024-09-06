# okay, the first thing we need to do is to create the socket object
# do this in your editor

import socket

# the default is ipv4 socket stream (which is tcp)
# but it's usually a good idea to spell it out for readability
# so where we could just leave the below alone, we'll change it
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# we next need to set this socket for reuse, if we don't, then 
# the kernel will keep the socket in use for ~5 minutes, by socket
# i mean the ip:port we're going to bind to. We MUST to this RIGHT
# AFTER creating the socket, as it has to be done before using the
# socket.
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

# default behavior for using a socket is to bind to a random port, 
# we'll accept that default for now

# to connect to a server, we do the following (requires a tuple)
# so the argument we're passing to s.connect() is ('google.com', 80)
s.connect(('google.com, 80))

# now our loop
while True:
    
    # at most basic, we read, let's say 4096 bytes
    recv = s.recv(4096)
    if recv:
        print recv
    else:
        # eof
        socket.close()
        
# summary, try running this (i do make mistakes ya know)
import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.connect(('google.com'))

while True:
    recv = s.recv(4096)
    if recv:
        print recv
    else:
        socket.close()
