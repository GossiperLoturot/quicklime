<script lang="ts">
  import { onMount } from "svelte";
  import * as core from "@tauri-apps/api/core";
  import * as event from "@tauri-apps/api/event";

  let inputText = $state("");
  let outputText = $state("");

  async function onConfirmInput(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await core.invoke("on_confirm_input", { text: outputText });
  }

  async function onChangeInput(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await core.invoke("on_change_input", { text: inputText });
  }

  function onKeyDown(event: KeyboardEvent) {
    if (event.key === "Escape") core.invoke("on_exit_input", {});
  }

  function onShowWindow(event: event.Event<void>) {
    location.reload();
  }

  function onHideWindow(event: event.Event<void>) {
    inputText = "";
    outputText = "";
  }

  function onUpdateOutput(event: event.Event<string>) {
    outputText = event.payload;
  }

  onMount(() => {
    document.addEventListener("keydown", onKeyDown);

    const unlisten0 = event.listen<void>("show_window", onShowWindow);
    const unlisten1 = event.listen<void>("hide_window", onHideWindow);
    const unlisten2 = event.listen<string>("update_output", onUpdateOutput);

    return async () => {
      document.removeEventListener("keydown", onKeyDown);
      (await unlisten0)();
      (await unlisten1)();
      (await unlisten2)();
    };
  });
</script>

<main class="container">
  <form class="row" onsubmit={onConfirmInput}>
    <input
      id="input"
      placeholder="Enter any text..."
      autocapitalize="none"
      autocomplete="off"
      autofocus
      bind:value={inputText}
      oninput={onChangeInput}
    />
    <button type="submit">Confirm</button>
  </form>
  <p>{outputText}</p>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
