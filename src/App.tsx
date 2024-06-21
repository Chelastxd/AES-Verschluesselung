import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  const [text, setText] = createSignal("");
  const [schluessel, setSchluessel] = createSignal("");
  const [output, setOutput] = createSignal("");


  async function verschluesseln() {
    let result: string = await invoke("aes_verschluesseln", {klartext: text(), key: schluessel() });
    setOutput(result);
  }

  async function entschluesseln() {
    let result: string = await invoke("aes_entschluesseln", {geheimtext: text(), key: schluessel() });
    setOutput(result);
  }


  return (
    <div>
        <div class = "heading">
        Fabians toller AES-Verschlüsselungsapparat :)
        </div>
        <div class = "input-box">
          <textarea onInput={(e) => {
            setText(e.currentTarget.value);
            verschluesseln()
          }}></textarea>
        </div>
        <div class = "key-box">
          <input onInput={(e) => setSchluessel(e.currentTarget.value)}></input>
        </div>
        <div class = "Buttons">
          <span>
          <button class="encrButton"onClick={verschluesseln}>VERSCHLÜSSELN</button>
          <button class="decrButton"onClick={entschluesseln}>ENTSCHLUESSELN</button>
          </span>
        </div>
        <div class = "output-box">
          <textarea value={output()} readOnly={true}></textarea>
        </div>
    </div>
  );
}

export default App;
