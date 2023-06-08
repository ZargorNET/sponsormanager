<template>
    <div class="h-full w-full flex items-center justify-center bg-blue-500 rounded relative">
        <img v-if="sponsor.imageUrl && !isError" :alt="`image of ${sponsor.name}`" :src="sponsorImage"
             @error="isError = true"
             class="max-w-full max-h-full object-contain"/>
        <div v-else>
            <n-empty description="No logo" size="huge"/>
        </div>
        <div
            class="absolute bg-black opacity-80 w-full h-full flex justify-center items-center hover:opacity-90 transition-opacity cursor-pointer"
            v-if="edit" @click="changeLogo()">
            <div class="mt-10">
                <n-h2>Click here to change</n-h2>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import {Sponsor} from "~/utils/sponsor";

const fileDialog = useFileDialog();
const appConfig = useAppConfig();
const emits = defineEmits<{
    (e: 'changeLogo', file: File): void
}>();
const props = defineProps<{
    sponsor: Sponsor,
    edit?: boolean
}>();
const sponsorImage = computed(() => (appConfig.apiEndpoint + props.sponsor.imageUrl).replaceAll(/(?<!:)\/+/gm, "/"));
const isError = ref(false);

fileDialog.onChange(files => {
    if (files == null || files.length == 0)
        return;

    emits('changeLogo', files[0]);
    fileDialog.reset();
})

function changeLogo() {
    fileDialog.open({
        multiple: false,
        accept: "image/png,image/jpeg"
    });
}




</script>
