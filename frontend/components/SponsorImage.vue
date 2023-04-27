<template>
    <div class="h-full w-full flex items-center justify-center bg-blue-500 rounded">
        <img v-if="sponsor.imageUrl" :alt="`image of ${sponsor.name}`" :src="sponsor.imageUrl"/>
        <div v-else>
            <n-empty description="No logo" size="huge"/>
        </div>
    </div>
</template>

<script lang="ts" setup>
import {Sponsor} from "~/utils/sponsor";

defineProps<{
    sponsor: Sponsor
}>();
</script>
