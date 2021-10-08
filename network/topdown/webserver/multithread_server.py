from socket import *
from threading import Thread
import sys

serverName = ''
serverSocket = socket(AF_INET, SOCK_STREAM)
port = 12000

serverSocket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
serverSocket.bind((serverName, port))
serverSocket.listen(5)

def thread(connectionSocket):
    try:
        message = connectionSocket.recv(2048).decode()
        print(message)
        filename = message.split()[1][1:]
        with open(filename) as f:
            data = f.read()
        connectionSocket.sendall("HTTP/1.1 200 ok\r\n\r\n".encode())
        connectionSocket.sendall(data.encode())
        connectionSocket.sendall("\r\n".encode())
    except IOError:
        print("can not find file")
        connectionSocket.sendall("HTTP/1.1 404 not found\r\n\r\n".encode())
    connectionSocket.close()

print('Ready to Serve')

threads = []

while True:
    connectionSocket, addr = serverSocket.accept()
    newThread = Thread(target=thread, args=[connectionSocket])
    newThread.start()
    threads.append(newThread)

serverSocket.close()
sys.exit()