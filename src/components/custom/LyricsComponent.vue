<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { LyricPlayer } from "@applemusic-like-lyrics/vue"
import { LyricLine, parseTTML } from "@applemusic-like-lyrics/lyric";
import { usePlayerStore } from "@/stores/playerStore"
import { useAppStore } from "@/stores/appStore"

const playerStore = usePlayerStore()
const appStore = useAppStore()
const playerRef = ref(null)
const isParsing = ref(false)
const parsedLyricLines = ref<LyricLine[]>([])

watch(
    () => playerStore.currentTrack?.lyrics,
    async (newLyrics) => {
      if (!newLyrics) {
        parsedLyricLines.value = []
        return
      }

      if (isParsing.value) return
      isParsing.value = true

      try {
        setTimeout(() => {
          try {
            const parsedLyric = parseTTML(newLyrics)
            parsedLyricLines.value = Array.isArray(parsedLyric.lines) ? parsedLyric.lines : []
              console.log('Lyrics line count:', parsedLyricLines.value.length)
          } catch (error) {
            console.error('Lyrics parse error:', error)
            parsedLyricLines.value = []
          } finally {
            isParsing.value = false
          }
        }, 0)
      } catch (error) {
        console.error('Lyrics parse error:', error)
        parsedLyricLines.value = []
        isParsing.value = false
      }
    },
    { immediate: true }
)

const lyricLines = computed<LyricLine[]>(() => {
  return parsedLyricLines.value
})

watch(
    () => lyricLines.value.length,
    (length) => {
      if (length > 1000) {
        console.warn(`Lyrics line count(${length})`)
      }
    }
)
</script>

<template>
  <div class="h-full">
    <div v-if="isParsing" class="text-foreground flex items-center justify-center">Loading lyrics...</div>

    <LyricPlayer
        v-else-if="appStore.rightArea === 'lyrics' && lyricLines.length > 0"
        class="h-full w-full lp p-1 text-foreground lyrics-player"
        :lyric-lines="lyricLines"
        :current-time="playerStore.currentPosition"
        :hide-passed-lines="false"
        :enable-blur="false"
        :enable-scale="false"
        :align-position="0.1"
        :enable-spring="false"
        ref="playerRef"
        @line-click="(e) => { playerStore.currentPosition = e.line.getLine().startTime / 1000 }"
    />

    <div v-else-if="appStore.rightArea === 'lyrics'" class="no-lyric">
      <div class="text-foreground flex items-center justify-center">No lyrics available</div>
    </div>
  </div>
</template>

<style scoped>
/* -------------------------- 主歌词与逐字颜色分离设置 -------------------------- */
/* 1. 所有歌词行基础样式 */
:deep([class^="lyricLine-"]) {
  font-size: 20px;
  font-weight: 400;
  margin: 10px 0;
}

/* 2. 当前激活行样式 */
:deep([class^="lyricLine-"].active) {
  font-size: 20px;
  font-weight: 700;
}

:deep([class^="lyricPlayer-"]) {
  padding: 0;
}


/* 3. 主歌词容器（灰色背景容器） */
:deep([class^="lyricMainLine-"]) {
  color: #aaa; /* 主歌词容器灰色（可调整深浅） */
}
:deep([class^="lyricLine-"].active [class^="lyricMainLine-"]) {
   /* 激活行主容器颜色（稍亮灰色） */
}

/* 4. 逐字动画字符（黑色） */
/* 普通行的逐字字符 */
:deep([class^="lyricMainLine-"] span) {
  color: #000 !important; /* 逐字字符黑色 */
}
/* 激活行的逐字字符（可加深黑色或调整为其他颜色） */
:deep([class^="lyricLine-"].active [class^="lyricMainLine-"] span) {
  color: #000 !important; /* 激活行逐字字符保持黑色 */
}

/* 5. 强调字符保持黑色 */
:deep([class^="lyricMainLine-"] .emphasize span) {
  color: #000 !important;
}

/* 6. 副歌词文本 */
:deep([class^="lyricSubLine-"]) {
  font-size: 16px;
  color: #888; /* 副歌词灰色 */
}
:deep([class^="lyricLine-"].active [class^="lyricSubLine-"]) {
  color: #aaa;
  opacity: 0.8;
}

/* 7. 二重唱歌词行 */
:deep([class^="lyricLine-"][class*="lyricDuetLine-"]) {
  color: #a8a8ff;
}

/* 8. 已模糊的歌词行 */
:deep([class^="lyricLine-"][style*="filter: blur"]) {
  opacity: 0.6;
}
</style>
