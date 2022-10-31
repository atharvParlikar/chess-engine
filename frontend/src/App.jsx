import "./App.css";
import Chessboard from "chessboardjsx";
import { Chess } from "chess.js";
import { useState } from "react";

const chess = new Chess();

function App() {
  const [fen, setFen] = useState("start");
  const handleDrop = (square) => {
    chess.move({ from: square.sourceSquare, to: square.targetSquare });
    if (chess.fen() === fen) {
      alert("invalid move");
    }
    setFen(chess.fen());
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
        Undo
      </button>
    </div>
  );
}

export default App;
