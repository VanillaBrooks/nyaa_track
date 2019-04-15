import requests
import bs4
from pprint import pprint
import urllib


ida = "73cfad05c0532129a4d2fe236085a9122311e455"
cpy = ''
L = len(ida)
digit = False
letter = False
for i in range(L):
	cur = ida[i]
	if i != 0:
		if ida[i-1].isdigit() and ida[i].isalpha():
			cpy += "%"
	cpy += cur
# after a number befor ea letter
	
urllib.parse.quote(ida)
print(ida)
print(cpy)

ip = "91.207.175.232"
port = 2345
peer_id = "ASDADSIBAOSDBIOBAS"

url = f"http://nyaa.tracker.wf:7777/announce?info_hash={cpy}&peer_id={peer_id}"#&ip={ip}&port={port}"

req = requests.get(url)
soup = bs4.BeautifulSoup(req.text)
# pprint(dir(req))
print(req.status_code)

# pprint(soup.prettify())
