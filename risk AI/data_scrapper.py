import os
import requests
from bs4 import BeautifulSoup
from googlesearch import search
from dotenv import load_dotenv
load_dotenv()

api_key = os.getenv("GOOGLE_API_KEY")


# Function to search for URLs
def search_for_urls(query, num_results=10):
    search_results = []
    for result in search(query, stop=15, num=num_results, pause=5):
        search_results.append(result)
    return search_results

# Function to scrape a webpage
def scrape_webpage(url):
    try:
        response = requests.get(url, timeout=30)
        if response.status_code == 200:
            soup = BeautifulSoup(response.content, "html.parser")
            # Extract and process data from the webpage
            body_tag = soup.find('body') #soup.get_text()  # Change this to extract specific data
            if body_tag:
                body_content = body_tag.get_text()
                
                # Remove extra whitespace, tabs, and newline characters
                data = ' '.join(body_content.split())
                
                return data+" \n"
    except Exception as e:
        print(f"Error scraping {url}: {str(e)}")
        return None

# Function to save data to a file
def save_to_file(data, filename):
    with open(filename, "a", encoding="utf-8") as file:
        file.write(data + "\n")

def loop_and_scrap():
  queries = [
      "ETHEREUM",
      "BITCOIN",
      "Crypto",
      "SOLANA" #many more topics
  ]

  # Specify the number of search results per query
  num_results_per_query = 10

  # Specify the output file
  output_filename = "sources/main_data.txt"
  # Loop through the queries and scrape data
  for query in queries:
      print(f"Searching for: {query}")
      urls = search_for_urls(query, num_results=num_results_per_query)
      print(len(urls))
      
      for url in urls:
          print(f"Scraping data from: {url}")
          scraped_data = scrape_webpage(url)
          
          if scraped_data:
              # Process the scraped data as needed (e.g., extract specific information)
              # For demonstration purposes, we're saving the entire webpage content.
              save_to_file(scraped_data, output_filename)

loop_and_scrap()