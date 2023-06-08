export default defineNuxtRouteMiddleware(async (to, from) => {
    const authStore = useAuthStore();

    await authStore.fetchUser();

    if (to.path !== "/login" && authStore.user === null)
        return navigateTo("/login");

    if (to.path === "login" && authStore.user !== null)
        return abortNavigation();
});
