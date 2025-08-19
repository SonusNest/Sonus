<template>
    <div class="flex flex-col p-2 pl-30 pr-30">
        <div class="flex flex-row items-center gap-3 text-sm">
            <span>02:13</span>
            <Progress v-model="progress" />
            <span>04:36</span>
        </div>
        <div class="flex flex-row items-center">
            <div class="flex flex-row items-center gap-1 text-sm">
                <div class="flex flex-row items-center">
                    <Button variant="ghost" class="hover:bg-transparent ">
                        <Shuffle />
                    </Button>
                    <Button variant="ghost" class="hover:bg-transparent group">
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
                            <path ref="iconPath" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round"></path>
                        </svg>
                    </Button>
                    <Button variant="ghost" class="hover:bg-transparent group">
                        <svg width="32" height="28" viewBox="0 0 32 28" xmlns="http://www.w3.org/2000/svg"
                            class="size-8">
                            <path
                                d="M18.14 20.68c.365 0 .672-.107 1.038-.323l8.508-4.997c.623-.365.938-.814.938-1.37 0-.564-.307-.988-.938-1.361l-8.508-4.997c-.366-.216-.68-.324-1.046-.324-.73 0-1.337.556-1.337 1.569v4.773c-.108-.399-.406-.73-.904-1.021L7.382 7.632c-.357-.216-.672-.324-1.037-.324-.73 0-1.345.556-1.345 1.569v10.235c0 1.013.614 1.569 1.345 1.569.365 0 .68-.108 1.037-.324l8.509-4.997c.49-.29.796-.631.904-1.038v4.79c0 1.013.615 1.569 1.345 1.569z"
                                fill="currentColor" fill-rule="nonzero"></path>
                        </svg>
                    </Button>
                    <Button @click="repeatStatus === 1" variant="ghost" class="hover:bg-transparent">
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
            <div class="text-sm">
                There are no songs playing
            </div>
            <div class="flex-1"></div>
            <div class="flex flex-row items-center gap-1 text-sm">
                <Button variant="ghost">
                    <Volume fill="currentColor" v-if="volumeStatus <= 5" />

                    <Volume1 fill="currentColor" v-else-if="volumeStatus <= 25" />

                    <Volume2 fill="currentColor" v-else-if="volumeStatus <= 100" />

                    <VolumeOff fill="currentColor" v-else-if="volumeStatus <= 0" />
                </Button>
                <Slider :default-value="[volumeStatus]" :max="100" :step="1" class="w-20 bg-input" />
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
import { ref, onMounted } from 'vue'
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

const appStore = useAppStore()

const progress = ref(13)

const repeatStatus = ref(0)
const volumeStatus = ref(10)

// 注册GSAP插件
gsap.registerPlugin(MorphSVGPlugin);

// 定义播放和暂停状态的SVG路径（从你提供的SVG中提取）
const playPath = "M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z";
const pausePath = "M14 3h5v18h-5zM5 3h5v18H5z"; // 合并两个rect为路径格式

// 状态和引用
const playStatus = ref(false); // false: 播放状态, true: 暂停状态
const iconPath = ref<SVGPathElement | null>(null); 

// 切换播放/暂停状态并执行动画
const togglePlayPause = () => {
    // 更新状态
    playStatus.value = !playStatus.value;

    // 执行SVG变形动画
    gsap.to(iconPath.value, {
        duration: 0.4,
        morphSVG: {
            type: "rotational",
            map: "complexity",
            shape: playStatus.value ? pausePath : playPath
        },
        ease: "power3.inOut"
    });
};

// 组件挂载时初始化路径
onMounted(() => {
    // 初始渲染播放图标
    if (iconPath.value) {
        iconPath.value.setAttribute('d', playPath);
    }
});
</script>

<style scoped></style>