<template>
  <div></div>
</template>

<script setup lang="ts">
const router = useRouter();
const authStore = useAuthStore();
onBeforeMount(() => {
  authStore.user = {
    uid: "0",
    name: "Zargor",
    sessionToken: "wd",
    season: "2023"
  }
  router.push("/");
});
</script>
