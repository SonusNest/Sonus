<template>
    <div class="w-full relative">
        <div class="flex gap-2 items-center py-4">
            <div class="text-2xl font-bold mr-5">Songs</div>
            <Input class="max-w-sm" placeholder="Filter songs..."
                :model-value="table.getColumn('title')?.getFilterValue() as string"
                @update:model-value=" table.getColumn('title')?.setFilterValue($event)" />
            <DropdownMenu>
                <DropdownMenuTrigger as-child>
                    <Button variant="outline" class="ml-auto">
                        Columns
                        <ChevronDown class="ml-2 h-4 w-4" />
                    </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                    <DropdownMenuCheckboxItem
                        v-for="column in table.getAllColumns().filter((column) => column.getCanHide())" :key="column.id"
                        class="capitalize" :model-value="column.getIsVisible()" @update:model-value="(value) => {
                            column.toggleVisibility(!!value)
                        }">
                        {{ column.id }}
                    </DropdownMenuCheckboxItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
        <div class="rounded-md border">
            <Table>
                <TableHeader>
                    <TableRow v-for="headerGroup in table.getHeaderGroups()" :key="headerGroup.id">
                        <TableHead v-for="header in headerGroup.headers" :key="header.id"
                            :data-pinned="header.column.getIsPinned()" :class="cn(
                                { 'sticky bg-background/95': header.column.getIsPinned() },
                                header.column.getIsPinned() === 'left' ? 'left-0' : 'right-0',
                            )">
                            <FlexRender v-if="!header.isPlaceholder" :render="header.column.columnDef.header"
                                :props="header.getContext()" />
                        </TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <template v-if="table.getRowModel().rows?.length">
                        <template v-for="row in table.getRowModel().rows" :key="row.id">
                            <TableRow :data-state="row.getIsSelected() && 'selected'">
                                <TableCell v-for="cell in row.getVisibleCells()" :key="cell.id"
                                    :data-pinned="cell.column.getIsPinned()" :class="cn(
                                        { 'sticky bg-background/95': cell.column.getIsPinned() },
                                        cell.column.getIsPinned() === 'left' ? 'left-0' : 'right-0',
                                    )">
                                    <FlexRender :render="cell.column.columnDef.cell" :props="cell.getContext()" />
                                </TableCell>
                            </TableRow>
                            <TableRow v-if="row.getIsExpanded()">
                                <TableCell :colspan="row.getAllCells().length">
                                    {{ JSON.stringify(row.original) }}
                                </TableCell>
                            </TableRow>
                        </template>
                    </template>

                    <TableRow v-else>
                        <TableCell :colspan="columns.length" class="h-24 text-center">
                            No results.
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </div>

        <div class="flex items-center justify-end space-x-2 py-4">
            <div class="flex-1 text-sm text-muted-foreground">
                {{ table.getFilteredSelectedRowModel().rows.length }} of
                {{ table.getFilteredRowModel().rows.length }} row(s) selected.
            </div>
            <div class="space-x-2">
                <Button variant="outline" size="sm" :disabled="!table.getCanPreviousPage()"
                    @click="table.previousPage()">
                    Previous
                </Button>
                <Button variant="outline" size="sm" :disabled="!table.getCanNextPage()" @click="table.nextPage()">
                    Next
                </Button>
            </div>
        </div>
        <div v-if="data === null"
            class="absolute w-full h-full top-0 flex items-center justify-center bg-background/50">
            <Loader2 class="w-8 h-8 animate-spin" />
        </div>
    </div>
</template>

<script setup lang="ts">
import type {
    ColumnFiltersState,
    ExpandedState,
    SortingState,
    VisibilityState,
} from "@tanstack/vue-table"
import {
    createColumnHelper,
    FlexRender,
    getCoreRowModel,
    getExpandedRowModel,
    getFilteredRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    useVueTable,
} from "@tanstack/vue-table"
import { ChevronDown, ChevronsUpDown, Loader2, Heart } from "lucide-vue-next"

import {h, ref} from "vue"
import { cn } from "@/lib/utils"
import { valueUpdater } from "@/components/ui/table/utils"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import {
    DropdownMenu,
    DropdownMenuCheckboxItem,
    DropdownMenuContent,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Input } from "@/components/ui/input"
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import {invoke} from "@tauri-apps/api/core";
import DefaultCover from "@/assets/default.png"

export interface Song {
    id: string
    title: string
    album: string
    artist: string[]
    genre: string[]
    duration: number
    cover_art: string[] // 不传播放列表和底栏组件不显示
    release_date: Date
    audio_format: string
    is_love: number
    remark: string
}

import {Track, usePlayerStore} from "@/stores/playerStore"

const playerStore = usePlayerStore()

const data = ref<Song[]>([])
invoke("get_all_songs", {
  limit: 100,
  offset: 0,
}).then((res) => {
  data.value = res as Song[]
})

const columnHelper = createColumnHelper<Song>()

