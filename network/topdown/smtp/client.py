from socket import *
import sys
import base64

# Choose a mail server (e.g. Google mail server) and call it mailserver
mailserver =('smtp.163.com', 25)
 
# Create socket called clientSocket and establish a TCP connection with mailserver
#Fill in start
clientSocket = socket(AF_INET, SOCK_STREAM)
clientSocket.connect(mailserver)
#Fill in end
 
recv = clientSocket.recv(1024).decode()
print (recv)
if recv[:3] != '220':
    print('220 reply not received from server.')
    sys.exit()
 
# Send HELO command and print server response.
heloCommand = 'HELO ysj1173886760@163.com\r\n'
clientSocket.send(heloCommand.encode())

recv1 = clientSocket.recv(1024).decode()
print (recv1)
if recv1[:3] != '250':
    print ('250 reply not received from server.')
    sys.exit()
	
loginCommand = 'AUTH LOGIN\r\n'
clientSocket.send(loginCommand.encode())

recv2 = clientSocket.recv(1024).decode()
print (recv2)
if recv2[:3] != '334':
    print ('334 reply not received from server.')
    sys.exit()

username = base64.b64encode(''.encode())
password = base64.b64encode(''.encode())

clientSocket.send(f"{username.decode()}\r\n".encode())
recv3 = clientSocket.recv(1024).decode()
print (recv3)

clientSocket.send(f"{password.decode()}\r\n".encode())
recv4 = clientSocket.recv(1024).decode()
print (recv4)

if recv4[:3] != '235':
    print("failed to login")
    sys.exit()
# Send MAIL FROM command and print server response.
# Fill in start
 
mail_fromCommand = 'MAIL FROM:<ysj1173886760@163.com>\r\n'
clientSocket.send(mail_fromCommand.encode())

recv5 = clientSocket.recv(1024).decode()
print (recv5)
if recv5[:3] != '250':
    print ('250 reply not received from server.')
    sys.exit()
# Fill in end
 
# Send RCPT TO command and print server response.
# Fill in start
rcpt_toCommand = 'RCPT TO:<1173886760@qq.com>\r\n'
clientSocket.send(rcpt_toCommand.encode())

recv6 = clientSocket.recv(1024).decode()
print (recv6)
if recv6[:3] != '250':
    print ('250 reply not received from server.')
    sys.exit()
# Fill in end
 
# Send DATA command and print server response.
# Fill in start
dataCommand = 'DATA\r\n'
clientSocket.send(dataCommand.encode())

recv7 = clientSocket.recv(1024).decode()
print (recv7)
if recv7[:3] != '354':
    print ('354 reply not received from server.')
    sys.exit()
# Fill in end
 
msg = "from:ysj1173886760@163.com\r\n" + \
      "to:1173886760@qq.com\r\n" + \
      "subject:test\r\n" + \
      "\r\n" + \
      "hello world" + \
      "\r\n.\r\n"
 
# Send message data.
# Fill in start
clientSocket.send(msg.encode())
# Fill in end
 
# Message ends with a single period.
# Fill in start
recv8 = clientSocket.recv(1024).decode()

print (recv8)
if recv8[:3] != '250':
    print ('250 reply not received from server.')
    sys.exit()
# Fill in end
 
# Send QUIT command and get server response.
# Fill in start
quitCommand = 'QUIT\r\n'
clientSocket.send(quitCommand.encode())

recv9 = clientSocket.recv(1024)
print (recv9)
if recv9[:3] != '221':
    print ('221 reply not received from server.')
    sys.exit()
# Fill in end
 
print("send successful") 