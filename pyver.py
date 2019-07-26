import requests
import bs4
from pprint import pprint
import urllib
import bencoder

# returns dictionary with (key: hex), (value: character)
def hex_to_char_dict():
	letters = "ABCDEF"
	hex = []
	for k in range(2,8):
		new = [str(i) for i in range(k*10,(k+1)*10)] + [str(k) + letters[j] for j in range(6)]
		hex += new
	hex.pop(-1)
	chars = '! " # $ % & \' ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = > ? @ A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \\ ] ^ _ ` a b c d e f g h i j k l m n o p q r s t u v w x y z { | } ~'.split(' ')
	chars = [" "] + chars
	return {hex[i]: chars[i] for i in range(len(hex))}

# primitive bencoding for strings
def bencode_str(instr):
	return f"{len(instr)}:{instr}"

# percent encoding urls based on two sequential bytes
def url_encoding(hex):
	converter = hex_to_char_dict()

	ret = ''

	for i in range(0, len(hex), 2):

		ab = hex[i:i+2]
		conversion = converter.get(ab)
		print(f"chars are {ab}")

		if conversion is None:
			new_conv = "%" + ab
			ret += new_conv

		else:
			new_conv = urllib.parse.quote(conversion)
			print(f"quoting {ab} to {conversion} result: {new_conv}")
			ret += new_conv


		print(ret)
	return ret

# ensures that the test case functions correctly
def test():
	ida = "123456789abcdef123456789abcdef123456789a"
	k = url_encoding(ida)
	print(k)
	print('%124Vx%9a%bc%de%f1%23Eg%89%ab%cd%ef%124Vx%9a')
	assert(r'%124Vx%9a%bc%de%f1%23Eg%89%ab%cd%ef%124Vx%9a' == k)

#magnet:?xt=urn:btih:0187aa8b9ab3ef51afd1737a4be49e3ec1711cb0&dn=%5BNSBC%5D%20Shitsuji%20Saionji%20no%20Meisuiri%202%20E02%20%5BWEBDL%5D%20%5B720p%5D&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce
def pull_stats(info_hash):
	url_hash = url_encoding(info_hash)
	port = 9932
	peer_id = url_hash
	numwant = 20


	url = f'https://nyaa.tracker.wf:7777/announce?info_hash={url_hash}&peer_id={url_hash}&port={port}&uploaded=0&downloaded=0&numwant={numwant}&compact=1'
	#url = f'http://nyaa.tracker.wf:7777/scrape?info_hash={url_hash}'
	print(f'url is: {url}')

	#req = urllib.request.urlopen(url).read()
	#pprint(req)
	#pprint(bencoder.decode(req))
#%de%f20%00%91T%13%4e%bc%bf%91%d3xV%13d%00v%a5%05


if __name__ == "__main__":
	url = "578d141ae78bfb5629fa1be94b00a8a6d0f2553a"
	print(url)
	print(url_encoding(url))

	pull_stats(url)


