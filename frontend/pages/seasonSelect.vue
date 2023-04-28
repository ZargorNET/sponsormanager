<template>
    <div class="flex flex-col items-center">
        <div class="w-2/3">
            <n-card>
                <template #header>
                    <n-h1>
                        Season Select
                    </n-h1>
                </template>

                Select the season you want to see

                <n-select v-model:value="value" :options="options"/>
                <div class="mt-2">
                    <n-button type="primary">Save</n-button>
                </div>
            </n-card>
        </div>
    </div>
</template>

<script setup lang="ts">

const mainStore = useMainStore();

const value = ref();
const options = computed(() => mainStore.seasons.map(season => {
    return {label: season, value: season};
}));


</script>
