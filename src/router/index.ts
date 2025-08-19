import { createRouter, createMemoryHistory } from "vue-router"

const routes = [
    {
        path: '/',
        name: 'Main',
        component: () => import('@/views/MainView.vue'),
        children: [
            {
                path: '/home',
                name: 'Home',
                component: () => import('@/views/pages/HomePage.vue')
            },
            {
                path: '/album',
                name: 'Album',
                component: () => import('@/views/pages/AlbumPage.vue')
            },
            {
                path: '/artist',
                name: 'Artist',
                component: () => import('@/views/pages/ArtistPage.vue')
            },
            {
                path: '/favorite-songs',
                name: 'FavoriteSongs',
                component: () => import('@/views/pages/FavoriteSongsPage.vue')
            },
            {
                path: '/search',
                name: 'Search',
                component: () => import('@/views/pages/SearchPage.vue')
            },
            {
                path: '/songs',
                name: 'Songs',
                component: () => import('@/views/pages/SongsPage.vue')
            },
            {
                path: '/playlist',
                name: 'Playlist',
                component: () => import('@/views/pages/PlaylistPage.vue')
            },
            {
                path: '/recently-played',
                name: 'RecentlyPlayed',
                component: () => import('@/views/pages/RecentlyPlayedPage.vue')
            }
        ]
    },
    {
        path: '/oobe',
        name: 'OOBE',
        component: () => import('@/views/pages/OOBEPage.vue')
    }
]

const router = createRouter({
    history: createMemoryHistory(),
    routes
})

export default router