<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const text = ref("");
const convertedText = ref("");
const defaultInputFontSize = 1.2;
const defaultOutputFontSize = 1.2;
const inputFontSize = ref(defaultInputFontSize);
const outputFontSize = ref(defaultOutputFontSize);

/**
 * 繁 <=> 簡 轉換
 * @param config - 轉換配置 ("s2t" 或 "t2s")
 */
async function convert(config: string) {
  convertedText.value = await invoke("convert_text", { text: text.value, config: config });
}

/**
 * 增加輸入面板字體大小
 */
function increaseInputFontSize() {
  inputFontSize.value += 0.1;
}

/**
 * 減少輸入面板字體大小 - 最小限制為 0.5em
 */
function decreaseInputFontSize() {
  if (inputFontSize.value > 0.5) { inputFontSize.value -= 0.1; }
}

/**
 * 增加輸出面板字體大小
 */
function increaseOutputFontSize() {
  outputFontSize.value += 0.1;
}

/**
 * 減少輸出面板字體大小 - 最小限制為 0.5em
 */
function decreaseOutputFontSize() {
  if (outputFontSize.value > 0.5) { outputFontSize.value -= 0.1; }
}

/**
 * 回復面板預設字體大小
 */
function resetFontSize() {
  inputFontSize.value = defaultInputFontSize
  outputFontSize.value = defaultOutputFontSize
}

// MARK: - event handlers
/**
 * 處理鍵盤事件以增減字體大小 (Command / Ctrl + '+' / '=' / '-')
 * @param event - 鍵盤事件
 */
function handleKeyboardEvent(event: KeyboardEvent) {

  if (event.metaKey || event.ctrlKey) {
    
    // 停用這些快捷鍵
    if (['=', '+', '-', '0'].includes(event.key)) { event.preventDefault(); }

    switch (event.key) {
      case '=': // Typically the key for '+' without Shift
      case '+': increaseInputFontSize(); increaseOutputFontSize(); break;
      case '-': decreaseInputFontSize(); decreaseOutputFontSize(); break;
      case '0': resetFontSize(); break
      default: break;
    }
  }
}

// MARK: - 生命週期
onMounted(() => {
  window.addEventListener('keydown', handleKeyboardEvent);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyboardEvent);
});
</script>

<template>
  <main class="container">
    <div class="row main-row">
      <textarea v-model="text" placeholder="輸入文字..." :style="{ fontSize: inputFontSize + 'em'}"></textarea>
      <div class="column">
        <button @click="convert('s2t')">🇹🇼</button>
        <button @click="convert('t2s')">🇨🇳</button>
      </div>
      <textarea v-model="convertedText" placeholder="轉換結果..." readonly :style="{ fontSize: outputFontSize + 'em'}"></textarea>
    </div>
  </main>
</template>

<style>
html, body {
  background-color: #2f2f2f;
  margin: 0;
  padding: 0;
  height: 100%;
}
</style>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  padding: 1rem 1rem 2.5rem 1rem;
  box-sizing: border-box;
}

.row {
  display: flex;
  justify-content: center;
  gap: 1rem;
  width: 100%;
}

.main-row {
  flex-grow: 1;
}

.column {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 1rem;
}

textarea {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  background-color: #0f0f0f98;
  color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  resize: none;
  width: 100%;
  height: 100%;
}

button {
  background-color: transparent;
  border: none;
  padding: 0.6em 1.2em;
  font-size: 2em;
  font-weight: 500;
  font-family: inherit;
  color: #ffffff;
  cursor: pointer;
  transition: color 0.25s;
}

button:hover {
  color: #24c8db;
}
</style>