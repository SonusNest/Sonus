import { defineStore } from 'pinia'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'

// 定义轨道类型接口（与后端Track结构对应）
export interface Track {
    id: string
    title: string
    artist: string
    album: string
    file_path: string
    duration?: number // 毫秒
    cover_art: string[]
    lyrics: string
    // 可根据后端Track结构补充其他字段
}

// 定义播放列表接口（与后端Playlist结构对应）
export interface Playlist {
    id: string
    name: string
    tracks: Track[]
    created_at: string // ISO时间字符串
    updated_at: string // ISO时间字符串
}

// 播放模式类型（与后端PlayMode对应）
export type PlayMode = 'Repeat' | 'Random' | 'Single'

// 播放状态类型
export type PlaybackState = 'stopped' | 'playing' | 'paused'

export const usePlayerStore = defineStore('player', {
    state: () => ({
        // 播放状态
        playbackState: 'stopped' as PlaybackState,
        // 音量（0-1范围）
        volume: 1.0,
        // 当前播放位置（毫秒）
        currentPosition: 0,
        // 总时长（毫秒）
        totalDuration: null as number | null,
        // 当前播放文件路径
        currentFile: null as string | null,
        // 当前播放轨道信息
        currentTrack: null as Track | null,
        // 当前播放列表
        currentPlaylist: {
            id: '',
            name: '',
            tracks: [],
            created_at: '',
            updated_at: ''
        } as Playlist,
        // 当前播放模式
        currentPlayMode: 'Repeat' as PlayMode,
        // 当前播放索引
        currentIndex: null as number | null,
        // 加载状态
        isLoading: false
    }),

    getters: {
        // 计算播放进度百分比
        progressPercentage(): number {
            if (!this.totalDuration || this.totalDuration === 0) return 0
            return Math.min((this.currentPosition / this.totalDuration) * 100, 100)
        },

        // 格式化当前时间（分:秒）
        formattedCurrentTime(): string {
            return formatDuration(this.currentPosition)
        },

        // 格式化总时长（分:秒）
        formattedTotalDuration(): string {
            return this.totalDuration ? formatDuration(this.totalDuration) : '00:00'
        },
    },

    actions: {

        // 初始化事件监听
        async init() {
            // 监听后端状态更新事件
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

                // 更新所有状态字段
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

        // 播放操作
        async play() {
            this.isLoading = true
            try {
                await invoke('play')
            } catch (error) {
                console.error('播放失败:', error)
                this.isLoading = false
            }
        },

        // 暂停操作
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

        // 下一曲
        async nextTrack() {
            this.isLoading = true
            try {
                await invoke('next_track')
            } catch (error) {
                console.error('下一曲失败:', error)
                this.isLoading = false
            }
        },

        // 上一曲
        async prevTrack() {
            this.isLoading = true
            try {
                await invoke('previous_track')
            } catch (error) {
                console.error('上一曲失败:', error)
                this.isLoading = false
            }
        },

        // 调整音量
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

        // 调整播放进度
        async seek(position: number) {
            try {
                await invoke('player_seek', { position })
            } catch (error) {
                console.error('调整进度失败:', error)
            }
        },

        // 切换播放模式
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

        // 播放指定轨道
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

        // 停止播放
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