<script lang="ts">
  import { onMount } from "svelte";
  import * as core from "@tauri-apps/api/core";

  const LLM_CHATGPT = 0;
  const LLM_GROK = 1;

  interface Config {
    llm: number;
    token: string;
    rate: number;
    language: string;
    cache_size: number;
  }

  // default config for view
  let config = $state<Config>({
    llm: LLM_CHATGPT,
    token: "",
    rate: 0.5,
    language: "English",
    cache_size: 1024,
  });

  async function onChangeConfig(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    await core.invoke("on_change_config", { config });
  }

  onMount(() => {
    core
      .invoke<Config>("on_get_config", {})
      .then((new_config) => (config = new_config));
  });
</script>

<main class="container">
  <h1>Configuration</h1>

  <hr />

  <div>
    <label for="item-llm">LLM Provider</label>
    <select
      id="item-llm"
      class="item-select"
      bind:value={config.llm}
      onchange={onChangeConfig}
    >
      <option value={LLM_CHATGPT}>Chat GPT</option>
      <option value={LLM_GROK}>Grok</option>
    </select>
  </div>

  <div>
    <label for="item-token">API Token</label>
    <input
      type="text"
      id="item-token"
      class="item-input"
      bind:value={config.token}
      onchange={onChangeConfig}
    />
  </div>

  <div>
    <label for="item-rate">Request Rate [s]</label>
    <input
      type="number"
      id="item-rate"
      class="item-input"
      min={0.1}
      bind:value={config.rate}
      onchange={onChangeConfig}
    />
  </div>

  <div>
    <label for="item-language">Language</label>
    <input
      type="text"
      id="item-language"
      class="item-input"
      bind:value={config.language}
      onchange={onChangeConfig}
    />
  </div>

  <div>
    <label for="item-cachesize">Cache Size</label>
    <input
      type="number"
      id="item-cachesize"
      class="item-input"
      min={1}
      bind:value={config.cache_size}
      onchange={onChangeConfig}
    />
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

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  h1 {
    font-size: 32px;
  }

  label {
    display: block;
    padding: 8px 0;
    color: #0f0f0f80;
  }

  .container {
    margin: 0;
    display: flex;
    padding: 16px;
    flex-direction: column;
    justify-content: center;
  }

  .item-select {
    border: 1px solid transparent;
    padding: 8px 12px;
    color: #0f0f0f;
    background-color: #f0f0f0;
    cursor: pointer;
    outline: none;
    margin-bottom: 1em;
  }

  .item-input {
    border: 1px solid transparent;
    padding: 8px 12px;
    color: #0f0f0f;
    background-color: #f0f0f0;
    outline: none;
    margin-bottom: 1em;
  }

  .item-select:hover,
  .item-input:hover {
    border-color: #396cd8;
  }

  .item-input::-webkit-inner-spin-button,
  .item-input::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
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
    }

    label {
      color: #f0f0f080;
    }

    .item-select,
    .item-input {
      color: #f0f0f0;
      background-color: #1f1f1f;
    }

    hr {
      border-top: 1px solid #f0f0f050;
    }
  }
</style>
