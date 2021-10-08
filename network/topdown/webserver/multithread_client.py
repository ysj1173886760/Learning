from socket import *
from threading import Thread

def connect(i):
    clientSocket = socket(AF_INET, SOCK_STREAM)
    host = 'localhost'
    port = 12000
    clientSocket.connect((host, port))

    print("client {} start".format(i))
    clientSocket.sendall("GET /abc.txt HTTP/1.1\r\n\r\n".encode())
    message = clientSocket.recv(2048).decode()
    while len(message) != 0:
        print(message)
        message = clientSocket.recv(2048).decode()
    clientSocket.close()
    print("client {} close".format(i))

threads = []

for i in range(10):
    newThread = Thread(target=connect, args=[i])
    newThread.start()
    threads.append(newThread)