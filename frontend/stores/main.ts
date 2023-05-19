import {Sponsor, SponsorFavour} from "~/utils/sponsor";
import {getHttpClient} from "~/utils/http";
import {Ref} from "vue";
import {Settings} from "~/utils/settings";

export const useMainStore = defineStore('main', () => {
    //  const seasons: Ref<Array<string>> = ref(["2023", "2022"]);
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

    fetchSettings();
    fetchAllSponsors();

    return {fetchAllSponsors, getAllFavours, fetchSettings, settings, sponsors};

});

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useMainStore, import.meta.hot))
}
