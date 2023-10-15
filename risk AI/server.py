from fastapi import FastAPI
from app import pre_update, clear_data, get_data, chatbot, initiated
from pydantic import BaseModel
import uuid 
  
app = FastAPI()

class Ask(BaseModel):
    user_id: str
    message: str
    max_time: float

@app.get("/")
async def root():
    return {"message": "Hello World"}

@app.post("/register")
async def create_user():
  user_id = uuid.uuid4() 
  pre_update(user_id)
  return {"message": user_id}

@app.delete("/clear-history/{user_id}")
async def clear_history(user_id:str):
  clear_data(user_id)
  return {"message": "Cleared History"}

@app.get("/get-history/{user_id}")
async def get_history(user_id: str):
    return {"message": get_data(user_id)}

@app.post("/ask")
async def ask_question(ask: Ask):
    initiated()
    response = chatbot(ask.message, ask.user_id)
    return {"message": response}
  
@app.post("/reinitialize")
async def initialize():
    initiated()
    return {"message": "done"}

# 34160e70-6817-444e-b8c9-60a528ce0446