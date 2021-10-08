from socket import *

clientSocket = socket(AF_INET, SOCK_STREAM)
host = 'localhost'
port = 12000
clientSocket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
clientSocket.connect((host, port))

clientSocket.sendall("GET /abc.txt HTTP/1.1\r\n\r\n".encode())
message = clientSocket.recv(2048).decode()
while len(message) != 0:
    print(message)
    message = clientSocket.recv(2048).decode()
clientSocket.close()