import {Ref} from "vue";
import {getHttpClient} from "~/utils/http";

export const useAuthStore = defineStore('user', () => {
    const user: Ref<Auth | null> = ref(null);
    const sessionCookie = useCookie("session", {secure: import.meta.env.PROD, expires: getSessionCookieExpiry()});

    async function fetchUser() {
        if (user.value !== null)
            return;

        const response = await getHttpClient(false, false).get("/whoami");
        if (!response)
            return;

        user.value = await response.data;
    }

    async function login(email: string, password: string) {
        const res = await getHttpClient().post("/login", {email: email, password: password});
        const data = await res.data;

        sessionCookie.value = data.token;
        await fetchUser();
    }

    function logout() {
        user.value = null;
        sessionCookie.value = null;
    }


    return {user, fetchUser, login, logout, sessionCookie};
});

export interface Auth {
    sub: String,
    email: String,
    exp: number,
    dn: String,
}

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useAuthStore, import.meta.hot))
}

function getSessionCookieExpiry(): Date {
    const date = new Date();
    date.setDate(date.getDate() + 31);

    return date;
}
