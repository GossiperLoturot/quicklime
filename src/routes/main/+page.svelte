<script lang="ts">
  import { onMount } from "svelte";
  import * as core from "@tauri-apps/api/core";
  import * as event from "@tauri-apps/api/event";

  const MODE_TRANSLATION = 0;
  const MODE_POLISHING = 1;
  const MODE_COMPLETION = 2;

  let inputRef: HTMLInputElement | null = null;
  let inputText = $state("");
  let outputText = $state("");
  let mode = $state(0);

  function modeTagClass(index: number) {
    return index === mode ? "mode-tag mode-tag-focus" : "mode-tag";
  }

  function modeTagHandle(index: number) {
    return async function (event: Event) {
      event.preventDefault();
      // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
      await core.invoke("on_change_mode", { mode: index });
    };
  }

  async function onConfirmInput(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await core.invoke("on_confirm_input", { input: outputText });
    inputText = "";
    outputText = "";
  }

  async function onChangeInput(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await core.invoke("on_change_input", { input: inputText });
  }

  function onKeyDown(event: KeyboardEvent) {
    if (event.key === "Escape") core.invoke("on_exit_input", {});
    else if (event.ctrlKey && event.key === "1") core.invoke("on_change_mode", { mode: MODE_TRANSLATION });
    else if (event.ctrlKey && event.key === "2") core.invoke("on_change_mode", { mode: MODE_POLISHING });
    else if (event.ctrlKey && event.key === "3") core.invoke("on_change_mode", { mode: MODE_COMPLETION });
  }

  function onShowWindow(event: event.Event<void>) {
    if (inputRef) inputRef.focus();
  }

  function onHideWindow(event: event.Event<void>) {
  }

  function onUpdateOutput(event: event.Event<string>) {
    outputText = event.payload;
  }

  function onUpdateMode(event: event.Event<number>) {
    mode = event.payload;
    core.invoke("on_change_input", { input: inputText });
  }

  onMount(() => {
    document.addEventListener("keydown", onKeyDown);

    const unlisten0 = event.listen<void>("show_window", onShowWindow);
    const unlisten1 = event.listen<void>("hide_window", onHideWindow);
    const unlisten2 = event.listen<string>("update_output", onUpdateOutput);
    const unlisten3 = event.listen<number>("update_mode", onUpdateMode);

    return async () => {
      document.removeEventListener("keydown", onKeyDown);
      (await unlisten0)();
      (await unlisten1)();
      (await unlisten2)();
      (await unlisten3)();
    };
  });
</script>

<main class="container">
  <div class="output">
    <div class="output-text">
      {outputText}
    </div>
  </div>
  <hr />
  <form class="input" onsubmit={onConfirmInput}>
    <input
      class="input-input"
      placeholder="Input any sentence..."
      autocapitalize="none"
      autocomplete="off"
      bind:this={inputRef}
      bind:value={inputText}
      oninput={onChangeInput}
    />
  </form>
  <hr />
  <div class="mode">
    <button class={modeTagClass(MODE_TRANSLATION)} onclick={modeTagHandle(MODE_TRANSLATION)}>
      translation
    </button>
    <button class={modeTagClass(MODE_POLISHING)} onclick={modeTagHandle(MODE_POLISHING)}>
      polishing
    </button>
    <button class={modeTagClass(MODE_COMPLETION)} onclick={modeTagHandle(MODE_COMPLETION)}>
      completion
    </button>
  </div>
</main>

<style>
* {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
}

:root {
  color: #0f0f0f;
  background-color: #f6f6f6;
  background: transparent;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;

  border: 1px solid transparent;
  border-radius: 8px;
  background: linear-gradient(#f6f6f6, #f6f6f6) padding-box,
              linear-gradient(to right, orchid, cyan) border-box;
}

.output, .input {
  display: flex;
}

.output-text, .input-input {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 8px 16px;
  color: #0f0f0f;
  background-color: #f6f6f6;
  outline: none;

  width: 100%;
  word-break: break-all;
}

.output-text {
  height: 48px;
}

input::placeholder {
  color: #0f0f0f50;
}

.mode {
  padding: 8px 16px;
  display: flex;
  justify-content: center;
}

.mode-tag {
  padding: 4px 8px;
  margin: 0 4px;
  outline: none;
  border-radius: 8px;
  border: 1px solid transparent;
  
  color: #0f0f0f50;
  background-color: #f6f6f6;
}

.mode-tag-focus {
  color: #f6f6f6;
  background-color: #0f0f0f;
}

.mode-tag:hover:not(.mode-tag-focus) {
  background-color: #0f0f0f10;
}

hr {
  width: calc(100% - 16px);
  border: none;
  border-top: 1px solid #0f0f0f50;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
    background: transparent;
  }

  .container {
    background: linear-gradient(#0f0f0f, #0f0f0f) padding-box,
                linear-gradient(to right, darkviolet, darkcyan) border-box;
  }

  .output-text,
  .input-input {
    color: #f6f6f6;
    background-color: #0f0f0f;
  }

  input::placeholder {
    color: #f6f6f650;
  }

  .mode-tag {
    color: #f6f6f650;
    background-color: #0f0f0f;
  }

  .mode-tag-focus {
    color: #0f0f0f;
    background-color: #f6f6f6;
  }

  .mode-tag:hover:not(.mode-tag-focus) {
    background-color: #f6f6f610;
  }

  hr {
    border-top: 1px solid #f6f6f650;
  }
}
</style>
