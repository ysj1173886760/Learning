from socket import *
import sys

serverName = ''
serverSocket = socket(AF_INET, SOCK_STREAM)
port = 12000

serverSocket.bind((serverName, port))
serverSocket.listen(1)

print('Ready to Serve')
connectionSocket, addr = serverSocket.accept()
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

serverSocket.close()
sys.exit()