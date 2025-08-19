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

import { h, ref } from "vue"
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

export interface Song {
    id: string
    title: string
    album: string
    artist: string[]
    genre: string[]
    duration: number
    cover_art: string
    release_date: Date
    is_love: number
}

const data: Song[] = [
    {
        id: "1",
        title: "Deadman",
        album: "Deadman - Single",
        artist: ["蔡徐坤"],
        genre: ["Pop"],
        duration: 180,
        cover_art: "data:image/avif;base64,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",
        release_date: new Date(),
        is_love: 1,
    },
    {
        id: "2",
        title: "我们的明天",
        album: "我们的明天 - Single",
        artist: ["en"],
        genre: ["Mandopop"],
        duration: 180,
        cover_art: "data:image/avif;base64,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",
        release_date: new Date(),
        is_love: 1,
    }
]

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
            const cover_art = row.getValue("cover_art")
            return h("div", { class: "w-8 h-8 rounded", style: "background-image: url(" + cover_art + "); background-size: cover; background-position: center;" })
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
            return h("div", { class: "text-right font-medium" }, (release_date as Date).toLocaleDateString())

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
]

const sorting = ref<SortingState>([])
const columnFilters = ref<ColumnFiltersState>([])
const columnVisibility = ref<VisibilityState>({})
const rowSelection = ref({})
const expanded = ref<ExpandedState>({})

const table = useVueTable({
    data,
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