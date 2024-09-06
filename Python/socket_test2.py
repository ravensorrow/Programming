import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
s.bind(('', 4444))
s.listen(5)
#s.setblocking(False)

while True:
    conn, addr = s.accept()
    data = conn.recv(1024)
    if data:
        conn.send(data) # echo server
    conn.close()