const columns = [
    columnHelper.display({
        id: "select",
        header: ({ table }) => h(Checkbox, {
            "modelValue": table.getIsAllPageRowsSelected() || (table.getIsSomePageRowsSelected() && "indeterminate"),
            "onUpdate:modelValue": value => table.toggleAllPageRowsSelected(!!value),
            "ariaLabel": "Select all",
        }),
        cell: ({ row }) => {
            return h(Checkbox, {
                "modelValue": row.getIsSelected(),
                "onUpdate:modelValue": value => row.toggleSelected(!!value),
                "ariaLabel": "Select row",
            })
        },
        enableSorting: false,
        enableHiding: false,
    }),
    columnHelper.accessor("is_love", {
        enablePinning: true,
        header: "",
        cell: ({ row }) => h(Heart, {
            class: "size-4 text-right",
            fill: row.getValue("is_love") === 1 ? "red" : "transparent",
            stroke: row.getValue("is_love") === 1 ? "red" : "transparent",
        }),
    }),
    columnHelper.accessor("cover_art", {
        header: () => h("div", { class: "text-left" }, ""),
      cell: ({ row }) => {
        let cover_art = row.getValue("cover_art")
        //@ts-ignore
        if (cover_art[0] === "default") {
          cover_art = DefaultCover
        }

        // 避免数据量，优化不必要数据
        //@ts-ignore
        row.original.remark = ""
        const handlePlay = () => {
          playerStore.playTrack([row.original as Track])
        };

        return h("div", {
          class: "relative w-8 h-8 rounded overflow-hidden group",
          style: "cursor: pointer" // 鼠标悬停时显示指针
        }, [
          // 专辑封面背景层
          h("div", {
            class: "w-full h-full",
            style: `background-image: url(${cover_art}); background-size: cover; background-position: center;`
          }),

          // Hover 时显示的灰色背景 + 播放图标
          h("div", {
            class: "absolute inset-0 bg-gray-500/50 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-200",
            style: "backdrop-filter: blur(1px);" // 可选：轻微模糊背景增强图标可见性
          }, [
            // 自定义 SVG 播放图标
            h("svg", {
              onClick: handlePlay, // 绑定播放点击事件
              class: "w-5 h-5 text-white", // 图标大小和颜色
              viewBox: "0 0 24 24",
              fill: "currentColor",
              style: "margin-left: 1px; transition: transform 0.2s;" // 图标微调与缩放过渡
            }, [
              h("path", {
                d: "M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z"
              })
            ])
          ])
        ]);
      },
    }),
    columnHelper.accessor("title", {
        header: ({ column }) => {
            return h(Button, {
                variant: "ghost",
                onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
            }, () => ["Title", h(ChevronsUpDown, { class: "ml-2 h-4 w-4" })])
        },
        cell: ({ row }) => h("div", row.getValue("title")),
    }),
    columnHelper.accessor("artist", {
        header: () => h("div", { class: "text-right" }, "Artist"),
        cell: ({ row }) => {
            const artist = row.getValue("artist") as string[]
            return h("div", { class: "text-right font-medium" }, artist.join(", "))
        },
    }),
    columnHelper.accessor("album", {
        header: () => h("div", { class: "text-right" }, "Album"),
        cell: ({ row }) => {
            return h("div", { class: "text-right font-medium" }, row.getValue("album"))

        },
    }),
    columnHelper.accessor("genre", {
        header: () => h("div", { class: "text-right" }, "Genre"),
        cell: ({ row }) => {
            const genre = row.getValue("genre") as string[]
            return h("div", { class: "text-right font-medium" }, genre.join(", "))
        },
    }),
    columnHelper.accessor("duration", {
        header: () => h("div", { class: "text-right" }, "Time"),
        cell: ({ row }) => {
            const duration = row.getValue("duration")
            return h("div", { class: "text-right font-medium" }, formatSecondToTime(duration as number))
        },
    }),
    columnHelper.accessor("release_date", {
        header: () => h("div", { class: "text-right" }, "Released"),
        cell: ({ row }) => {
            const release_date = row.getValue("release_date")
            return h("div", { class: "text-right font-medium" }, new Date(release_date as string).toLocaleDateString())

        },
    }),
    columnHelper.display({
        id: "actions",
        enableHiding: false,
        cell: ({ row }) => {
            const payment = row.original

            return h("div", { class: "relative" }, h(DropdownMenu, {
                payment,
                onExpand: row.toggleExpanded,
            }))
        },
    }),
    columnHelper.accessor("audio_format", {
        header: () => h("div", { class: "text-right" }, "File Type"),
        cell: ({ row }) => {
            return h("div", { class: "text-right font-medium" }, row.getValue("audio_format"))
        },
    }),
]

const sorting = ref<SortingState>([])
const columnFilters = ref<ColumnFiltersState>([])
const columnVisibility = ref<VisibilityState>({})
const rowSelection = ref({})
const expanded = ref<ExpandedState>({})

const table = useVueTable({
    data: data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    getExpandedRowModel: getExpandedRowModel(),
    onSortingChange: updaterOrValue => valueUpdater(updaterOrValue, sorting),
    onColumnFiltersChange: updaterOrValue => valueUpdater(updaterOrValue, columnFilters),
    onColumnVisibilityChange: updaterOrValue => valueUpdater(updaterOrValue, columnVisibility),
    onRowSelectionChange: updaterOrValue => valueUpdater(updaterOrValue, rowSelection),
    onExpandedChange: updaterOrValue => valueUpdater(updaterOrValue, expanded),
    state: {
        get sorting() { return sorting.value },
        get columnFilters() { return columnFilters.value },
        get columnVisibility() { return columnVisibility.value },
        get rowSelection() { return rowSelection.value },
        get expanded() { return expanded.value },
        columnPinning: {
            left: ["is_love"],
            right: ["actions"],
        },
    },
})

const formatSecondToTime = (second: number) => {
    const minute = Math.floor(second / 60)
    const seconds = Math.floor(second % 60)
    return `${minute}:${seconds < 10 ? '0' + seconds : seconds}`
}


</script>

<style scoped></style>