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
  <div class="h-screen w-screen">
    <RouterView />
    <!-- Public components -->
    <Toaster />
  </div>
</template>
