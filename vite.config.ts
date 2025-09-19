import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': '/src', // 将 @ 别名指向项目根目录下的 src 文件夹
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },

    // 虚拟代理，解决跨域问题（建议在后端解决跨域问题）
    proxy: {
      '/api-ziyi': {
        target: 'http://api.ziyi.site',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api-ziyi/, '')
      },
      // '/api-youName': {
      //   target: '基地址',
      //   changeOrigin: true,
      //   rewrite: (path) => path.replace(/^\/api-youName/, '')
      // }
    }
  },
}));
