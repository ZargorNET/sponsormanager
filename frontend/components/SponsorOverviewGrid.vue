<template>
    <div class="flex flex-wrap justify-center">
        <div v-for="sponsor in sponsors" class="w-96 m-2">
            <n-card class="h-[470px]">
                <template #cover>
                    <NuxtLink :to="`/sponsor/${sponsor.uid}`">
                        <div class="h-72 w-96">
                            <SponsorImage :sponsor="sponsor"/>
                        </div>
                    </NuxtLink>
                </template>
                <template #header>
                    <NuxtLink :to="`/sponsor/${sponsor.uid}`">
                        {{ sponsor.name }}
                    </NuxtLink>
                </template>
                <template #header-extra>
                    <n-tooltip trigger="hover">
                        <template #trigger>
                            <Icon v-if="sponsor.favoursCompleted" class="text-green-500" name="ooui:success"
                                  size="1.8em"/>
                            <Icon v-else class="text-red-500" name="ooui:clear" size="1.8em"/>
                        </template>
                        <template #default>
                            <template v-if="sponsor.favoursCompleted">
                                All return favours done! Good job!
                            </template>
                            <template v-else>
                                Some favours have not yet been addressed.
                            </template>
                        </template>
                    </n-tooltip>
                </template>
                <template #default>
                    <n-ellipsis :line-clamp="2">
                        {{ sponsor.shortDescription }}
                    </n-ellipsis>
                </template>
                <template #footer>
                    <div class="w-full">
                        <SponsorTags :tags="sponsor.tags"/>
                    </div>
                </template>
            </n-card>
        </div>
        <div v-if="sponsors.length === 0" class="flex justify-center w-full">
            <n-empty description="No sponsors found" size="huge"/>
        </div>
    </div>
</template>

<script lang="ts" setup>
import {Sponsor} from "~/utils/sponsor";
import SponsorImage from "~/components/SponsorImage.vue";

const props = defineProps<{
    sponsors: Sponsor[]
}>();

</script>
