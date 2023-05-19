import {Ref} from "vue";

export const useAuthStore = defineStore('user', () => {
    const user: Ref<Auth | null> = ref(null);


    const sessionCookie = useCookie("session");

    async function fetchUser() {
        const response = await getHttpClient().get("/whoami");
        response.data
        user.value = {...response.data, sessionToken: "11111"};
    }


    //loginBySessionCookie();
    fetchUser();

    return {user, fetchUser};
});

export interface Auth {
    name: String,
    email: String,
    sessionToken: String,
}

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useAuthStore, import.meta.hot))
}
