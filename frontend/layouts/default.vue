<template>
  <n-config-provider
      :theme="darkTheme"
      :inline-theme-disabled="true"
      :locale="deDE"
      :date-locale="dateDeDE"
      class="flex basis-full flex-grow"
  >
    <n-loading-bar-provider>
      <n-message-provider placement="top" keep-alive-on-hover>
        <n-notification-provider>
          <n-dialog-provider>
            <nuxt-layout name="custom"/>
          </n-dialog-provider>
        </n-notification-provider>
      </n-message-provider>
    </n-loading-bar-provider>
  </n-config-provider>
</template>

<script setup>
import {darkTheme, dateDeDE, deDE} from "naive-ui"

useHead({
  title: "Sponsored.",
  link: [
    {
      href: "https://fonts.gstatic.com",
      crossorigin: "anonymous",
      rel: "preconnect"
    },
    {
      href: "https://fonts.googleapis.com",
      crossorigin: "anonymous",
      rel: "preconnect"
    },
    {
      href: "https://fonts.googleapis.com/css2?family=Gloock&display=swap",
      rel: "stylesheet"
    }
  ]
});

</script>

<style>
body, html {
  background-color: black;
}
</style>
