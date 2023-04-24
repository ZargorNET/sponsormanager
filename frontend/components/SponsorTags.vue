<template>
  <div class="overflow-auto max-w-full h-8">
    <n-space size="small">
      <n-tag :bordered="false" size="small" v-for="tag in props.tags">
        <NuxtLink :to="`/search/%23${encodeURI(`${tag}`)}`" class="overflow-hidden text-ellipsis">{{ tag }}</NuxtLink>
      </n-tag>
    </n-space>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  tags: String[]
}>();
</script>
