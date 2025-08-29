<template>
  <div class="flex flex-col p-2 pl-30 pr-30">
    <div class="flex flex-row items-center gap-3 text-sm">
      <span>{{ playerStore.formattedCurrentTime }}</span>
      <Progress v-model="playerStore.progressPercentage" />
      <span>{{ playerStore.formattedTotalDuration }}</span>
    </div>
    <div class="flex flex-row items-center">
      <div class="flex flex-row items-center gap-1 text-sm">
        <div class="flex flex-row items-center">
          <Button variant="ghost" class="hover:bg-transparent ">
            <Shuffle />
          </Button>
          <Button variant="ghost" class="hover:bg-transparent group" @click="prevTrack">
            <svg width="32" height="28" viewBox="0 0 32 28" xmlns="http://www.w3.org/2000/svg"
                 class="size-8" style="transform: rotateY(180deg);">
              <path
                  d="M18.14 20.68c.365 0 .672-.107 1.038-.323l8.508-4.997c.623-.365.938-.814.938-1.37 0-.564-.307-.988-.938-1.361l-8.508-4.997c-.366-.216-.68-.324-1.046-.324-.73 0-1.337.556-1.337 1.569v4.773c-.108-.399-.406-.73-.904-1.021L7.382 7.632c-.357-.216-.672-.324-1.037-.324-.73 0-1.345.556-1.345 1.569v10.235c0 1.013.614 1.569 1.345 1.569.365 0 .68-.108 1.037-.324l8.509-4.997c.49-.29.796-.631.904-1.038v4.79c0 1.013.615 1.569 1.345 1.569z"
                  fill="currentColor" fill-rule="nonzero"></path>
            </svg>
          </Button>
          <Button @click="togglePlayPause" variant="ghost"
                  class="hover:bg-transparent size-12 flex items-center justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                 fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                 stroke-linejoin="round" class="size-8">
              <path
                  ref="iconPath"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
              ></path>
            </svg>
          </Button>
          <Button variant="ghost" class="hover:bg-transparent group" @click="nextTrack">
            <svg width="32" height="28" viewBox="0 0 32 28" xmlns="http://www.w3.org/2000/svg"
                 class="size-8">
              <path
                  d="M18.14 20.68c.365 0 .672-.107 1.038-.323l8.508-4.997c.623-.365.938-.814.938-1.37 0-.564-.307-.988-.938-1.361l-8.508-4.997c-.366-.216-.68-.324-1.046-.324-.73 0-1.337.556-1.337 1.569v4.773c-.108-.399-.406-.73-.904-1.021L7.382 7.632c-.357-.216-.672-.324-1.037-.324-.73 0-1.345.556-1.345 1.569v10.235c0 1.013.614 1.569 1.345 1.569.365 0 .68-.108 1.037-.324l8.509-4.997c.49-.29.796-.631.904-1.038v4.79c0 1.013.615 1.569 1.345 1.569z"
                  fill="currentColor" fill-rule="nonzero"></path>
            </svg>
          </Button>
          <Button variant="ghost" class="hover:bg-transparent" @click="toggleRepeat">
            <Repeat v-if="repeatStatus === 0" />
            <Repeat1 v-else-if="repeatStatus === 1" />
            <Repeat2 v-else />
          </Button>
        </div>
        <Button variant="ghost">
          <Heart />
        </Button>
        <Button variant="ghost">
          <ListPlus />
        </Button>
      </div>
      <div class="flex-1"></div>
      <div class="text-sm flex flex-row items-center gap-2">
        <div class="w-[40px] h-[40px] rounded bg-center bg-cover"
             :style="{ backgroundImage: bgi() }">
        </div>
        <div class="text-ellipsis whitespace-nowrap">
          <div class="text-[16px] font-bold">{{ currentTrack?.title }}</div>
          <div class="text-xs text-foreground/60">{{ currentTrack?.artist.join(', ') }}</div>
        </div>
      </div>
      <div class="flex-1"></div>
      <div class="flex flex-row items-center gap-1 text-sm">
        <Button variant="ghost" @click="toggleMute">
          <Volume fill="currentColor" v-if="volumeStatus <= 5 && volumeStatus > 0" />
          <Volume1 fill="currentColor" v-else-if="volumeStatus <= 25 && volumeStatus > 5" />
          <Volume2 fill="currentColor" v-else-if="volumeStatus <= 100 && volumeStatus > 25" />
          <VolumeOff fill="currentColor" v-else-if="volumeStatus <= 0" />
        </Button>
        <Slider
            :model-value="volUI"
            :min="0"
            :max="100"
            :step="1"
            class="w-20 bg-input"
            @pointerdown="handleVolDragStart"
            @update:model-value="onVolChange"
            @pointerup="onVolCommit"
            @touchend.prevent.stop="onVolCommit"
            @keyup.enter="onVolCommit"
        />
        <div v-if="false">
          <Button>
            <Podcast />
          </Button>
          <Button>
            <ListStart />
          </Button>
          <Button>
            <ListEnd />
          </Button>
          <Button>
            <ListRestart />
          </Button>
        </div>
        <Button variant="ghost">
          <Shrink />
        </Button>
        <Button variant="ghost" @click="appStore.setImmersionDrawerState(true)">
          <TvMinimal />
        </Button>
        <Button variant="ghost">
          <MessageSquareQuote />
        </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, onMounted, watch, computed, onUnmounted} from 'vue'
