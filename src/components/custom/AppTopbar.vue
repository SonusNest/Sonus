<template>
    <div class="flex flex-row items-center">
        <MacOSControl v-if="currentPlatform === 'macos'" class="ml-4" />
        <div class="h-12 w-full flex flex-row items-center ml-1">
            <div class="gap-1 flex flex-row">
                <DropdownMenu>
                    <DropdownMenuTrigger>
                        <Button variant="ghost">
                            <Ellipsis />
                        </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent class="bg-popover/80">
<!--                        <label class="text-xs font-medium ml-2 text-foreground/80">Online Music</label>-->
<!--                        <DropdownMenuItem @click="toast.warning('Sorry, this feature is not implemented yet.')">-->
<!--                            <svg t="1754584485409" class="icon" viewBox="0 0 1024 1024" version="1.1"-->
<!--                                xmlns="http://www.w3.org/2000/svg" p-id="7706" width="200" height="200"-->
<!--                                fill="currentColor">-->
<!--                                <path-->
<!--                                    d="M783.872 121.6l0.725333 8.277333A468.736 468.736 0 0 1 981.333333 512c0 259.2-210.133333 469.333333-469.333333 469.333333S42.666667 771.2 42.666667 512 252.8 42.666667 512 42.666667c64.341333 0 125.653333 12.928 181.504 36.394666-75.178667 35.541333-135.381333 51.285333-290.474667 64.512-56.789333 2.304-69.248 26.538667-37.333333 72.746667l4.053333 5.632 195.541334 338.773333A236.245333 236.245333 0 0 0 512 554.666667c-117.802667 0-213.333333 85.973333-213.333333 192S394.197333 938.666667 512 938.666667s213.333333-85.973333 213.333333-192c0-25.770667-5.632-50.389333-15.829333-72.832l-4.650667-9.514667-1.749333-3.114667-1.621333-2.858666-1.706667-2.858667L512 328.661333l15.872-10.538666c99.072-67.370667 169.386667-139.008 210.944-214.869334 18.56-38.698667 28.288-60.416 29.098667-65.194666 0.938667-5.12 10.666667 23.338667 15.957333 83.541333z"-->
<!--                                    p-id="7707"></path>-->
<!--                            </svg>-->
<!--                            <span>Log in QQ Music Account</span>-->
<!--                        </DropdownMenuItem>-->
<!--                        <DropdownMenuItem @click="toast.warning('Sorry, this feature is not implemented yet.')">-->
<!--                            <svg t="1754584512212" class="icon" viewBox="0 0 1024 1024" version="1.1"-->
<!--                                xmlns="http://www.w3.org/2000/svg" p-id="9563" width="200" height="200"-->
<!--                                fill="currentColor">-->

<!--                                <path-->
<!--                                    d="M512 0a512 512 0 1 1-512 512 512 512 0 0 1 512-512z m87.552 185.152c-55.296 18.304-81.856 69.76-65.92 128.32l5.632 21.568c-11.968 2.56-23.68 6.08-35.136 10.432-54.016 20.864-96.896 67.008-111.872 120.576a163.968 163.968 0 0 0-5.44 58.688 147.136 147.136 0 0 0 61.44 107.328 129.664 129.664 0 0 0 110.08 18.752 129.024 129.024 0 0 0 67.968-45.44c26.368-34.368 34.176-79.68 22.144-127.424-4.48-17.408-9.92-36.672-15.36-55.36l-5.76-20.288a140.8 140.8 0 0 1 61.312 33.152c59.712 55.744 71.232 151.872 26.752 223.488-39.04 62.976-115.136 103.552-193.792 103.552a240.832 240.832 0 0 1-240.576-240.512 236.416 236.416 0 0 1 62.592-160c24.128-26.304 53.76-46.912 86.848-60.16a33.856 33.856 0 0 0-25.088-62.912A308.8 308.8 0 0 0 271.36 343.68a305.664 305.664 0 0 0-57.92 178.304 308.48 308.48 0 0 0 308.224 308.16c101.568 0 200.192-53.12 251.264-135.552 62.208-100.096 46.08-229.952-38.08-308.608a212.032 212.032 0 0 0-126.72-55.232c-2.432-9.6-6.272-24.128-9.216-35.008a47.616 47.616 0 0 1-0.896-25.408 33.92 33.92 0 0 1 40.96-22.848 37.76 37.76 0 0 1 13.056 6.4c4.096 3.072 7.296 7.04 11.072 10.56a33.92 33.92 0 0 0 53.12-40.128l-0.64-1.152a69.76 69.76 0 0 0-15.36-17.536 107.2 107.2 0 0 0-56.96-24.96 102.528 102.528 0 0 0-43.648 4.48z m-42.88 215.36c3.584 13.248 7.488 27.072 11.456 40.704 5.248 18.432 10.56 36.736 14.72 53.248 4.864 19.392 7.04 47.168-10.24 69.888a60.8 60.8 0 0 1-32.32 21.248 62.336 62.336 0 0 1-53.504-9.152 78.848 78.848 0 0 1-32.512-57.536c-1.088-11.584 0-23.296 3.136-34.56 9.344-33.472 36.544-62.4 71.04-75.712 9.28-3.584 18.688-6.272 28.16-8.128z"-->
<!--                                    p-id="9564"></path>-->
<!--                            </svg>-->
<!--                            <span>Log in NetEase Cloud Music Account</span>-->
<!--                        </DropdownMenuItem>-->
<!--                        <DropdownMenuItem @click="toast.warning('Sorry, this feature is not implemented yet.')">-->
<!--                            <svg t="1754584556689" class="icon" viewBox="0 0 1024 1024" version="1.1"-->
<!--                                xmlns="http://www.w3.org/2000/svg" p-id="11623" width="200" height="200"-->
<!--                                fill="currentColor">-->

