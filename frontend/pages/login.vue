<template>
  <div class="w-full h-full flex justify-center items-center">
    <div>
      <n-card class="text-center">
        <template #header> Please Login</template>

        <form class="child:mt-2" @submit.prevent="login()">
          <n-input
            type="text"
            title="email"
            placeholder="E-Mail"
            v-model:value="email"
          />
          <n-input
            type="password"
            placeholder="Password"
            v-model:value="password"
            show-password-toggle
          />
          <div>
            <n-button type="primary" attr-type="submit">Send</n-button>
          </div>
        </form>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Ref } from "vue";

const router = useRouter();
const authStore = useAuthStore();
const email: Ref<string> = ref("");
const password: Ref<string> = ref("");

onBeforeMount(async () => {
  if (authStore.user !== null) {
    await router.push("/");
    return;
  }

  const { apiEndpoint } = useAppConfig();
  await navigateTo(`${apiEndpoint}login`, { external: true });
});

async function login() {
  await authStore.login(email.value, password.value);

  if (authStore.user !== null) {
    getNotificationApi().success({ title: "Welcome!", duration: 4000 });
    await router.push("/");
  }
}
</script>
