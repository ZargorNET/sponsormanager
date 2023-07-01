<template>
  <div
    class="h-full w-full flex items-center justify-center rounded relative"
    :class="hasLogo() ? 'bg-white' : 'bg-blue-500'"
  >
    <img
      v-if="hasLogo()"
      :alt="`image of ${sponsor.name}`"
      :src="sponsorImage"
      @error="isError = true"
      class="max-w-full max-h-full object-contain"
    />
    <div v-else>
      <n-empty description="No logo" size="huge" />
    </div>
    <div
      class="absolute bg-black opacity-80 w-full h-full flex flex-col justify-center items-center hover:opacity-90 transition-opacity cursor-pointer"
      v-if="edit"
      @click="changeLogo()"
    >
      <div class="mt-10">
        <n-h2>Click here to change</n-h2>
      </div>

      <div
        class="z-10 absolute bottom-2"
        @click="$event.stopPropagation()"
        v-if="hasLogo()"
      >
        <n-button @click="emits('deleteLogo')">Delete</n-button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Sponsor } from "~/utils/sponsor";

const fileDialog = useFileDialog();
const appConfig = useAppConfig();
const emits = defineEmits<{
  (e: "changeLogo", file: File): void;
  (e: "deleteLogo"): void;
}>();
const props = defineProps<{
  sponsor: Sponsor;
  edit?: boolean;
}>();
const sponsorImage = computed(() =>
  (appConfig.apiEndpoint + props.sponsor.imageUrl).replaceAll(
    /(?<!:)\/+/gm,
    "/"
  )
);
const isError = ref(false);

fileDialog.onChange((files) => {
  if (files == null || files.length == 0) return;

  emits("changeLogo", files[0]);
  fileDialog.reset();
});

function changeLogo() {
  fileDialog.open({
    multiple: false,
    accept: "image/png,image/jpeg",
  });
}

function hasLogo(): boolean {
  return props.sponsor.imageUrl !== undefined && !isError.value;
}
</script>
