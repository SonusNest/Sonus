import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'node:path'
import tailwindcss from '@tailwindcss/vite'
import wasm from "vite-plugin-wasm"
import topLevelAwait from "vite-plugin-top-level-await"

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [vue(), tailwindcss(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 5174,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  assetsInclude: ['**/*.ttml'],
}));
