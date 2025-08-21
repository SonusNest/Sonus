<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView } from 'vue-router'
import { Toaster } from './components/ui/sonner'
import { useColorMode } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from '@/stores/appStore'
import router from './router'

import 'vue-sonner/style.css'


const mode = useColorMode()
const appStore = useAppStore()

router.push('/home')

interface MaterialInfo {
  id: number;
  name: string;
}

onMounted(() => {
  mode.value = 'auto'

  invoke('get_current_window_materials')
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
})

</script>

<template>
  <div class="h-screen w-screen relative">
    <RouterView />
    <!-- Public components -->
    <Toaster />
    <!-- Internal Test Version Watermark -->
    <div class="absolute bottom-[100px] right-[100px] text-white/50">
      <p class="text-2xl font-bold">Internal Test Version</p>
      <p class="text-sm text-right">v0.1.0-alpha-sha.gd90d9f8</p>
    </div>
  </div>
</template>
