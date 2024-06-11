import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  const [input, setInput] = createSignal("");

  async function buttonClick() {
    let result: number = await invoke("add", { links: 1, rechts: 2 });
    console.log(result);
  }


  return (
    <div>
        <div class = "Heading">
        Fabians toller Verschlüsselungsapparat :)
        </div>
        <div class = "Input-Box">
          <input ></input>
        </div>
        <div class = "Key-Box">
          <input></input>
        </div>
        <div class = "Button">
          <button onClick={buttonClick}>VERSCHLÜSSELN</button>
        </div>
        <div class = "Output-Box">
          <input disabled={true}></input>
        </div>
    </div>
  );
}

export default App;
