import { useEffect, useState } from "react";

let gameLoaded = false;

const Game = () => {
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const run = async () => {
      setLoading(true);
      const { main } = await import("./game");
      setLoading(false);
      main();
    };

    if (!gameLoaded) {
      gameLoaded = true;
      run();
    }
  }, []);

  const statusText = loading ? <h1 className="text-4xl text-center my-4">Loading your Prize...</h1> : null;

  return (
    <> 
      {statusText}
      <canvas className="bevy-instance__canvas overflow-hidden" id="bevy" onContextMenu={(e) => e.preventDefault()}></canvas>
    </>
  );
};

export default Game;

