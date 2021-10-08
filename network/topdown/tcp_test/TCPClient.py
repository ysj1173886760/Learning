from socket import *

serverName = gethostname()
serverPort = 12000

clientSocket = socket(AF_INET, SOCK_STREAM)
clientSocket.connect((serverName, serverPort))

sentence = input("Input sentence:")
clientSocket.send(sentence.encode())
modifiedSentence = clientSocket.recv(2048)
print("received {} from server".format(modifiedSentence.decode()))
clientSocket.close()