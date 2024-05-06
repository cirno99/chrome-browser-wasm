<script setup lang="ts">
import init, { greet } from "../../wasm/pkg"

init().then(() => {
    greet("Chrome Extension")
})
</script>

<template>
    <div>
        <h1>Popup!</h1>
    </div>
</template>