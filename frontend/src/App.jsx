import "./App.css";
import Chessboard from "chessboardjsx";
import { Chess } from "chess.js";
import { useEffect, useState } from "react";
import axios from 'axios';
import { ReactNotifications, Store } from 'react-notifications-component'
import 'react-notifications-component/dist/theme.css'

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
      Store.addNotification({
        title: "Invalid move",
        message: `move: ${square.sourceSquare}${square.targetSquare} is invalid...`,
        type: "danger",
        insert: "top",
        container: "top-right",
        animationIn: ["animate__animated", "animate__fadeIn"],
        animationOut: ["animate__animated", "animate__fadeOut"],
        dismiss: {
          duration: 2000,
          onScreen: true
        }
      });
      return "snapback";
    }
    else {
      setFen(chess.fen());
    }
  };
  return (
    <div className="App">
      <ReactNotifications />
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
