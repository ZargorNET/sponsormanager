<template>
    <div class="w-full h-full flex flex-col">
        <SponsorOverviewGrid :sponsors="mainStore.sponsors"/>
    </div>
</template>

<script lang="ts" setup>
import SponsorOverviewGrid from "~/components/SponsorOverviewGrid.vue";

const mainStore = useMainStore();
onMounted(async () => {
    await mainStore.fetchAllSponsors();
});
</script>
