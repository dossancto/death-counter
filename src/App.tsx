import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import { DisplayInfo } from "./Model/DisplayInfo"
import { SelectDisplay } from './Components/selectDisplay/selectDisplay'

function App() {
  const [displaysInfos, setDisplaysInfos] = useState<DisplayInfo[]>([]);

  async function greet() {
    const displays: Array<string> = await invoke("get_all_screens_info");
    const parsedDisplays: DisplayInfo[] = displays.map(display => JSON.parse(display));
    setDisplaysInfos(parsedDisplays)
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <div>
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>
      </div>
      <SelectDisplay display={displaysInfos} />

    </div>
  );
}

export default App;
