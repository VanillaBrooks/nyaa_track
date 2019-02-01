import requests
import bs4

t= requests.get("https://nyaa.si")
soup = bs4.BeautifulSoup(t.text)

print(soup)
# t = t.text()

# for i in t:
	# print(i)
