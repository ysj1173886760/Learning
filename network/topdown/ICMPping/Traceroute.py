from socket import *
import os
import sys
import struct
import time
import select
import binascii  
		
ICMP_ECHO_REQUEST = 8
MAX_HOPS = 30
TIMEOUT  = 1.0 

def checksum(str):
    csum = 0
    countTo = (len(str) / 2) * 2
    count = 0
    while count < countTo:
        thisVal = str[count + 1] * 256 + str[count]
        csum = csum + thisVal
        csum = csum & 0xffffffff
        count = count + 2
    if countTo < len(str):
        csum = csum + str[len(str) - 1].decode()
        csum = csum & 0xffffffff
    # round adding?
    csum = (csum >> 16) + (csum & 0xffff)
    csum = csum + (csum >> 16)
    # upper 16 bit will be ingore
    answer = ~csum
    answer = answer & 0xffff
    # change endian, small to big endian
    answer = answer >> 8 | (answer << 8 & 0xff00)
    return answer

def build_packet(seq):
	# Header is type (8), code (8), checksum (16), sequence (16)
	myChecksum = 0
	# Make a dummy header with a 0 checksum
	# struct -- Interpret strings as packed binary data
	ID = os.getpid() & 0xFFFF
	header = struct.pack("!bbHHh", ICMP_ECHO_REQUEST, 0, myChecksum, ID, seq)
	data = struct.pack("!d", time.time())
	# Calculate the checksum on the data and the dummy header.
	myChecksum = checksum(header + data)
	
	header = struct.pack("!bbHHh", ICMP_ECHO_REQUEST, 0, myChecksum, ID, seq)
	packet = header + data

	return packet

def get_route(hostname):
	print("tracing {}".format(hostname))
	for ttl in range(1, MAX_HOPS):
		destAddr = gethostbyname(hostname)
		# SOCK_RAW is a powerful socket type. For more details:   http://sock-raw.org/papers/sock_raw
		mySocket = socket(AF_INET, SOCK_RAW, IPPROTO_ICMP)
		# setsockopt method is used to set the time-to-live field. 
		mySocket.setsockopt(IPPROTO_IP, IP_TTL, struct.pack('I', ttl))
		mySocket.settimeout(TIMEOUT)
		try:
			d = build_packet(ttl)
			mySocket.sendto(d, (destAddr, 1))
			t= time.time()
			whatReady = select.select([mySocket], [], [], TIMEOUT)
			if whatReady[0] == []: # Timeout
				print("  *        *        *    Request timed out.")
			recvPacket, addr = mySocket.recvfrom(1024)
			timeReceived = time.time()
			
		except timeout:
			continue			
		
		else:
			# Fetch the ICMP type and code from the received packet
			types, code, checksum, packetId, seq = struct.unpack("!bbHHh", recvPacket[20:28])

			if types == 11:
				bytes = struct.calcsize("!d") 
				timeSent = struct.unpack("!d", recvPacket[28:28 + bytes])[0]
				print("  %d    rtt=%.0f ms    %s" %(ttl, (timeReceived -t)*1000, addr[0]))
			
			elif types == 3:
				bytes = struct.calcsize("!d") 
				timeSent = struct.unpack("!d", recvPacket[28:28 + bytes])[0]
				print("  %d    rtt=%.0f ms    %s" %(ttl, (timeReceived-t)*1000, addr[0]))
				print("done")
			
			elif types == 0:
				bytes = struct.calcsize("!d") 
				timeSent = struct.unpack("!d", recvPacket[28:28 + bytes])[0]
				print("  %d    rtt=%.0f ms    %s" %(ttl, (timeReceived - timeSent)*1000, addr[0]))
				print("echoing {} {}".format(packetId, seq))
				return
		
			else:
				print("error")			
		finally:				
			mySocket.close()		
get_route("neub607.xyz")	

