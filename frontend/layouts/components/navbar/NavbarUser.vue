<template>
  <div>
    <n-dropdown trigger="click" :options="loggedInDropdownOptions">
      <div class="flex items-center cursor-pointer">
        <div class="rounded-full p-2 bg-black">
          <Icon name="ph:user-fill" size="2em" />
        </div>
        <div class="ml-2 font-bold">
          {{ username }}
        </div>
      </div>
    </n-dropdown>
  </div>
</template>

<script setup lang="ts">
const router = useRouter();
const authStore = useAuthStore();
const username = computed(() => authStore.user?.sub ?? "");

const loggedInDropdownOptions = ref([
  /*    {
        label: `Season: ${authStore.user?.season ?? '----'}`,
        key: "season",
        props: {
            onClick: () => {
                router.push("/seasonselect");
            }
        }
    },*/
  {
    label: "Logout",
    key: "logout",
    props: {
      class: "bg-red-500",
      onClick: logout,
    },
  },
]);

async function logout() {
  authStore.logout();
  getNotificationApi().success({
    title: "Successfully logged out!",
    duration: 4000,
  });
  await router.push("/login");
}
</script>
