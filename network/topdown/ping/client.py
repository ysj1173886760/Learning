import time
from socket import *

clientSocket = socket(AF_INET, SOCK_DGRAM)
host = 'localhost'
port = 12000

clientSocket.settimeout(1)

for i in range(10):
    message = f"ping {i} {time.time()}"
    start = time.time()
    clientSocket.sendto(message.encode(), (host, port))
    try:
        response, addr = clientSocket.recvfrom(2048)
        print(response.decode())
    except Exception:
        print("packet loss")
    end = time.time()
    print(f"RTT {i}:{(end - start) * 1000:.3f} ms")