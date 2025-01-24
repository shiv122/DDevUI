export default defineNuxtConfig({
  modules: [
    "@vueuse/nuxt",
    "@nuxt/fonts",
    "nuxt-svgo",
    "@nuxt/eslint",
    "@nuxtjs/tailwindcss",
    "shadcn-nuxt",
    "@nuxtjs/color-mode",
    "@pinia/nuxt",
  ],
  components: {
    dirs: [
      {
        path: "@/components/ui",
      },
      {
        path: "@/components/custom",
      },
    ],
  },
  tailwindcss: {
    exposeConfig: true,
    configPath: "tailwind.config.js",
    viewer: true,
    cssPath: ["@/assets/css/tailwind.css", { injectPosition: "first" }],
  },
  colorMode: {
    preference: "system", // default value of $colorMode.preference
    fallback: "light", // fallback value if not system preference found
    hid: "nuxt-color-mode-script",
    globalName: "__NUXT_COLOR_MODE__",
    componentName: "ColorScheme",
    classPrefix: "",
    classSuffix: "",
    storage: "localStorage", // or 'sessionStorage' or 'cookie'
    storageKey: "nuxt-color-mode",
  },
  app: {
    head: {
      title: "Nuxtor",
      charset: "utf-8",
      viewport: "width=device-width, initial-scale=1",
      meta: [{ name: "format-detection", content: "no" }],
    },
    pageTransition: {
      name: "page",
      mode: "out-in",
    },
    layoutTransition: {
      name: "layout",
      mode: "out-in",
    },
  },
  css: [],
  fonts: {
    defaults: {
      weights: [`${100}..${900}`],
    },
  },

  svgo: {
    autoImportPath: "@/assets/",
  },
  ssr: false,
  dir: {
    modules: "app/modules",
  },
  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
      hmr: {
        protocol: "ws",
        host: "0.0.0.0",
        port: 3001,
      },
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
  },
  devServer: {
    host: "0.0.0.0",
  },
  eslint: {
    config: {
      standalone: false,
    },
  },
  experimental: {
    typedPages: true,
  },
  future: {
    compatibilityVersion: 4,
  },
  compatibilityDate: "2025-01-01",
});
