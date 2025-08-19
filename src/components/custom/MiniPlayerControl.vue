<template>
    <div
        class="w-112 h-42 flex flex-col items-center justify-center mt-5 p-5">
        <div>
            <p class="text-white text-center">
                {{ props.title }}
            </p>
            <p class="text-white/70 text-sm text-center">

                {{ props.artist }}
            </p>
        </div>
        <div class="w-full flex flex-row items-center gap-3 text-sm text-white/70 mt-5 mb-2">
            <span>{{ formatTime(playerStore.currentTime) }}</span>
            <Progress v-model="progress" class="w-full" />
            <span>{{ formatTime(playerStore.duration) }}</span>
        </div>
        <div class="flex flex-row items-center mb-2">
            <Button variant="ghost" class="hover:bg-transparent ">
                <Shuffle />
            </Button>
            <Button variant="ghost" class="hover:bg-transparent group">
                <Rewind fill="currentColor" class="size-6  transition-all " />
            </Button>
            <Button @click="togglePlayPause" variant="ghost"
                class="hover:bg-transparent size-12 flex items-center justify-center">
                <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor"
                    stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                    class="size-8">
                    <path ref="iconPath" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round"></path>
                </svg>
            </Button>
            <Button variant="ghost" class="hover:bg-transparent group">
                <FastForward fill="currentColor"
                    class="size-6 transition-all" />
            </Button>
            <Button @click="repeatStatus === 1" variant="ghost" class="hover:bg-transparent">
                <Repeat v-if="repeatStatus === 0" />

                <Repeat1 v-else-if="repeatStatus === 1" />

                <Repeat2 v-else />
            </Button>
        </div>
        <div></div>
    </div>
</template>

<script setup lang="ts">
    const props = defineProps({
        title: {
            type: String,
            default: '必杀技'
        },
        artist: {
            type: String,
            default: '古巨基'
        },
    })

import { ref, onMounted, watch } from 'vue'

import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import { gsap } from 'gsap';
import { MorphSVGPlugin } from 'gsap/MorphSVGPlugin'
import { Shuffle, Rewind, FastForward, Repeat, Repeat1, Repeat2 } from 'lucide-vue-next'
import { usePlayerStore } from '@/stores/playerStore';

const formatTime = (seconds: number): string => {
    // 处理NaN或无效值的情况
    if (isNaN(seconds) || seconds < 0) {
        return "00:00";
    }

    // 计算分钟和剩余秒数
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);

    // 补零操作，确保始终显示两位数
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
};

const playerStore = usePlayerStore()

const progress = ref(0)
const repeatStatus = ref(0)
gsap.registerPlugin(MorphSVGPlugin);

const playPath = "M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z";
const pausePath = "M14 3h5v18h-5zM5 3h5v18H5z";

const playStatus = ref(false); 
const iconPath = ref<SVGPathElement | null>(null);
const togglePlayPause = () => {
    playStatus.value = !playStatus.value;

    gsap.to(iconPath.value, {
        duration: 0.4,
        morphSVG: {
            type: "rotational",
            map: "complexity",
            shape: playStatus.value ? pausePath : playPath
        },
        ease: "power3.inOut"
    });

    playerStore.setIsPlaying(!playerStore.isPlaying)
};
watch(() => playerStore.currentTime, (newValue) => {
    progress.value = (newValue / playerStore.duration) * 100
})
onMounted(() => {
    if (iconPath.value) {
        iconPath.value.setAttribute('d', playPath);
    }
});
</script>

<style scoped>

</style>