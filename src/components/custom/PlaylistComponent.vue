<script setup lang="ts">
import {computed, ref} from 'vue'
import { usePlayerStore, Track } from '@/stores/playerStore'
import DefaultCover from '@/assets/default.png'
import {ListX} from 'lucide-vue-next'
import {Button} from "@/components/ui/button";

const playerStore = usePlayerStore()

const tracks = computed(() => playerStore.currentPlaylist.tracks)
console.log(playerStore)
console.log(tracks)
const bgi = (background: string[]) => {
  console.log(background)
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
  <div class="h-full w-full flex flex-col">
    <div class="flex w-full flex-row justify-between items-center pt-2 pb-2">
      <p class="text-lg font-bold">Will be played soon</p>
      <Button variant="ghost" size="icon">
        <ListX />
      </Button>
    </div>
      <ul class="w-full h-full overflow-y-auto">
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
            <p class="text-sm text-foreground-500">{{formatDuration(track.duration)}}</p>
          </div>
        </li>
      </ul>
  </div>
</template>

<style scoped>

</style>