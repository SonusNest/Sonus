<template>
    <Dialog v-model:open="aboutDialogOpen">
        <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
                <DialogTitle>Souns</DialogTitle>
                <DialogDescription>
                    Private music player
                </DialogDescription>
            </DialogHeader>
            <div class="grid gap-4 py-4">
                <div class="flex flex-col items-center">
                    <img src="/icon.png" alt="logo" class="w-16 h-16" />
                    <div class="text-2xl font-bold">Sonus v0.0.1</div>
                </div>
                <div class="text-sm text-muted-foreground font-bold">
                    Disclaimers
                </div>
                <div class="text-sm text-muted-foreground">
                    This software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.
                </div>
            </div>
        </DialogContent>
    </Dialog>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useAppStore } from "@/stores/appStore"
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogDescription,
} from "@/components/ui/dialog"

const appStore = useAppStore()

const aboutDialogOpen = computed({
    get() {
        return appStore.getAboutDialogState
    },
    set(value) {
        appStore.setAboutDialogState(value)
    }
})
</script>

<style scoped>

</style>