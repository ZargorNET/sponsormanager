<template>
  <div class="flex flex-col items-center">
    <div class="w-2/3">
      <n-card>
        <n-h1> Open favours</n-h1>
        Here is a list of all open favours that need to be addressed.
      </n-card>
      <div class="h-8"></div>
      <SponsorFavours
        :edit="false"
        :favours="openFavours"
        :fetch-sponsor="true"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ComputedRef } from "vue";
import { SponsorFavour } from "~/utils/sponsor";

const mainStore = useMainStore();
const openFavours: ComputedRef<SponsorFavour[]> = computed(() =>
  mainStore.getAllFavours().filter((favour) => !favour.completed)
);

onMounted(async () => {
  await mainStore.fetchAllSponsors();
});
</script>
