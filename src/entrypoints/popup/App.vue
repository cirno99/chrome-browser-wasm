<script setup lang="ts">
import { onMounted } from 'vue';

onMounted(() => {
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        if (tabs[0].id === undefined) return;
        chrome.tabs.sendMessage<{}, { innerText: string}>(tabs[0].id, {}, (response) => {
            console.log(response.innerText);
        });
    });
})
</script>

<template>
    <div>
        <h1>Popup!</h1>
    </div>
</template>