<script setup lang="ts">
import { onMounted, ref } from 'vue';
import init, { find } from "../../wasm/pkg"

const targetContent = ref<string>("");
const query = ref<string>("");
const searchResult = ref<string[]>([]);

const handleClick = async () => {
    await init();
    const result =  find(targetContent.value, query.value);
    searchResult.value = result;
}

onMounted(() => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        if (tabs[0].id === undefined) return;
        chrome.tabs.sendMessage<{}, { innerText: string}>(tabs[0].id, {}, (response) => {
            targetContent.value = response.innerText;
        });
    });
})
</script>

<template>
    <div>
        <h1>Popup!</h1>

        <input v-model="query" type="text" placeholder="Search" />
        <button @click="handleClick">CLICK</button>

        <ul>
            <li v-for="result in searchResult" :key="result">{{ result }}</li>
        </ul>
    </div>
</template>