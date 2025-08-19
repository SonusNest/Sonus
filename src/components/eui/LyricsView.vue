<template>
    <div ref="lyricsview" class="md-body lyric-body">
        <template v-if="lyrics && lyrics.length > 0 && !qqInstrumental(lyrics)">
            <template v-for="(lyric, index) in lyrics" :key="index"
                v-if="lyric && lyric.line && lyric.line !== 'lrcInstrumental'">
                <h3 class="lyric-line" @click="seekTo(lyric.startTime)"
                    :class="{ unsynced: lyric.startTime === 9999999 }" :line-index="index.toString()">
                    <template v-if="richlyrics && richlyrics.length > 0">
                        <div class="richl">
                            <template v-for="verse in getVerseLine(index - 1)" :key="verse.o">
                                <span class="verse" :lyricstart="lyric.startTime" :versestart="verse.o">
                                    {{ verse.c }}
                                </span>
                            </template>
                        </div>
                    </template>
                    <template v-else>
                        <div class="norm">
                            {{ lyric.line }}
                        </div>
                    </template>
                    <div class="lyrics-translation" v-if="lyric.translation && lyric.translation !== ''">
                        {{ lyric.translation }}
                    </div>
                </h3>
            </template>
            <template v-else>
                <h3 class="lyric-line" @click="seekTo(lyric.startTime)" :start="lyric.startTime" :end="lyric.endTime"
                    :line-index="index.toString()" :key="index">
                    <div class="lyricWaiting">
                        <div class='WaitingDot1'></div>
                        <div class='WaitingDot2'></div>
                        <div class='WaitingDot3'></div>
                    </div>
                </h3>
            </template>
        </template>
        <template v-else>
            <div class="no-lyrics">
                {{ app.getLz('term.noLyrics') }}
            </div>
        </template>
    </div>
</template>

<script setup>
import { ref, watch, onMounted, getCurrentInstance } from 'vue';

const props = defineProps({
    time: {
        type: Number,
        required: true
    },
    lyrics: {
        type: Array,
        default: () => []
    },
    richlyrics: {
        type: Array,
        default: () => []
    },
    translation: {
        type: String,
        default: ''
    },
    onindex: {
        type: Number,
        default: 0
    },
    yoffset: {
        type: Number,
        default: 128
    }
});

const emit = defineEmits(['seek']);

// 获取应用实例
const instance = getCurrentInstance();
const app = instance.appContext.config.globalProperties.$root;

// 引用
const lyricsview = ref(null);
let position = ref(0);
let checkIfScrollIsStatic = ref(null);

// 方法
const seekTo = (startTime) => {
    if (startTime !== 9999999) {
        app.seekTo(startTime, false);
    }
};

const getVerseLine = (index) => {
    if (props.richlyrics[index] && props.richlyrics[index].l) {
        return props.richlyrics[index].l;
    }
    return [];
};

const qqInstrumental = (lyrics) => {
    for (const lyric of lyrics) {
        if (lyric.line.includes("纯音乐") && lyric.line.includes("欣赏")) {
            return true;
        }
    }
    return false;
};

const getActiveLyric = () => {
    const delayfix = app.activeCasts[0]?.hasOwnProperty('airplay2') ? -2.5 : 0.1;
    const prevLine = app.currentLyricsLine;

    for (let i = 0; i < props.lyrics.length; i++) {
        if (
            (props.time + delayfix >= props.lyrics[i].startTime) &&
            (props.time + delayfix <= app.lyrics[i].endTime)
        ) {
            if (app.currentLyricsLine !== i) {
                app.currentLyricsLine = i;

                if (
                    ((app.lyricon && app.drawer.open) || app.appMode === 'fullscreen') &&
                    lyricsview.value?.querySelector(`.lyric-line[line-index="${i}"]`)
                ) {
                    if (lyricsview.value.querySelector(`.lyric-line[line-index="${prevLine}"]`)) {
                        lyricsview.value.querySelector(`.lyric-line[line-index="${prevLine}"]`).classList.remove("active");
                    }

                    lyricsview.value.querySelector(`.lyric-line[line-index="${i}"]`).classList.add("active");

                    if (checkIfScrollIsStatic.value) {
                        const lyricElement = lyricsview.value.querySelector(`.lyric-line[line-index="${i}"]`);
                        const parent = lyricElement.parentElement;
                        const parentRect = parent.getBoundingClientRect();
                        const lyricElementRect = lyricElement.getBoundingClientRect();
                        const parentScrollTop = parent.scrollTop;
                        const parentScrollLeft = parent.scrollLeft;
                        const parentScrollTopDiff = parentScrollTop - parentRect.top;
                        const parentScrollLeftDiff = parentScrollLeft - parentRect.left;
                        const lyricElementScrollTop = lyricElementRect.top + parentScrollTopDiff;
                        const lyricElementScrollLeft = lyricElementRect.left + parentScrollLeftDiff;
                        const scrollTopDiff = lyricElementScrollTop - parentScrollTop;
                        const scrollLeftDiff = lyricElementScrollLeft - parentScrollLeft;
                        const scrollTop = parent.scrollTop + scrollTopDiff;
                        const scrollLeft = parent.scrollLeft + scrollLeftDiff;

                        parent.scrollTo({
                            top: scrollTop - (props.yoffset ?? 128),
                            left: scrollLeft,
                            behavior: 'smooth'
                        });
                    }
                }
            } else if (app.currentLyricsLine === 0 && app.drawer.open) {
                const firstLine = lyricsview.value?.querySelector(`.lyric-line[line-index="0"]`);
                if (firstLine && !firstLine.classList.contains("active")) {
                    firstLine.classList.add("active");
                }
            }
            break;
        }
    }

    try {
        if ((app.drawer.open) || app.appMode === 'fullscreen') {
            try {
                const prevLineEl = lyricsview.value?.querySelector(`.lyric-line[line-index="${prevLine}"]`);
                if (prevLineEl) {
                    prevLineEl.childNodes.forEach(node => {
                        node.classList?.remove("verse-active");
                    });
                }
            } catch (e) {
                console.error(e);
            }

            const currentLineEl = lyricsview.value?.querySelector(`.lyric-line[line-index="${app.currentLyricsLine}"]`);
            if (currentLineEl) {
                const verses = currentLineEl.querySelectorAll(".verse");
                verses.forEach(child => {
                    if (props.time + delayfix >= Number(child.getAttribute("lyricstart")) + Number(child.getAttribute("versestart"))) {
                        child.classList.add("verse-active");
                    } else {
                        child.classList.remove("verse-active");
                    }
                });
            }
        }
    } catch (e) {
        console.error(e);
    }
};

