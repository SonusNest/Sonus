<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView } from 'vue-router'
import { Toaster } from './components/ui/sonner'
import { useColorMode } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '@/stores/appStore'
import { usePlayerStore } from '@/stores/playerStore'
import router from './router'

import 'vue-sonner/style.css'

const mode = useColorMode()
const appStore = useAppStore()

interface MaterialInfo {
  id: number;
  name: string;
}

const setWindowMaterial = async () => {
  await invoke('get_current_window_materials')
      .then((value: unknown) => {
        const currentMaterials = (value as MaterialInfo[]).map((material) => ({
          id: material.id,
          name: material.name
        }));

        if (currentMaterials[0].id === 0 || currentMaterials[0].id === 1) {
          console.log('Current window material:', currentMaterials[0].name)
          appStore.setGlobalBackdropState(false)
        } else {
          console.log('Current window material:', currentMaterials[0].name)
          appStore.setGlobalBackdropState(true)
        }
      })
      .catch((error) => {
        console.error('Get current window materials failed:', error);
      });
}

onMounted(async() => {
  const playerStore = usePlayerStore()
  await playerStore.init()
  console.log(' playerStore initialized')

  mode.value = 'auto'
  await router.push('/home')
  await setWindowMaterial()

  // 等待3s
  await new Promise(resolve => setTimeout(resolve, 3000))
  console.log(appStore.getGlobalBackdropState)
})

router.afterEach(async () => {
  await setWindowMaterial()
})

</script>

<template>
  <div class="h-screen w-screen relative transition-colors duration-300 ease-in-out"
       :class=" { 'bg-transparent' : appStore.getGlobalBackdropState, 'bg-background' : !appStore.getGlobalBackdropState }">
    <RouterView />
    <!-- Public components -->
    <Toaster />
    <!-- Internal Test Version Watermark -->
    <div class="absolute bottom-[100px] right-[100px] text-foreground/50">
      <p class="text-2xl font-bold">Internal Test Version</p>
      <p class="text-sm text-right">v0.1.3-alpha</p>
    </div>
  </div>
</template>
