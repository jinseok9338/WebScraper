import requests
from bs4 import BeautifulSoup as BS


if __name__ == "__main__":
    r = requests.get("https://m.khan.co.kr/politics/assembly/article/202111011034001#c2b")
    soup = BS(r.content, 'html.parser')
    all_text = soup.find_all('p', class_=False, id=False)
    print(all_text)