// 监听时间变化
watch(
    () => props.time,
    () => {
        if (
            ((app.lyricon && app.drawer.open) || app.appMode === 'fullscreen') &&
            lyricsview.value
        ) {
            const currentLine = lyricsview.value.querySelector(`.lyric-line.active`);

            if (currentLine && currentLine.getElementsByClassName('lyricWaiting').length > 0) {
                const duration = Number(currentLine.getAttribute("end")) - Number(currentLine.getAttribute("start"));
                const u = (props.time - Number(currentLine.getAttribute("start"))) / duration;

                if (u < 0.25 && !currentLine.classList.contains('mode1')) {
                    try {
                        currentLine.classList.add('mode1');
                        currentLine.classList.remove('mode3');
                        currentLine.classList.remove('mode2');
                    } catch (e) {
                        console.error(e);
                    }

                    currentLine.getElementsByClassName('WaitingDot1')[0].style.animation = `dotOpacity ${0.25 * duration}s cubic-bezier(0.42, 0, 0.58, 1) forwards`;
                    currentLine.getElementsByClassName('WaitingDot2')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot3')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot2')[0].style.opacity = 0.25;
                    currentLine.getElementsByClassName('WaitingDot3')[0].style.opacity = 0.25;
                } else if (u >= 0.25 && u < 0.5 && !currentLine.classList.contains('mode2')) {
                    try {
                        currentLine.classList.add('mode2');
                        currentLine.classList.remove('mode1');
                        currentLine.classList.remove('mode3');
                    } catch (e) {
                        console.error(e);
                    }

                    currentLine.getElementsByClassName('WaitingDot2')[0].style.animation = `dotOpacity ${0.25 * duration}s cubic-bezier(0.42, 0, 0.58, 1) forwards`;
                    currentLine.getElementsByClassName('WaitingDot1')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot3')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot1')[0].style.opacity = 1;
                    currentLine.getElementsByClassName('WaitingDot3')[0].style.opacity = 0.25;
                } else if (u >= 0.5 && u < 0.75 && !currentLine.classList.contains('mode3')) {
                    try {
                        currentLine.classList.add('mode3');
                        currentLine.classList.remove('mode1');
                        currentLine.classList.remove('mode2');
                    } catch (e) {
                        console.error(e);
                    }

                    currentLine.getElementsByClassName('WaitingDot3')[0].style.animation = `dotOpacity ${0.25 * duration}s cubic-bezier(0.42, 0, 0.58, 1) forwards`;
                    currentLine.getElementsByClassName('WaitingDot1')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot2')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot1')[0].style.opacity = 1;
                    currentLine.getElementsByClassName('WaitingDot2')[0].style.opacity = 1;
                } else if (u >= 0.75 && currentLine.classList.contains('mode3')) {
                    try {
                        currentLine.classList.remove('mode1');
                        currentLine.classList.remove('mode2');
                        currentLine.classList.remove('mode3');
                    } catch (e) {
                        console.error(e);
                    }

                    currentLine.getElementsByClassName('WaitingDot1')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot2')[0].style.animation = ``;
                    currentLine.getElementsByClassName('WaitingDot1')[0].style.opacity = 1;
                    currentLine.getElementsByClassName('WaitingDot2')[0].style.opacity = 1;
                }
            }
        }

        getActiveLyric();
    }
);

// 组件挂载时初始化
onMounted(() => {
    // 设置滚动检查定时器
    checkIfScrollIsStatic.value = setInterval(() => {
        try {
            if (position.value === lyricsview.value?.scrollTop) {
                clearInterval(checkIfScrollIsStatic.value);
            }
            position.value = lyricsview.value?.scrollTop;
        } catch (e) {
            console.error(e);
        }
    }, 50);
});

// 清理定时器
onUnmounted(() => {
    if (checkIfScrollIsStatic.value) {
        clearInterval(checkIfScrollIsStatic.value);
    }
});
</script>