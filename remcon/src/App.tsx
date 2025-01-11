import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open,confirm,message } from '@tauri-apps/plugin-dialog';
import "./App.css";

function App() {
  const [dir, setDir] = useState("");

  function executeCommands() {
    // invoke('simple_command')
    invoke('command_with_message', {filepath: dir}).then(async () => {
      await confirm(
        "分割まとめに成功しました",
        {
          title: ""
        }
      )
    }).catch(async (error) => {
      await message(
        error,
        {
          kind: "warning",
          title: "convert error"
        }
      )
    }
  );
  }

  async function openDialog() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: 'vpp',
          extensions: ['vpp'],
        }
      ]
    })

    if(file == null){
      //await message(
      //  "vppファイル選択に失敗しました。",
      //  {
      //    kind: "warning",
      //    title: "selection error"
      //  }
      //)
    }else{
      setDir(file)
    }
  }

  return (
    <main className="container">
      <h1>VOICEPEAK分割まとめくん</h1> 

      <div className="file-selection">
        <button onClick={openDialog}>vppファイル選択</button>
        <input
          type="text"
          value={dir}
          placeholder="Directory"
          className="text-box"
        />
      </div>
      <button onClick={executeCommands}>分割まとめ実行</button>
    </main>
  );
}

export default App;
