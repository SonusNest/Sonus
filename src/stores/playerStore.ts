import { defineStore } from 'pinia'

export const usePlayerStore = defineStore('player', {
    state: () => ({
        isPlaying: false,
        currentTime: 0,
        duration: 0,
    }),
    getters: {
        getIsPlaying: (state) => state.isPlaying,
        getCurrentTime: (state) => state.currentTime,
        getDuration: (state) => state.duration,
    },
    actions: {
        setIsPlaying(isPlaying: boolean) {
            this.isPlaying = isPlaying
        },
        setCurrentTime(currentTime: number) {
            this.currentTime = currentTime
        },
        setDuration(duration: number) {
            this.duration = duration
        }
    }
})

