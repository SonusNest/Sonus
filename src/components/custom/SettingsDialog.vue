<script setup lang="ts">
import {
    Bell,
    Globe,
    Paintbrush,
    Settings,
    Palette,
    ChevronsUpDown
} from "lucide-vue-next"

import { ref, computed, onMounted } from "vue"

import { Button } from "@/components/ui/button"

import {
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"

import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogTitle
} from "@/components/ui/dialog"
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarProvider,
} from "@/components/ui/sidebar"

import { Combobox, ComboboxAnchor, ComboboxEmpty, ComboboxGroup, ComboboxItem, ComboboxList, ComboboxTrigger } from "@/components/ui/combobox"

import { useAppStore } from '@/stores/appStore'

import { invoke } from '@tauri-apps/api/core'


const appStore = useAppStore()

const settingsDialogOpen = computed({
    get() {
        return appStore.getSettingsDialogState
    },
    set(value) {
        appStore.setSettingsDialogState(value)
    }
})

const data = {
    nav: [
        { name: "Appearance", icon: Paintbrush, id: 'appearance' },
        { name: "Notifications", icon: Bell, id: 'notifications' },
        { name: "Language & region", icon: Globe, id: 'language' },
        { name: "Advanced", icon: Settings, id: 'advanced' },
    ],
}

const currentTab = ref('appearance')

const getCurrentTabName = computed(() => {
    // @ts-ignore
    return data.nav.find(item => item.id === currentTab.value).name
})

/**
 * Window Material
 * 
 * @description
 * The window material is the material that the window is made of.
 * It can be changed to make the window look different.
 */

interface MaterialInfo {
    id: number;
    name: string;
}

const materials = ref<MaterialInfo[]>([])

const selectedMaterial = ref<MaterialInfo | null>(null)

const setMaterial = (material: MaterialInfo) => {
    invoke('set_window_material', { material: material.id })
        .then((value: unknown) => {
            if (material.id === 0 || material.id === 1) {
                appStore.setGlobalBackdropState(false)
            } else {
                appStore.setGlobalBackdropState(true)
            }
            console.log('Set window material success:', value)
            selectedMaterial.value = material
        })
        .catch((error) => {
            console.error('Set window material failed:', error)
        });

}

/**
 * Audio output devices
 * 
 * @description
 * The audio output devices are the devices that the audio is output to.
 * It can be changed to make the audio output to a different device.
 */

 interface AudioDeviceInfo {
    id: number;
    name: string;
}

const audioDevices = ref<AudioDeviceInfo[]>([])

const selectedAudioDevice = ref<AudioDeviceInfo | null>(null)

const setAudioDevice = (device: AudioDeviceInfo) => {
    invoke('set_audio_device', { device: device.id })
        .then((value: unknown) => {
            console.log('Set audio device success:', value)
        })
        .catch((error) => {
            console.error('Set audio device failed:', error)
        });
}


onMounted(() => {
    invoke('get_supported_window_materials')
        .then((value: unknown) => {
            materials.value = (value as MaterialInfo[]).map((material) => ({
                id: material.id,
                name: material.name
            }));
        })
        .catch((error) => {
            console.error('Get supported window materials failed:', error);
        });

    invoke('get_current_window_materials')
        .then((value: unknown) => {
            const currentMaterials = (value as MaterialInfo[]).map((material) => ({
                id: material.id,
                name: material.name
            }));
            selectedMaterial.value = currentMaterials[0];
        })
        .catch((error) => {
            console.error('Get current window materials failed:', error);
        });


});
</script>

