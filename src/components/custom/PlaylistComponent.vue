<script setup lang="ts">
import {computed} from 'vue'
import { usePlayerStore } from '@/stores/playerStore'
import {useAppStore} from "@/stores/appStore"
import DefaultCover from '@/assets/default.png'
import {ListX} from 'lucide-vue-next'
import {Button} from "@/components/ui/button";

const playerStore = usePlayerStore()
const appStore = useAppStore()

const tracks = computed(() => playerStore.currentPlaylist.tracks)
const bgi = (background: string[]) => {
  if (background[0] === 'default') {
    return 'url('+DefaultCover+')'
  }
  return 'url('+background.join(',')+')'
}
const formatDuration = (duration: number) => {
  const minutes = Math.floor(duration / 60)
  const seconds = Math.floor(duration % 60)
  return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`
}
</script>

<template>
  <div v-if="appStore.rightArea === 'playlist'" class="h-full w-full flex flex-col mr-1">
    <div class="flex w-full flex-row justify-between items-center pt-2 pb-2">
      <p class="text-lg font-bold">Will be played soon</p>
      <Button variant="ghost" size="icon">
        <ListX />
      </Button>
    </div>
      <ul class="w-full h-full overflow-y-auto mr-1">
        <li class="flex w-full flex-row gap-1.5 items-center border-b border-gray/60 pt-2 pb-2" v-for="track in tracks" :key="track.id">
          <div class="w-[40px] h-[40px] rounded bg-center bg-cover"
          :style="{backgroundImage: bgi(track.cover_art)}">
          </div>
          <div>
            <p class="text-sm font-bold ">{{track.title}}</p>
            <p class="text-xs text-foreground/60">{{track.artist[0]}}</p>
          </div>
          <div class="flex-1"></div>
          <div class="mr-1">
            <p class="text-sm text-foreground-500">{{formatDuration(track.duration??0)}}</p>
          </div>
        </li>
      </ul>
  </div>
</template>

<style scoped>

</style>