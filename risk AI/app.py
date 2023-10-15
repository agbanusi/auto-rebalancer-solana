from llama_index import SimpleDirectoryReader, VectorStoreIndex
import gradio as gr
import os
import redis
import json
from llama_index.llms import ChatMessage, MessageRole
from dotenv import load_dotenv
load_dotenv()

# Initialize a Redis connection
redis_client = redis.StrictRedis(host='localhost', port=6379, db=0)

api_key = os.getenv("OPENAI_API_KEY")
index = None

def construct_index(directory_path):
    documents = SimpleDirectoryReader(directory_path).load_data()
    indax = VectorStoreIndex.from_documents(documents)

    return indax

def serialize_chat_message(chat_message):
    return {"role": chat_message.role, "content": chat_message.content}

def deserialize_chat_message(data):
    content = data["content"]
    if (data["role"] == "USER"):
        return ChatMessage(role=MessageRole.USER, content=content)
    else:
        return ChatMessage(role=MessageRole.ASSISTANT, content=content)


def add_to_history(message,user_id, ai_or_human):
    context = None
    
    if (ai_or_human == "USER"):
       context = json.dumps({
                "role":MessageRole.USER, 
                "content":message
            
            })
    else:
        context = json.dumps({
                "role":MessageRole.ASSISTANT, 
                "content":message
            
            })    
    redis_client.rpush(f'conversation:{user_id}', context)

def clear_data(user_id):
    redis_client.delete(f'conversation:{user_id}')

def get_data(user_id):
    custom_chat_history = redis_client.lrange(f'conversation:{user_id}', 0, -1)
    custom_chat_history = [deserialize_chat_message(json.loads(msg)) for msg in custom_chat_history]
    return custom_chat_history
    
def is_topic_related_query(index, query, user_id):
  custom_chat_history = redis_client.lrange(f'conversation:{user_id}', 0, -1)
  custom_chat_history = [deserialize_chat_message(json.loads(msg)) for msg in custom_chat_history]

  engine = index.as_query_engine(verbose=True, chat_history=custom_chat_history,)
  query_text = engine.query(query).response
  return query_text.find("I'm sorry, but") == -1

def generate_response(index, query, user_id):
  # Get the user's custom chat history
  custom_chat_history = redis_client.lrange(f'conversation:{user_id}', 0, -1)
  custom_chat_history = [deserialize_chat_message(json.loads(msg)) for msg in custom_chat_history]

  # Create a chat engine
  engine = index.as_query_engine(verbose=True, chat_history=custom_chat_history,)
  chat_engine = index.as_chat_engine(
      query_engine=engine,
      chat_mode='context',
      chat_history=custom_chat_history,
      verbose=True
  )

  # Generate a response
  response = chat_engine.chat(f" {query}").response
  # Add the response to the user's conversation history
  add_to_history(response, user_id,"AI")
  return response

def pre_update(user_id):
    #set up context for new convo, should only be called once
    add_to_history('Tell me about crypto and portfolio management generally', user_id, "USER")
    add_to_history("", user_id, "AI")
def chatbot(input_text, user_id):
    add_to_history(input_text, user_id, "USER")
    if not is_topic_related_query(index,input_text, user_id):
      # If the query is not related to pets, return a default response
        response = "I'm sorry, but I can only answer questions about pets."
    else:
        # If the query is related to pets, generate a response using the chat engine
        response = generate_response(index, input_text, user_id)
    
    return response

def launch_gradio():
    
    iface = gr.Interface(fn=chatbot,
                        inputs=gr.components.Textbox(lines=7, label="Enter your text"),
                        outputs="text",
                        title="Custom-trained AI Chatbot On Pets")
    io = iface.launch(share=True)
    return io

def initiated():
    if( index == None):
        initiate_index()
    return True

def initiate_index():
    global index
    new_index = construct_index("sources")
    index = new_index
    
# redis_client.flushall()
# pre_update(user_id)
#launch_gradio()
