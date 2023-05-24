import {Sponsor, SponsorFavour} from "~/utils/sponsor";
import {getHttpClient} from "~/utils/http";
import {Ref} from "vue";
import {Settings} from "~/utils/settings";

export const useMainStore = defineStore('main', () => {
    const sponsors: Ref<Sponsor[]> = ref([]);
    const settings: Ref<Settings> = ref({} as Settings);

    async function fetchAllSponsors() {
        const res = await getHttpClient().get("/get_all");
        sponsors.value = await res.data;
    }

    async function fetchSettings() {
        const res = await getHttpClient().get("/settings/get");
        settings.value = await res.data;
    }

    function getAllFavours(): SponsorFavour[] {
        return sponsors.value.map((sponsor) => sponsor.favours).flat();
    }

    async function saveSettings() {
        const res = await getHttpClient().post("/settings/update", settings.value);
        settings.value = await res.data;
    }

    return {fetchAllSponsors, getAllFavours, fetchSettings, saveSettings, settings, sponsors};

});

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useMainStore, import.meta.hot))
}
