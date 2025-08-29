import { defineStore } from 'pinia'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

export interface Track {
    id: string
    title: string
    artist: string[]
    album: string
    file_path: string
    duration?: number
    cover_art: string[]
    lyrics: string
}

export interface Playlist {
    id: string
    name: string
    tracks: Track[]
    created_at: string 
    updated_at: string 
}

export type PlayMode = 'Repeat' | 'Random' | 'Single'

export type PlaybackState = 'stopped' | 'playing' | 'paused'

export const usePlayerStore = defineStore('player', {
    state: () => ({
        playbackState: 'stopped' as PlaybackState,
        volume: 1.0,
        currentPosition: 0,
        totalDuration: null as number | null,
        currentFile: null as string | null,
        currentTrack: null as Track | null,
        currentPlaylist: {
            id: '',
            name: '',
            tracks: [],
            created_at: '',
            updated_at: ''
        } as Playlist,
        currentPlayMode: 'Repeat' as PlayMode,
        currentIndex: null as number | null,
        isLoading: false
    }),

    getters: {
        progressPercentage(): number {
            if (!this.totalDuration || this.totalDuration === 0) return 0
            return Math.min((this.currentPosition / this.totalDuration) * 100, 100)
        },

        formattedCurrentTime(): string {
            return formatDuration(this.currentPosition)
        },

        formattedTotalDuration(): string {
            return this.totalDuration ? formatDuration(this.totalDuration) : '00:00'
        },
    },

    actions: {
        async init() {
            await listen('player-state-updated', (event) => {
                const state = event.payload as {
                    playback_state: PlaybackState
                    volume: number
                    current_position: number
                    total_duration: number | null
                    current_file: string | null
                    current_track: Track | null
                    current_playlist: Playlist
                    current_play_mode: PlayMode
                    current_index: number | null
                }

                this.playbackState = state.playback_state
                this.volume = state.volume
                this.currentPosition = state.current_position
                this.totalDuration = state.total_duration
                this.currentFile = state.current_file
                this.currentTrack = state.current_track
                this.currentPlaylist = state.current_playlist
                this.currentPlayMode = state.current_play_mode
                this.currentIndex = state.current_index
                this.isLoading = false
            })
        },

        async play() {
            this.isLoading = true
            try {
                await invoke('play')
            } catch (error) {
                console.error('播放失败:', error)
                this.isLoading = false
            }
        },

        async pause() {
            try {
                await invoke('pause')
            } catch (error) {
                console.error('暂停失败:', error)
            }
        },

        async resume() {
            try {
                await invoke('resume')
            } catch (error) {
                console.error('恢复播放失败:', error)
            }
        },

        async nextTrack() {
            this.isLoading = true
            try {
                await invoke('next_track')
            } catch (error) {
                console.error('下一曲失败:', error)
                this.isLoading = false
            }
        },

        async prevTrack() {
            this.isLoading = true
            try {
                await invoke('previous_track')
            } catch (error) {
                console.error('上一曲失败:', error)
                this.isLoading = false
            }
        },

        async setVolume(volume: number) {
            // 限制音量范围0-1
            const clampedVolume = Math.max(0, Math.min(1, volume))
            this.volume = clampedVolume
            try {
                await invoke('set_volume', { volume: clampedVolume })
            } catch (error) {
                console.error('设置音量失败:', error)
            }
        },

        async seek(position: number) {
            try {
                await invoke('player_seek', { position })
            } catch (error) {
                console.error('调整进度失败:', error)
            }
        },

        async setPlayMode(mode: PlayMode) {
            this.currentPlayMode = mode
            try {
                await invoke('set_play_mode', { mode })
            } catch (error) {
                console.error('设置播放模式失败:', error)
            }
        },

        async setCurrentPlayMode(mode: PlayMode) {
            this.currentPlayMode = mode
        },

        async playTrack(tracks: Track[]) {
            this.isLoading = true
            try {
                await invoke('play_to_playlist', {
                    tracks,
                    playMode: "Single"
                })
            } catch (error) {
                console.error('播放指定轨道失败:', error)
                this.isLoading = false
            }
        },

        async stop() {
            try {
                await invoke('stop')
            } catch (error) {
                console.error('停止播放失败:', error)
            }
        }
    }
})
const formatDuration = (ms: number): string => {
    const totalSeconds = Math.floor(ms / 1000)
    const minutes = Math.floor(totalSeconds / 60)
    const seconds = totalSeconds % 60
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
}