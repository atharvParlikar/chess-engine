from flask import Flask
from stockfish import Stockfish
import json

app = Flask(__name__)

stockfish = Stockfish(path="/home/atharv/stuff/stockfish_15_linux_x64/stockfish_15_x64")

print(stockfish)

@app.route("/")
def hello_world():
    return json.dumps({"message": "hello there"})

