import datetime
from fastapi import FastAPI,Query, Request
import uvicorn
from pydantic import BaseModel
from typing import Annotated
import time

app = FastAPI()

"""
Task:
Step 1: Create a FastAPI endpoint /books/ that accepts query parameters author (string) and year (integer).
Step 2: Use Pydantic models to validate these parameters, ensuring that author is at least 3 characters long and year is between 1900 and the current year.
Step 3: The endpoint should return a list of books matching the filters from a predefined list or database.
Step 4: If validation fails, return a detailed error message indicating which parameter is invalid and why.
"""

"""
Task:
Step 1: Create a FastAPI middleware that calculates the execution time of each request.
Step 2: Log the method, URL path, and execution time to the console or a log file.
Step 3: Apply this middleware globally so it wraps all endpoints in the application.
Example Log Output:
INFO: Method: GET, Path: /items/, Execution Time: 15ms

"""

current_year = datetime.datetime.now().year

books_db = [
    {"author": "J.K. Rowling", "year": 1997},
    {"author": "J.K. Rowling", "year": 1997},
    {"author": "George Orwell", "year": 1949},
    {"author": "F. Scott Fitzgerald", "year": 1925},
    {"author": "Agatha Christie", "year": 1934},
    {"author": "Leo Tolstoy", "year": 1901},
    {"author": "Yuval Noah Harari", "year": 2015},
]

class Book(BaseModel):
    author:str 
    year:int 


@app.middleware("http")
async def log_request_time(request:Request, call_next):
    start_time = time.perf_counter()
    response = await call_next(request)
    end_time = time.perf_counter()
    time_taken = (end_time-start_time)*1000
    print(f"INFO: Method: {request.method}, Path: {request.url.path}, Execution Time: {time_taken}ms")
    return response




@app.get("/books/", response_model=list[Book])
async def books(author:Annotated[str,Query(min_length=3)],year:Annotated[int,Query(ge=1900,le=current_year)]):
    # Filter books based on year and author
    results =filter(lambda b:(author == b["author"]) and (year == b["year"]),books_db)

    return results
    


if __name__ == "__main__":
    uvicorn.run("app:app", port=8000)