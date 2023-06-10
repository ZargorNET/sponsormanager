<template>
  <div class="flex flex-grid justify-center">
    <div class="w-1/2">
      <n-card>
        <template #header>
          <n-h1>Settings</n-h1>
        </template>

        <n-h2> Mandatory Fields</n-h2>
        <n-p>Fields that are required on each new sponsor entry</n-p>
        <VariableInputs
          v-model:items="mainStore.settings.mandatoryFields"
          placeholder="Field Name"
        />
        <div class="mt-2">
          <n-button type="primary" @click="save()">Save</n-button>
        </div>
        <n-divider />
        <n-h2> Statistics</n-h2>
        <n-p>Nothing here yet...</n-p>
      </n-card>
    </div>
  </div>
</template>

<script lang="ts" setup>
import VariableInputs from "~/components/VariableInputs.vue";
import { getNotificationApi } from "~/utils/misc";

const mainStore = useMainStore();

onMounted(async () => {
  await mainStore.fetchSettings();
});

async function save() {
  await mainStore.saveSettings();
  getNotificationApi().success({ title: "Settings saved!" });
}
</script>
