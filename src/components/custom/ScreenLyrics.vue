<template>
    <div class="absolute h-full w-full flex flex-row items-center justify-center z-10">
        <div class="w-1/2 h-full flex flex-col items-center justify-center">
            <div>
                <div class="w-128 h-128 rounded-2xl shadow-2xl" :style="{
                    backgroundImage: `url(${props.imageUrl})`,
                    backgroundSize: 'cover'
                }"></div>
            </div>
            <div>
                <MiniPlayerControl />
                <!-- Test audio -->
                <audio ref="audioRef" :src="testAudio" preload="auto" @loadedmetadata="handleLoadedMetadata"
                    @timeupdate="handleTimeUpdate"></audio>
            </div>
        </div>
        <div class="w-1/2 h-full">
            <LyricPlayer class="h-[120%] w-full font-bold lp p-1" :lyric-lines="result.lines" :current-time="lyricsTime"
                :hide-passed-lines="true" :enable-blur="true" ref="playerRef"
                @line-click="e => { if (audioRef) audioRef.currentTime = (e.line.getLine().startTime / 1000) }" />
            <!-- <LyricsView :lyrics="result.lines" :current-time="lyricsTime" /> -->

        </div>
        <div class="absolute top-0 w-full h-8 flex flex-row items-center justify-between">
            <div>
                <Button variant="outline" class="rounded-full">
                    <ChevronsDown />
                </Button>
            </div>
            <div class="flex-1 w-full h-full" data-tauri-drag-region></div>
            <div>
                <Button variant="outline" class="rounded-full">
                    <ChevronsDown />
                </Button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { Button } from '@/components/ui/button'
import MiniPlayerControl from './MiniPlayerControl.vue'
import { LyricPlayer } from "@applemusic-like-lyrics/vue"
import { parseTTML } from '@applemusic-like-lyrics/lyric'
import testAudio from '@/assets/test.flac'
import { ChevronsDown } from 'lucide-vue-next'
import { usePlayerStore } from '@/stores/playerStore'
// import LyricsView from '@/components/eui/LyricsView.vue'


const playerStore = usePlayerStore()

const audioRef = ref<HTMLAudioElement>()

const handleLoadedMetadata = () => {
    playerStore.setDuration(audioRef.value?.duration as number)
    playerStore.setCurrentTime(audioRef.value?.currentTime as number)
}
const handleTimeUpdate = () => {
    playerStore.setCurrentTime(audioRef.value?.currentTime as number)
}

const lyricsTime = ref(0)
watch(() => playerStore.currentTime, (newValue) => {
    lyricsTime.value = Math.round(newValue * 1000)
})

onMounted(() => {

})
watch(() => playerStore.isPlaying, () => {
    if (audioRef.value) {
        audioRef.value.paused ? audioRef.value.play() : audioRef.value.pause()
        playerStore.setIsPlaying(!audioRef.value.paused)
    }
})

const props = defineProps<{
    imageUrl: string;
    lyrics?: string;
}>();
// 从prop.lyrics中解析歌词
const lyric = props.lyrics
const result = parseTTML(lyric as string)
</script>

<style scoped>
:deep(.lp div) {
    padding-left: 2rem !important;
}
</style>