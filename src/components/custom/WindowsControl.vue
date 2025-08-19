<template>
    <div class="flex flex-row justify-end">
        <Button variant="ghost" size="icon"
            class="h-12 w-12 border-0 rounded-none shadow-none hover:bg-[#e0e0e0] dark:text-white dark:hover:bg-[#8b8b8b44]"
            @click="window.minimize()">
            <Minus class="size-[19px]" />
        </Button>
        <Button variant="ghost" size="icon"
            class="h-12 w-12 border-0 rounded-none shadow-none hover:bg-[#e0e0e0] dark:text-white dark:hover:bg-[#8b8b8b44]"
            @click="handleMaximize">
            <Square class="size-[14px]" v-if="!isMaximized" />
            <div v-else>
                <svg t="1754161475123" class="size-[19px]" viewBox="0 0 1024 1024" version="1.1"
                    xmlns="http://www.w3.org/2000/svg" p-id="12583" width="200" height="200" fill="currentColor">
                    <path
                        d="M736 241.066667c12.970667 0 24.021333 4.565333 33.194667 13.738666 9.173333 9.173333 13.738667 20.224 13.738666 33.194667v332.501333c0 12.928-4.565333 24.021333-13.738666 33.152a45.226667 45.226667 0 0 1-33.194667 13.781334H686.933333v68.565333c0 12.970667-4.565333 24.021333-13.738666 33.194667a45.226667 45.226667 0 0 1-33.194667 13.738666H288a45.226667 45.226667 0 0 1-33.194667-13.738666 45.226667 45.226667 0 0 1-13.738666-33.194667V384c0-12.970667 4.565333-24.021333 13.738666-33.194667A45.226667 45.226667 0 0 1 288 337.066667h68.096V288c0-12.970667 4.608-24.021333 13.738667-33.194667a45.226667 45.226667 0 0 1 33.194666-13.738666z m-110.933333 157.866666H302.933333v322.133334h322.133334V398.933333z m95.957333-96H417.962667v34.133334H640c12.970667 0 24.021333 4.565333 33.194667 13.738666 9.173333 9.173333 13.738667 20.224 13.738666 33.194667v221.568h32.896L721.066667 302.933333z"
                        p-id="12584"></path>
                </svg>
            </div>
        </Button>
        <Button variant="ghost" size="icon"
            class="h-12 w-12 border-0 rounded-none shadow-none hover:bg-[#f53131] hover:text-white dark:text-white dark:hover:bg-[#f53131]"
            @click="window.close()">
            <X class="size-[18px]" />
        </Button>
    </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { Button } from '@/components/ui/button'
import { Minus, Square, X } from "lucide-vue-next"
import { getCurrentWindow } from '@tauri-apps/api/window'

const window = getCurrentWindow()
const isMaximized = ref(false)

const windowMaximizeListener = await window.listen<string>('tauri://resize', async () => {
    let maxOrUnmax = await window.isMaximized()
    isMaximized.value = maxOrUnmax
})

onUnmounted(() => {
    windowMaximizeListener()
})

const handleMaximize = async () => {
    if (await window.isMaximized()) {
        await window.unmaximize()
    } else {
        await window.maximize()
    }
}

</script>