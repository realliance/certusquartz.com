import { useEffect, useState } from "react";

const Game = () => {
  const [gameLoaded, setGameLoaded] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const run = async () => {
      setLoading(true);
      const { main } = await import("./game");
      setGameLoaded(true);
      setLoading(false);
      main();
    };

    if (!gameLoaded && !loading) {
      run();
    }
  }, [gameLoaded]);

  const statusText = loading ? <h1 className="text-4xl text-center my-4">Loading Game...</h1> : null;

  return (
    <>
      {statusText}
      <canvas className="bevy-instance__canvas" id="bevy" onContextMenu={(e) => e.preventDefault()}></canvas>
    </>
  );
};

export default Game;

