from socket import *

serverPort = 12000
serverSocket = socket(AF_INET, SOCK_STREAM)
# For IPv4 addresses, two special forms are accepted instead of a host address: '' represents INADDR_ANY, which is used to bind to all interfaces
serverSocket.bind(('', serverPort))
serverSocket.listen(1)

while True:
    connectionSocket, addr = serverSocket.accept()
    sentence = connectionSocket.recv(2048).decode()
    connectionSocket.send(sentence.upper().encode())
    connectionSocket.close()