<template>
    <div class="h-full w-full flex flex-row overflow-hidden">
      <SidebarProvider class="h-full w-full">
        <Sidebar v-bind="props" :class="{ close: !sidebarState }" class="sidebar">
          <SidebarContent class="sidebar-content w-[15rem] max-w-[15rem] transition-transform duration-350 pl-2">
            <Collapsible v-for="item in items" :key="item.title" :title="item.title" default-open
                         class="group/collapsible w-full">
              <SidebarGroup class="p-0 w-full">
                <SidebarGroupLabel as-child class="group/label text-sm text-sidebar-foreground h-6 pr-0">
                  <CollapsibleTrigger class="text-sidebar-foreground/60 text-xs font-bold">
                    {{ item.title }}
                    <ChevronRight
                        class="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90" />
                  </CollapsibleTrigger>
                </SidebarGroupLabel>
                <CollapsibleContent class="w-full">
                  <SidebarGroupContent>
                    <SidebarMenu class="w-full">
                      <SidebarMenuItem v-for="childItem in item.items" :key="childItem.title"
                                       class="w-full">
                        <SidebarMenuButton as-child :is-active="childItem.isActive" class="w-full cursor-pointer">
                          <a href="#"
                             @click="router.push(childItem.url)"
                             class="w-full text-ellipsis whitespace-nowrap overflow-hidden"
                             :title="childItem.title">
                            <component :is="childItem.icon" />
                            {{ childItem.title }}
                          </a>
                        </SidebarMenuButton>
                      </SidebarMenuItem>
                    </SidebarMenu>
                  </SidebarGroupContent>
                </CollapsibleContent>
              </SidebarGroup>
            </Collapsible>
          </SidebarContent>
          <SidebarRail />
        </Sidebar>
        <SidebarInset class="rounded-2xl border overflow-auto h-full w-full mr-3 ml-3 bg-sidebar/80 p-5">
          <RouterView />
        </SidebarInset>
      </SidebarProvider>
      <div :class="{ close: !rightAreaState }" class="rightArea w-100 h-full">
        <!-- 给内容容器添加 overflow-y-auto 和 h-full 确保滚动效果 -->
        <div class="rightArea-content w-100 h-full transition-transform duration-350 pr-2">
            <LyricsComponent v-if="rightArea === 'lyrics'"/>
            <PlaylistComponent v-if="rightArea === 'playlist'"/>
        </div>
      </div>
    </div>
</template>

<script setup lang="ts">
import type { SidebarProps } from "@/components/ui/sidebar"
import { 
    Home,
    LayoutGrid,
    ListMusic,
    Star,
    Clock,
    Disc3,
    ChevronRight,
    BookImage,
    Users

 } from "lucide-vue-next";
import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from "@/components/ui/collapsible"
import {
    Sidebar,
    SidebarContent,
    SidebarInset,
    SidebarProvider,
    SidebarGroup,
    SidebarGroupLabel,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarRail
} from "@/components/ui/sidebar"
import { useRouter } from "vue-router"

import {computed,
  // ref
} from 'vue'
import { useAppStore } from '@/stores/appStore'
import PlaylistComponent from "@/components/custom/PlaylistComponent.vue";
import LyricsComponent from "@/components/custom/LyricsComponent.vue";

const router = useRouter()

const props = withDefaults(defineProps<SidebarProps>(), {
    collapsible: "none",
})
const items = [
    {
        title: "Souns Music",
        items: [
            {
                title: "Home",
                url: "/home",
                icon: Home,
                isActive: false,
            },
            {
                title: "Music Memory",
                url: "#",
                icon: LayoutGrid,
                isActive: false,
            }
        ],
    },
    {
        title: "My Library",
        items: [
            {
                title: "Favorite Songs",
                url: "/favorite-songs",
                icon: Star,
                isActive: false,
            },
            {
                title: "Recently Played",
                url: "/recently-played",
                icon: Clock,
                isActive: false,
            },
            {
                title: "Songs",
                url: "/songs",
                icon: ListMusic,
                isActive: false,
            },
            {
                title: "Albums",
                url: "/album",
                icon: BookImage,
                isActive: false,
            },
            {
                title: "Artists",
                url: "/artist",
                icon: Users,
                isActive: false,
            }
        ],
    },
    {
        title: "Playlists",
        url: "#",
        icon: ListMusic,
        items: [
            {
                title: "Add...",
                url: "#",
                icon: Disc3,
                isActive: false,
            },
            {
                title: "All Playlists",
                url: "/playlist",
                icon: Disc3,
                isActive: false,
            },
            {
                title: "The best-selling Mandarin song of 2000",
                url: "#",
                icon: Disc3,
                isActive: false,
            },
            {
                title: "Weekly Top 100 Songs: Global",
                url: "#",
                icon: Disc3,
                isActive: false,
            },
            {
                title: "Hot hit song: Chinese pop",
                url: "#",
                icon: Disc3,
                isActive: false,
            },
            {
                title: "New discoveries in music",
                url: "#",
                icon: Disc3,
                isActive: false,
            },
        ],
    },
]


// 获取appStore实例
const appStore = useAppStore()
// const playerStore = usePlayerStore()
// 通过计算属性获取sidebarState状态
const sidebarState = computed(() => {
    return appStore.getSidebarState
})

const rightArea = computed(() => {
    return appStore.getRightArea
})
const rightAreaState = computed(() => {
    return appStore.getRightAreaState
})
</script>

<style scoped>
.sidebar {
    transition: width 0.3s ease-in-out;
}
.close {
    width: 0;
    overflow: hidden;
}
.sidebar-content {
    transform: translateX(0);
}

.sidebar.close .sidebar-content {
    transform: translateX(-100%);
}

.rightArea {
  transition: width 0.3s ease-in-out;
}
.rightArea-content {
  transform: translateX(0);
}
.rightArea.close .rightArea-content {
  transform: translateX(100%);
}
</style>