<template>
  <div>
    <n-input
      placeholder="Suche"
      round
      clearable
      :attr-size="25"
      v-model:value="searchTerm"
      @keydown.enter="onSearch"
    >
      <template #prefix>
        <Icon name="material-symbols:search" size="1.5em" />
      </template>
    </n-input>
  </div>
</template>

<script setup lang="ts">
import { Ref } from "vue";

const searchTerm: Ref<String> = ref("");
const router = useRouter();
const route = useRoute();

function onSearch() {
  if (searchTerm.value.length == 0) return;

  router.push(`/search/${searchTerm.value}`);
}

onMounted(() => {
  if (route.path.startsWith("/search/"))
    searchTerm.value = route.params.term as string;
});
</script>
