import requests
from bs4 import BeautifulSoup as BS
from flask import Flask


app = Flask(__name__)


def scrape():
    response = requests.get("https://www.donga.com/news/Politics/article/all/20211101/110023396/1")
    soup = BS(response.content, 'html.parser')
    all_text = soup.find_all('p', class_=False, id=False)
    print(all_text)
    return ''.join(all_text)

@app.route("/")
def hello():
    all_text = scrape()
    return all_text

if __name__ == "__main__":
    app.run() 
