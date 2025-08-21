<script setup lang="ts">
import { computed } from "vue"
import { Button } from "@/components/ui/button"
import { useAppStore } from "@/stores/appStore"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog"
import { invoke } from '@tauri-apps/api/core'

const appStore = useAppStore()

const developerOptionsDialogOpen = computed({
    get() {
        return appStore.getDeveloperOptionsDialogState
    },
    set(value) {
        appStore.setDeveloperOptionsDialogState(value)
    }
})

// Get Window HWND

const getWindowHWND = async () => {
    const hwnd = await invoke('get_window_hwnd').then((res) => {
        console.log(res)
        return res
    })
    return hwnd
}

const getSupportedWindowMaterials = async () => {
    const materials = await invoke('get_supported_window_materials').then((res) => {
        console.log(res)
        return res
    })
    return materials
}

const getCurrentWindowMaterials = async () => {
    const materials = await invoke('get_current_window_materials').then((res) => {
        console.log(res)
        return res
    })
    return materials
}

const getMusicMetadata = async () => {
    const metadata = await invoke('get_song_metadata_by_file_path', { file_path: 'D:\\Project\\Gabriel Ryder\\Sonus\\src\\assets\\test.flac' }).then((res) => {
        console.log(res)
        return res
    })
    return metadata
}

const startDirectoryScan = async () => {
  const scanResult = await invoke('start_directory_scan', { path: "D:\\Music\\music" }).then((res) => {
    console.log(res)
    return res
  })
  return scanResult
}

const getTaskStats = async () => {
  const stats = await invoke('get_task_stats').then((res) => {
    console.log(res)
    return res
  })
  return stats
}

const registerTaskListener = async () => {
  try {
    // 调用后端命令完成监听器注册
    await invoke('register_task_listener')
    console.log('任务监听器已注册')

    // 当需要取消监听时调用
    // unlisten()
  } catch (error) {
    console.error('注册任务监听器失败:', error)
  }
}


</script>

<template>
    <Dialog v-model:open="developerOptionsDialogOpen">
        <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
                <DialogTitle>Developer Options</DialogTitle>
                <DialogDescription>
                    If you don't understand the consequences of clicking, please don't try easily.
                </DialogDescription>
            </DialogHeader>
            <div class="grid gap-4 py-4">
                <Button @click="getWindowHWND">
                    Get Window HWND
                </Button>
                <Button @click="getSupportedWindowMaterials">
                    Get Supported Window Materials
                </Button>
                <Button @click="getCurrentWindowMaterials">
                    Get Current Window Materials
                </Button>
                <Button @click="getMusicMetadata">
                    Get Music Metadata
                </Button>
                <Button @click="startDirectoryScan">
                   Start Scan Directory
                </Button>
              <Button @click="getTaskStats">
                  Get Task Stats
              </Button>
                <Button @click="registerTaskListener">
                  Register Task Listener
                </Button>
            </div>
            <DialogFooter>
                <Button @click="appStore.setDeveloperOptionsDialogState(false)">
                    Close
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>