<!--                                <path-->
<!--                                    d="M1023.872 261.290667a393.813333 393.813333 0 0 0-10.24-93.44c-13.525333-55.893333-45.312-98.56-93.013333-129.834667A214.272 214.272 0 0 0 840.533333 6.997333a447.872 447.872 0 0 0-66.730666-6.4c-1.706667-0.128-3.541333-0.426667-5.290667-0.554666H255.488c-6.485333 0.426667-12.928 0.725333-19.413333 1.109333C204.202667 2.986667 172.501333 6.4 142.506667 18.261333 85.504 40.874667 44.373333 80.213333 20.266667 136.874667c-8.192 19.114667-12.458667 39.466667-15.488 60.074666a452.693333 452.693333 0 0 0-4.266667 50.346667c0 1.365333-0.298667 2.645333-0.426667 3.968v521.514667c0.426667 5.973333 0.725333 12.074667 1.152 18.090666 2.133333 34.773333 6.570667 69.290667 21.205334 101.248 27.733333 60.586667 74.154667 100.394667 137.984 119.552 17.92 5.418667 36.522667 7.978667 55.168 9.728 23.68 2.261333 47.36 2.56 71.125333 2.56h470.613333c22.4 0 44.714667-1.450667 66.986667-4.266666 35.114667-4.522667 68.138667-14.933333 97.962667-34.56a215.296 215.296 0 0 0 80.213333-94.208c7.936-17.92 12.501333-37.12 15.786667-56.490667 4.821333-28.8 5.888-57.941333 5.845333-87.04-0.085333-162.133333 0-324.053333-0.128-486.101333z m-274.048 170.24v243.712c0 17.792-2.474667 35.285333-10.410667 51.456-12.373333 25.173333-32.426667 41.045333-59.221333 48.64-14.933333 4.266667-30.122667 6.698667-45.653333 7.381333-40.533333 1.92-75.648-25.6-82.901334-65.536a80.213333 80.213333 0 0 1 44.288-86.272c13.781333-6.826667 28.586667-10.666667 43.434667-13.824 16.128-3.498667 32.341333-6.528 48.384-10.24 11.690667-2.688 19.498667-9.813333 21.76-22.016a38.570667 38.570667 0 0 0 0.853333-8.234667c0-77.44 0-154.88-0.085333-232.234666a30.933333 30.933333 0 0 0-1.109333-7.893334c-1.706667-6.4-6.4-10.368-12.970667-9.984-6.826667 0.426667-13.568 1.493333-20.266667 2.816-32.426667 6.4-64.853333 12.928-97.28 19.456l-99.242666 20.053334-58.624 11.861333c-0.682667 0.128-1.365333 0.426667-2.048 0.554667-11.818667 3.285333-16.085333 8.661333-16.64 20.906666-0.085333 1.792 0 3.669333 0 5.546667-0.085333 111.018667 0 222.037333-0.128 333.013333 0 17.92-2.005333 35.669333-9.173334 52.352-11.861333 27.306667-32.853333 44.373333-61.184 52.608-14.933333 4.266667-30.293333 6.826667-45.866666 7.338667-40.96 1.536-74.88-25.6-81.92-65.877333-5.973333-34.645333 9.813333-71.893333 49.237333-88.533334 15.232-6.4 31.146667-9.898667 47.274667-13.226666 12.245333-2.56 24.533333-4.949333 36.693333-7.552 16.341333-3.541333 24.874667-13.781333 25.6-30.464v-6.4c0-126.293333 0-252.672 0.085333-378.965334 0-5.248 0.554667-10.666667 1.792-15.786666 2.986667-12.16 11.648-19.114667 23.296-22.101334 10.88-2.816 21.973333-4.778667 33.024-7.04 31.274667-6.4 62.549333-12.629333 93.866667-18.944l96.853333-19.626666c28.586667-5.717333 57.173333-11.52 85.76-17.194667 9.386667-1.834667 18.901333-3.754667 28.330667-4.522667 13.226667-1.066667 22.314667 7.253333 23.637333 20.565334 0.341333 3.114667 0.512 6.314667 0.512 9.514666 0.085333 81.493333 0.085333 163.072 0 244.565334z"-->
<!--                                    p-id="11624"></path>-->
<!--                            </svg>-->
<!--                            <span>Log in Apple Music Account</span>-->
<!--                        </DropdownMenuItem>-->
<!--                        <DropdownMenuSeparator />-->
                        <label class="text-xs font-medium ml-2 text-foreground/80">Private Music</label>
                        <DropdownMenuItem @click="privateMusicDialogOpenFunc('local')">
                            <FolderSearch />
                            <span>Scan local folder</span>
                        </DropdownMenuItem>
                        <DropdownMenuItem @click="privateMusicDialogOpenFunc('webdav')">
                            <Earth />
                            <span>Connect remote WebDAV</span>
                        </DropdownMenuItem>
                        <DropdownMenuItem @click="privateMusicDialogOpenFunc('nas')">
                            <Server />
                            <span>Connect NAS server</span>
                        </DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <label class="text-xs font-medium ml-2 text-foreground/80">Preferences</label>
                        <DropdownMenuItem @click="appStore.setSettingsDialogState(true)">
                            <Settings />
                            <span>Settings</span>
                        </DropdownMenuItem>
                        <DropdownMenuItem @click="appStore.setDeveloperOptionsDialogState(true)">
                            <CodeXml />
                            <span>Developer options</span>
                        </DropdownMenuItem>
                        <DropdownMenuItem @click="appStore.setAboutDialogState(true)">
                            <Info />
                            <span>About</span>
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
                <Button variant="ghost">
                    <ChevronLeft />
                </Button>
                <Button variant="ghost">
                    <ChevronRight />
                </Button>
                <Button variant="ghost" @click="sidebarOpen">
                    <PanelLeftClose />
                </Button>