import { Button } from '../ui/button'
import { Progress } from '@/components/ui/progress'
import {
  Heart,
  Volume,
  Volume1,
  Volume2,
  VolumeOff,
  Podcast,
  ListPlus,
  ListStart,
  ListEnd,
  ListRestart,
  Shuffle,
  Repeat,
  Repeat1,
  Repeat2,
  TvMinimal,
  Shrink,
  MessageSquareQuote
} from 'lucide-vue-next'
import { Slider } from '@/components/ui/slider'
import { gsap } from 'gsap';
import { MorphSVGPlugin } from 'gsap/MorphSVGPlugin'
import { useAppStore } from '@/stores/appStore'
import { usePlayerStore } from '@/stores/playerStore'
import type { PlaybackState } from '@/stores/playerStore'
import DefaultCover from '@/assets/default.png'
import {storeToRefs} from "pinia"
import { throttle } from 'lodash-es'

const appStore = useAppStore()
const playerStore = usePlayerStore()

const repeatStatus = ref(0)
const volumeStatus = ref(10)
const iconPath = ref<SVGPathElement | null>(null)

const { volume } = storeToRefs(playerStore)
const isDragging = ref(false)
const volUI = ref<number[]>([Math.round(volume.value * 100)])

const throttledSetVolume = throttle((targetVol: number) => {
  playerStore.setVolume(targetVol)
}, 200, { leading: true, trailing: true })

const handleVolDragStart = () => {
  isDragging.value = true
}

const onVolChange = (val: number[] | undefined) => {
  if (!val || val.length === 0) return
  const targetVol = val[0] / 100
  volUI.value = val
  throttledSetVolume(targetVol)
}

const onVolCommit = () => {
  isDragging.value = false
  throttledSetVolume.flush()
  volUI.value = [Math.round(volume.value * 100)]
}

watch(volume, (newVol) => {
  if (!isDragging.value) {
    volUI.value = [Math.round(newVol * 100)]
  }
}, { immediate: true })

onUnmounted(() => {
  throttledSetVolume.cancel()
})


gsap.registerPlugin(MorphSVGPlugin);
const playPath = "M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z";
const pausePath = "M14 3h5v18h-5zM5 3h5v18H5z";


const updatePlayPauseIcon = (currentState: PlaybackState) => {
  if (!iconPath.value) return;
  // @ts-ignore
  const targetPath = currentState === 'Playing' ? pausePath : playPath;

  gsap.to(iconPath.value, {
    duration: 0.4,
    morphSVG: {
      type: "rotational",
      map: "complexity",
      shape: targetPath
    },
    ease: "power3.inOut"
  });
}

watch(
    () => playerStore.playbackState,
    (newState) => updatePlayPauseIcon(newState),
    { immediate: true }
)

const togglePlayPause = () => {
  if (playerStore.playbackState === 'Playing' as PlaybackState) {
    playerStore.pause();
  } else {
    playerStore.playbackState === 'Stopped' as PlaybackState
        ? playerStore.play()
        : playerStore.resume();
  }
}


const currentTrack = computed(() => {
  return playerStore.currentTrack
})
const bgi = () => {
  if (!currentTrack.value?.cover_art || currentTrack.value?.cover_art[0] === 'default') {
    return `url(${DefaultCover})`;
  }
  return `url(${currentTrack.value?.cover_art.join(',')})`;
}

const toggleRepeat = () => {
  repeatStatus.value = (repeatStatus.value + 1) % 3;
}

const updateVolume = () => {
  if (playerStore.setVolume) playerStore.setVolume(volumeStatus.value);
}

const toggleMute = () => {
  const targetVol = volumeStatus.value === 0 ? 0.1 : 0
  volumeStatus.value = targetVol * 100
  throttledSetVolume(targetVol)
  volUI.value = [Math.round(targetVol * 100)]
}


const nextTrack = () => {
  playerStore.nextTrack()
}

const prevTrack = () => {
  playerStore.prevTrack()
}

onMounted(() => {
  if (iconPath.value) {
    // @ts-ignore
    const initialPath = playerStore.playbackState === 'Playing' ? pausePath : playPath;
    iconPath.value.setAttribute('d', initialPath);
  }
  updateVolume();
});
</script>

<style scoped></style>
