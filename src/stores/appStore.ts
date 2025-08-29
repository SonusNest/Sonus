import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', {
    state: () => ({
        settingsDialogState: false,
        sidebarState: (() => {
            try {
                const stored = localStorage.getItem('sidebarState')
                return stored !== null ? JSON.parse(stored) : true
            } catch (error) {
                console.error('Failed to parse sidebarState from localStorage:', error)
                return true
            }
        })(),
        rightAreaState: false,
        rightArea: 'none',
        developerOptionsDialogState: false,
        aboutDialogState: false,
        immersionDrawerState: false,
        globalBackdropState: true  // true: transparent, false: not transparent
    }),
    getters: {
        getSettingsDialogState: (state) => state.settingsDialogState,
        getSidebarState: (state) => state.sidebarState,
        getRightAreaState: (state) => state.rightAreaState,
        getRightArea: (state) => state.rightArea,
        getDeveloperOptionsDialogState: (state) => state.developerOptionsDialogState,
        getAboutDialogState: (state) => state.aboutDialogState,
        getImmersionDrawerState: (state) => state.immersionDrawerState,
        getGlobalBackdropState: (state) => state.globalBackdropState,
    },
    actions: {
        setSettingsDialogState(value: boolean) {
            this.settingsDialogState = value
        },
        setSidebarState(value: boolean) {
            this.sidebarState = value
            try {
                localStorage.setItem('sidebarState', JSON.stringify(value))
            } catch (error) {
                console.error('Failed to save sidebarState to localStorage:', error)
            }
        },
        setRightAreaState(value: boolean) {
            this.rightAreaState = value
        },
        setRightArea(value: string) {
            this.rightArea = value
        },
        setDeveloperOptionsDialogState(value: boolean) {
            this.developerOptionsDialogState = value
        },
        setAboutDialogState(value: boolean) {
            this.aboutDialogState = value
        },
        setImmersionDrawerState(value: boolean) {
            this.immersionDrawerState = value
        },
        setGlobalBackdropState(value: boolean) {
            this.globalBackdropState = value
        }
    }
})