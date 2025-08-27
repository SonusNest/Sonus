<script setup lang="ts">
import { computed } from 'vue'
import {LyricPlayer} from "@applemusic-like-lyrics/vue"
import { parseTTML} from "@applemusic-like-lyrics/lyric";
import {Track, usePlayerStore} from "@/stores/playerStore"

const playerStore = usePlayerStore()

const currentTrack: Track = computed(() => {playerStore.currentTrack})
const lyricLines = computed(() => {
  console.log(currentTrack)
  if (currentTrack?.lyrics) {
    return parseTTML(currentTrack.value?.lines)
  }
  return []
})
</script>

<template>
  <div>
    <LyricPlayer class="h-[120%] w-full font-bold lp p-1" :lyric-lines="lyricLines" :current-time="playerStore.currentPosition"
                 :hide-passed-lines="true" :enable-blur="true" ref="playerRef"
                 @line-click="e => { if (playerStore) playerStore.currentPosition = (e.line.getLine().startTime / 1000) }" />
  </div>
</template>

<style scoped>

</style>