<!--                <DropdownMenu>-->
<!--                  <DropdownMenuTrigger>-->
<!--                    <Button variant="ghost">-->
<!--                      <Hammer />-->
<!--                    </Button>-->
<!--                  </DropdownMenuTrigger>-->
<!--                  <DialogContent>-->
<!--                    <Card>-->
<!--                      <CardHeader>-->
<!--                        <CardTitle>-->
<!--                          Task Queue-->
<!--                        </CardTitle>-->
<!--                        <CardDescription>-->
<!--                          Task Queue-->
<!--                        </CardDescription>-->
<!--                      </CardHeader>-->
<!--                        <CardContent>-->
<!--                          None-->
<!--                        </CardContent>-->
<!--                    </Card>-->
<!--                  </DialogContent>-->
<!--                </DropdownMenu>-->
            </div>
            <div class="flex-1 w-full h-full" data-tauri-drag-region>
            </div>
            <div>
                <div class="relative w-100 items-center">
                    <Input id="search" type="text" placeholder="Songs, Artists, Albums and More" class="pl-3 pr-8" />
                    <span class="absolute end-0 inset-y-0 flex items-center justify-center px-2">
                        <Search class="size-4 text-muted-foreground" />
                    </span>
                </div>
            </div>
            <div class="flex-1 w-full h-full" data-tauri-drag-region>
            </div>
            <div class="gap-1 flex flex-row mr-1">
                <Button variant="ghost">
                    <MessageSquareQuote />
                </Button>
                <Button variant="ghost">
                    <ListMusic />
                </Button>
            </div>
        </div>
        <Suspense>
            <WindowsControl v-if="currentPlatform === 'windows' || currentPlatform === 'linux'" />
        </Suspense>
        <!-- Dialog -->
        <Dialog v-model:open="privateMusicDialogOpen">
            <DialogContent class="sm:max-w-[800px]">
                <DialogHeader>
                    <DialogTitle>Private Music</DialogTitle>
                    <DialogDescription>
                        Scan local folder or connect remote WebDAV or NAS server to private music.
                    </DialogDescription>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                    <Tabs :default-value="privateMusicTab" class="w-full">
                        <TabsList class="grid w-full grid-cols-3">
                            <TabsTrigger value="local">
                                Local Folder
                            </TabsTrigger>
                            <TabsTrigger value="webdav">
                                WebDAV
                            </TabsTrigger>
                            <TabsTrigger value="nas">
                                NAS
                            </TabsTrigger>
                        </TabsList>
                        <TabsContent value="local">
                            <Card>
                                <CardContent class="space-y-2 flex flex-row items-center gap-2">
                                    <Input type="text" class="m-0" placeholder="Click right button to select folder" />
                                    <Button variant="outline">
                                        <FolderPlus />
                                    </Button>
                                </CardContent>
                                <CardFooter class="flex justify-between items-center">

                                    <p>{{ }}</p>
                                    <Button variant="outline">Scan</Button>
                                </CardFooter>
                            </Card>
                        </TabsContent>
                        <TabsContent value="webdav">
                            <Card>
                                <CardContent class="space-y-2 flex flex-col items-center gap-2">
                                    <Input type="text" class="m-0" placeholder="Enter your WebDAV server URL" />
                                    <Input type="text" class="m-0" placeholder="Enter your WebDAV username" />
                                    <Input type="text" class="m-0" placeholder="Enter your WebDAV password" />
                                    <Input type="text" class="m-0" placeholder="Enter your WebDAV mount point path" />
                                </CardContent>
                                <CardFooter class="flex justify-between items-center">

                                    <p>{{ }}</p>
                                    <Button variant="outline">Connect</Button>
                                </CardFooter>
                            </Card>
                        </TabsContent>
                        <TabsContent value="nas">
                            <Card>
                                <CardContent class="space-y-2 flex flex-col items-center gap-2">
                                    <Alert>
                                        <AlertTitle>Tips:</AlertTitle>
                                        <AlertDescription>
                                            NAS server must support SMB protocol. If your NAS does not support SMB
                                            protocol, please turn on the WebDAV function of NAS and use WebDAV to
                                            connect.
                                        </AlertDescription>
                                    </Alert>
                                    <Input type="text" class="m-0" placeholder="Enter your NAS server URL" />
                                    <Input type="text" class="m-0" placeholder="Enter your NAS username" />
                                    <Input type="text" class="m-0" placeholder="Enter your NAS password" />
                                    <Input type="text" class="m-0" placeholder="Enter your NAS mount point path" />
                                </CardContent>
                                <CardFooter class="flex justify-between items-center">

                                    <p>{{ }}</p>
                                    <Button variant="outline">Connect</Button>
                                </CardFooter>
                            </Card>
                        </TabsContent>
                    </Tabs>
                </div>
            </DialogContent>
        </Dialog>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Button } from '@/components/ui/button'
import {
    Ellipsis,
    ChevronLeft,
    ChevronRight,
    PanelLeftClose,
    Search,
    MessageSquareQuote,
    ListMusic,
    Settings,
    Info,
    CodeXml,
    FolderSearch,
    Earth,
    Server,
    FolderPlus,
    Hammer
} from 'lucide-vue-next'
import { Input } from '@/components/ui/input'
import WindowsControl from '@/components/custom/WindowsControl.vue'
import MacOSControl from '@/components/custom/MacOSControl.vue'
// @ts-ignore
import { platform } from '@tauri-apps/plugin-os'
import { toast } from 'vue-sonner'
import { useAppStore } from '@/stores/appStore'
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/dialog'
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui/card'
import { Alert, AlertTitle, AlertDescription } from '@/components/ui/alert'

const appStore = useAppStore()
const privateMusicDialogOpen = ref(false)
const currentPlatform = platform();

const privateMusicTab = ref('local')
const privateMusicDialogOpenFunc = (tab: string) => {
    privateMusicTab.value = tab
    privateMusicDialogOpen.value = true
}


const sidebarOpen = () => {
    appStore.setSidebarState(!appStore.getSidebarState)
}

</script>

<style scoped></style>