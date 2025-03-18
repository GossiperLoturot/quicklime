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
  let modeIndex = $state(MODE_TRANSLATION);

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
    await core.invoke("on_change_input", { input: inputText, mode: modeIndex });
  }

  function modeTagClass(index: number) {
    return index === modeIndex ? "mode-tag mode-tag-focus" : "mode-tag";
  }

  function modeTagHandle(index: number) {
    return async function (event: Event) {
      event.preventDefault();
      // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
      modeIndex = index;
      await core.invoke("on_change_input", { input: inputText, mode: modeIndex });
    };
  }

  function onKeyDown(event: KeyboardEvent) {
    // Exit input mode
    if (event.key === "Escape") {
      core.invoke("on_exit_input", {});

    // Switch to translation mode
    } else if (event.ctrlKey && event.key === "1") {
      modeIndex = MODE_TRANSLATION;
      core.invoke("on_change_input", { input: inputText, mode: modeIndex });

    // Switch to polishing mode
    } else if (event.ctrlKey && event.key === "2") {
      modeIndex = MODE_POLISHING
      core.invoke("on_change_input", { input: inputText, mode: modeIndex });

    // Switch to completion mode
    } else if (event.ctrlKey && event.key === "3") {
      modeIndex = MODE_COMPLETION;
      core.invoke("on_change_input", { input: inputText, mode: modeIndex });
    }
  }

  function onShowWindow(event: event.Event<void>) {
    if (inputRef) {
      inputRef.focus();
    }
  }

  function onHideWindow(event: event.Event<void>) {
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
  background-color: #ffffff;
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

  border-radius: 8px;
  border: 1px solid #396cd8;
  background-color: #ffffff;
}

.output, .input {
  display: flex;
}

.output-text,
.input-input {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 8px 16px;
  color: #0f0f0f;
  background-color: #ffffff;
  outline: none;

  width: 100%;
  word-break: break-all;
}

.output-text {
  height: 48px;
}

input::placeholder {
  color: #0f0f0f80;
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
  cursor: pointer;
  border-radius: 8px;
  border: 1px solid transparent;
  
  color: #0f0f0f80;
  background-color: #ffffff;
}

.mode-tag-focus {
  color: #0f0f0f;
  background-color: #f0f0f0;
}

hr {
  width: calc(100% - 16px);
  border: none;
  border-top: 1px solid #0f0f0f50;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f0f0f0;
    background-color: #0f0f0f;
    background: transparent;
  }

  .container {
    background-color: #0f0f0f;
  }

  .output-text,
  .input-input {
    color: #f0f0f0;
    background-color: #0f0f0f;
  }

  input::placeholder {
    color: #f0f0f080;
  }

  .mode-tag {
    color: #f0f0f080;
    background-color: #0f0f0f;
  }

  .mode-tag-focus {
    color: #f0f0f0;
    background-color: #1f1f1f;
  }

  hr {
    border-top: 1px solid #f0f0f050;
  }
}
</style>
