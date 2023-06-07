<template>
    <div class="flex flex-1">
        <div class="w-1/2">
            <n-h1 class="text-center">Sponsors</n-h1>
            <SponsorOverviewGrid :sponsors="sponsors"/>
        </div>
        <div class="w-1/2">
            <n-h1 class="text-center">Favours</n-h1>
            <SponsorFavours :favours="favours" :edit="false" :fetch-sponsor="true"/>
        </div>
    </div>
</template>

<script setup lang="ts">

import {Sponsor, SponsorFavour} from "~/utils/sponsor";
import SponsorFavours from "~/components/SponsorFavours.vue";

const route = useRoute();
const term = route.params.term as string;

const sponsors = ref([] as Sponsor[]);
const favours = ref([] as SponsorFavour[]);

onMounted(() => {
    searchSponsors();
    searchFavours();
});

async function searchSponsors() {
    const response = await getHttpClient().get(`/search?type=sponsors&search=${encodeURI(term)}`);
    sponsors.value = await response.data.results as Sponsor[];
}

async function searchFavours() {
    const response = await getHttpClient().get(`/search?type=favours&search=${encodeURI(term)}`);
    favours.value = await response.data.results as SponsorFavour[];
}
</script>
