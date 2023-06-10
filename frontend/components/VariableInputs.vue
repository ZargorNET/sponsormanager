<template>
  <n-input-group v-for="(item, index) in _items" :key="item" class="mt-2">
    <n-input
      :placeholder="placeholder"
      class="mr-2"
      @focus="() => (item.focused = true)"
      @blur="
        () => {
          item.focused = false;
          updateEmit();
        }
      "
      v-model:value="item.value"
    />
    <n-button @click="remove(index)">
      <Icon name="material-symbols:close" />
    </n-button>
  </n-input-group>
</template>

<script setup lang="ts">
import { Ref } from "vue";

const emits = defineEmits(["update:items"]);
const _items: Ref<Array<{ value: string; focused: boolean }>> = ref([
  { value: "", focused: false },
]);

const props = defineProps<{
  items?: string[];
  placeholder: string;
}>();

watch(
  () => props.items,
  (items) => {
    if (items)
      _items.value = items.map((i) => {
        return { value: i, focused: false };
      });
    updateCount();
  }
);

function updateCount() {
  _items.value = _items.value.filter((i) => i.focused || i.value !== "");

  if (newPossible()) _items.value.push({ value: "", focused: false });
}

function remove(index: number) {
  _items.value.splice(index, 1);
  updateEmit();
}

function newPossible(): boolean {
  const valuesList = _items.value;

  return !(
    valuesList.length != 0 && valuesList[valuesList.length - 1].value === ""
  );
}

function updateEmit() {
  updateCount();
  emits(
    "update:items",
    _items.value.map((item) => item.value).filter((value) => value !== "")
  );
}
</script>
