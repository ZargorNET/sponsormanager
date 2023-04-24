<template>
  <div>
    <div v-for="sponsor in sponsors" class="w-96">
      <n-card>
        <template #cover>
          <NuxtLink :to="`/sponsor/${sponsor.uid}`">
            <div class="h-72 w-full flex items-center justify-center bg-blue-500">
              <img :src="sponsor.imageUrl" :alt="`image of ${sponsor.name}`" v-if="sponsor.imageUrl"/>
              <div v-else>
                <n-empty size="huge" description="No logo"/>
              </div>
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
              <Icon name="ooui:success" size="1.8em" class="text-green-500" v-if="sponsor.favoursCompleted"/>
              <Icon name="ooui:clear" size="1.8em" class="text-red-500" v-else/>
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
          <n-ellipsis :line-clamp="5">
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
  </div>
</template>

<script setup lang="ts">
import {Sponsor} from "~/utils/sponsor";

const props = defineProps<{
  sponsors: Sponsor[]
}>();

</script>
