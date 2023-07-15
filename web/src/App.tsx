import Game from "./Game"

const App = () => {

  return (
    <div className="App flex flex-col gap-6 my-6 items-center">
      <h1 className="text-5xl font-bold">You have aquired the Certus Quartz</h1>
      <div className="max-w-screen-xl">
        <Game />
      </div>
    </div>
  )
}

export default App
