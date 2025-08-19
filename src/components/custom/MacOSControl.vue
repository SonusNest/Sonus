<template>
    <div class="flex flex-row gap-1">
        <span class="group rounded-2xl w-3.5 h-3.5 bg-[#ff5b52] flex items-center justify-center cursor-pointer"
        @click="window.close()">

            <X class="size-2.5 opacity-0 group-hover:opacity-100 transition-opacity duration-200 dark:text-black" />
        </span>
        <span class="group rounded-2xl w-3.5 h-3.5 bg-[#e6c02a] flex items-center justify-center cursor-pointer"
        @click="window.minimize()">

            <Minus class="size-2.5 opacity-0 group-hover:opacity-100 transition-opacity duration-200 dark:text-black" />

        </span>
        <span class="group rounded-2xl w-3.5 h-3.5 bg-[#53c22b] flex items-center justify-center cursor-pointer"
        @click="handleMaximize">

            <svg t="1751712608216" class=" opacity-0 group-hover:opacity-100 transition-opacity duration-200"
                viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="24305" width="200"
                height="200">
                <path
                    d="M749.056 230.4l-343.552 1.536c-24.576 0-30.208 14.336-12.8 31.232l368.128 368.128c17.408 17.408 31.232 11.264 31.232-12.8l1.024-343.552c0-24.576-19.968-44.544-44.032-44.544zM263.168 392.192c-17.408-17.408-31.232-11.264-31.232 12.8l-1.024 343.552c0 24.576 19.968 44.032 44.032 44.032l343.552-1.536c24.576 0 30.208-14.336 12.8-31.232L263.168 392.192z"
                    p-id="24306"></path>
            </svg>
        </span>
    </div>
</template>

<script setup lang="ts">
import { Minus, X } from "lucide-vue-next"
import { getCurrentWindow } from '@tauri-apps/api/window'

const window = getCurrentWindow()

const handleMaximize = async () => {
    if (await window.isMaximized()) {
        await window.unmaximize()
    } else {
        await window.maximize()
    }
}
</script>

<style scoped>

</style>