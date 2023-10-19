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
        <n-h2>Admins</n-h2>
        <n-p>Email Addresses from admins</n-p>
        <VariableInputs
          placeholder="c.slipsager@greenbear.berlin"
          v-model:items="admins"
        />
        <div class="mt-2">
          <n-button type="primary" @click="saveAdmins()">Save</n-button>
        </div>
      </n-card>
    </div>
  </div>
</template>

<script lang="ts" setup>
import VariableInputs from "~/components/VariableInputs.vue";
import { getNotificationApi } from "~/utils/misc";
import { getHttpClient } from "~/utils/http";

const mainStore = useMainStore();

const admins = ref();

onMounted(async () => {
  await mainStore.fetchSettings();

  admins.value = (await getHttpClient().get("/settings/admins")).data;
});

async function save() {
  await mainStore.saveSettings();
  getNotificationApi().success({ title: "Settings saved!" });
}

const emailRegex = /\S+@\S+\.\S{2,}/;
async function saveAdmins() {
  for (const admin of toRaw(admins.value)) {
    if (!emailRegex.test(admin)) {
      getNotificationApi().error({
        title: "Invalid Email",
        description: `Address: ${admin}`,
        duration: 4000,
      });
      return;
    }
  }

  await getHttpClient().post("/settings/admins/update", {
    admins: admins.value,
  });
  getNotificationApi().success({ title: "Admins saved!" });
}
</script>
