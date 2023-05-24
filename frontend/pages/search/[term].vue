<template>
    <div>
        {{ term }}
        <br/>
        <br/>
        {{ sponsors }}
        <br/>
        <br/>
        {{ favours }}
    </div>
</template>

<script setup lang="ts">
import {SearchSponsor} from "~/utils/search";
import {SponsorFavour} from "~/utils/sponsor";

const route = useRoute();
const term = route.params.term;

const sponsors = ref([] as SearchSponsor[]);
const favours = ref([] as SponsorFavour[]);

onMounted(() => {
    searchSponsors();
    searchFavours();
});

async function searchSponsors() {
    const response = await getHttpClient().get(`/search?type=sponsors&search=${encodeURI(term)}`);
    sponsors.value = await response.data.results as SearchSponsor[];
}

async function searchFavours() {
    const response = await getHttpClient().get(`/search?type=favours&search=${encodeURI(term)}`);
    favours.value = await response.data.results as SponsorFavour[];
}
</script>
