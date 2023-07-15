import { useEffect, useState } from "react";
import { Button } from "./components/Button";

const Game = () => {
  const [loading, setLoading] = useState(false);
  const [startGame, setStartGame] = useState(false);

  useEffect(() => {
    const run = async () => {
      console.log("Loading bevy");
      setLoading(true);
      const { main } = await import("./game");
      setLoading(false);
      main();
    };

    if (startGame) {
      run();
    }
  }, [startGame]);

  const joinForm = !startGame ? (
    <div className="flex flex-col gap-3">
      <Button disabled={false} text="Join Server" onClick={() => setStartGame(true)} />
    </div>
  ) : null;

  const statusText = loading ? <h1 className="text-4xl text-center my-4">Loading Game...</h1> : null;

  return (
    <>
      {joinForm}
      {statusText}
      <canvas className="bevy-instance__canvas" id="bevy" onContextMenu={(e) => e.preventDefault()}></canvas>
    </>
  );
};

export default Game;

