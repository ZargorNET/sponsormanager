// https://nuxt.com/docs/api/configuration/nuxt-config
// @ts-ignore
export default defineNuxtConfig({
  ssr: false,
  modules: [
    "@huntersofbook/naive-ui-nuxt",
    "@nuxtjs/tailwindcss",
    "nuxt-icon",
    "@vueuse/nuxt",
    [
      "@pinia/nuxt",
      {
        autoImports: ["defineStore", "acceptHMRUpdate"],
      },
    ],
  ],
  imports: {
    // Auto-import pinia stores defined in `~/stores`
    dirs: ["stores"],
  },
});