<template>
    <Dialog v-model:open="settingsDialogOpen">
        <DialogContent class="overflow-hidden p-0 md:max-h-[500px] md:max-w-[700px] lg:max-w-[800px]">
            <DialogTitle class="sr-only">
                Settings
            </DialogTitle>
            <DialogDescription class="sr-only">
                Settings
            </DialogDescription>
            <SidebarProvider class="items-start">
                <Sidebar collapsible="none" class="hidden md:flex bg-muted/10 border-r h-full">

                    <SidebarContent>
                        <SidebarGroup>
                            <SidebarGroupContent>
                                <SidebarMenu>
                                    <SidebarMenuItem v-for="item in data.nav" :key="item.name">
                                        <SidebarMenuButton as-child :is-active="currentTab === item.id"
                                            class="cursor-pointer">
                                            <a @click="currentTab = item.id">
                                                <component :is="item.icon" />
                                                <span>{{ item.name }}</span>
                                            </a>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarGroupContent>
                        </SidebarGroup>
                    </SidebarContent>
                </Sidebar>
                <main class="flex h-[480px] flex-1 flex-col overflow-hidden">
                    <header
                        class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-[[data-collapsible=icon]]/sidebar-wrapper:h-12">
                        <div class="flex items-center gap-2 px-4">
                            <Breadcrumb>
                                <BreadcrumbList>
                                    <BreadcrumbItem class="hidden md:block">
                                        <BreadcrumbLink href="#">
                                            Settings
                                        </BreadcrumbLink>
                                    </BreadcrumbItem>
                                    <BreadcrumbSeparator class="hidden md:block" />
                                    <BreadcrumbItem>
                                        <BreadcrumbPage>{{ getCurrentTabName }}</BreadcrumbPage>

                                    </BreadcrumbItem>
                                </BreadcrumbList>
                            </Breadcrumb>
                        </div>
                    </header>
                    <div v-if="currentTab === 'appearance'" class="flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0">
                        <div class=" flex items-center space-x-4 rounded-md border p-4">
                            <Palette />
                            <div class="flex-1 space-y-1">
                                <p class="text-sm font-medium leading-none">
                                    Window Materiel
                                </p>
                                <p class="text-sm text-muted-foreground">
                                    Change the window material.
                                </p>
                            </div>
                            <Combobox>
                                <ComboboxAnchor as-child>
                                    <ComboboxTrigger as-child>
                                        <Button variant="outline"
                                            class="flex p-2 justify-between items-center rounded-md border hover:bg-muted text-sm">
                                            {{ selectedMaterial?.name }}

                                            <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />

                                        </Button>
                                    </ComboboxTrigger>
                                </ComboboxAnchor>

                                <ComboboxList>
                                    <ComboboxEmpty>
                                        Nothing found.
                                    </ComboboxEmpty>

                                    <ComboboxGroup>
                                        <ComboboxItem v-for="material in materials" :key="material.id"
                                            class="hover:bg-muted" :value="material" @select="() => {
                                                setMaterial(material)
                                            }">
                                            {{ material.name }}
                                        </ComboboxItem>
                                    </ComboboxGroup>
                                </ComboboxList>
                            </Combobox>
                        </div>
                    </div>
                    <div v-if="currentTab === 'notifications'"
                        class="flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0">
                    </div>
                    <div v-if="currentTab === 'language'" class="flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0">
                    </div>
                    <div v-if="currentTab === 'advanced'" class="flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0">
                        <div class=" flex items-center space-x-4 rounded-md border p-4">
                            <Palette />
                            <div class="flex-1 space-y-1">
                                <p class="text-sm font-medium leading-none">
                                    Output Device
                                </p>
                                <p class="text-sm text-muted-foreground">
                                    Set up audio output device.
                                </p>
                            </div>
                            <Combobox>
                                <ComboboxAnchor as-child>
                                    <ComboboxTrigger as-child>
                                        <Button variant="outline"
                                            class="flex p-2 justify-between items-center rounded-md border hover:bg-muted text-sm">
                                            {{ selectedAudioDevice?.name }}

                                            <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />

                                        </Button>
                                    </ComboboxTrigger>
                                </ComboboxAnchor>

                                <ComboboxList>
                                    <ComboboxEmpty>
                                        Nothing found.
                                    </ComboboxEmpty>

                                    <ComboboxGroup>
                                        <ComboboxItem v-for="device in audioDevices" :key="device.id"
                                            class="hover:bg-muted" :value="device" @select="() => {
                                                setAudioDevice(device)
                                            }">
                                            {{ device.name }}
                                        </ComboboxItem>
                                    </ComboboxGroup>
                                </ComboboxList>
                            </Combobox>
                        </div>
                    </div>

                </main>
            </SidebarProvider>
        </DialogContent>
    </Dialog>
</template>
