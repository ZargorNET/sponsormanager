<template>
  <div class="flex flex-wrap justify-center">
    <div v-for="sponsor in sponsors" class="w-96 m-2">
      <n-card class="h-[470px]">
        <template #cover>
          <NuxtLink :to="`/sponsor/${sponsor.uid}`">
            <div class="h-72 w-96">
              <SponsorImage :sponsor="sponsor" />
            </div>
          </NuxtLink>
        </template>
        <template #header>
          <NuxtLink :to="`/sponsor/${sponsor.uid}`">
            {{ sponsor.name }}
          </NuxtLink>
        </template>
        <template #header-extra>
          <n-tooltip v-if="getStatusColor(sponsor)">
            <template #trigger>
              <div
                class="border-4 rounded-[50%] w-6 h-6 mr-2"
                :style="{borderColor: getStatusColor(sponsor)!.hex}"
              ></div>
            </template>
            Sponsor status is {{ getStatusColor(sponsor)!.name }}
          </n-tooltip>
          <n-tooltip trigger="hover">
            <template #trigger>
              <Icon
                v-if="sponsor.favoursCompleted"
                class="text-green-500"
                name="ooui:success"
                size="1.8em"
              />
              <Icon
                v-else
                class="text-red-500"
                name="ooui:clear"
                size="1.8em"
              />
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
            <SponsorTags :tags="sponsor.tags" />
          </div>
        </template>
      </n-card>
    </div>
    <div v-if="sponsors.length === 0" class="flex justify-center w-full">
      <n-empty description="No sponsors found" size="huge" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Sponsor } from "~/utils/sponsor";
import SponsorImage from "~/components/SponsorImage.vue";

const props = defineProps<{
  sponsors: Sponsor[];
}>();

function getStatusColor(
  sponsor: Sponsor
): { hex: string; name: string } | undefined {
  for (let tag of sponsor.tags) {
    const t = tag.toLowerCase();
    switch (t) {
      case "platinum":
        return { hex: "#B9F2FF", name: "Platinum" };
      case "gold":
        return { hex: "#ffd700", name: "Gold" };
      case "silver":
        return { hex: "#C0C0C0", name: "Silver" };
      case "bronze":
        return { hex: "#70380f", name: "Bronze" };
    }
  }

  return undefined;
}
</script>
