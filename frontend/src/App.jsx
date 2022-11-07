import "./App.css";
import Chessboard from "chessboardjsx";
import { Chess } from "chess.js";
import { useEffect, useState } from "react";
import axios from 'axios';

const chess = new Chess();

function App() {
  const [fen, setFen] = useState("start");
  useEffect(() => {
    const logData = async () => {
      const data = await axios.get("http://localhost:8000/");
      console.log(data.data);
    }
    logData();

  }, [])
  const handleDrop = (square) => {
    chess.move({ from: square.sourceSquare, to: square.targetSquare });
    if (chess.fen() === fen) {
      alert("invalid move");
    }
    else {
      setFen(chess.fen());
    }
  };
  return (
    <div className="App">
      <div className="board">
        <Chessboard width={400} position={fen} onDrop={handleDrop} />
      </div>
      <button
        onClick={() => {
          chess.undo();
          setFen(chess.fen());
        }}
      >
        Click me!
      </button>
    </div>
  );
}

export default App;
