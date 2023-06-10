import { Sponsor, SponsorFavour } from "~/utils/sponsor";
import { getHttpClient } from "~/utils/http";
import { Ref } from "vue";
import { Settings } from "~/utils/settings";

export const useMainStore = defineStore("main", () => {
  const sponsors: Ref<Sponsor[]> = ref([]);
  const settings: Ref<Settings> = ref({} as Settings);

  let _lastFetchAllSponsor = new Date(0);

  async function fetchAllSponsors() {
    const now = new Date();
    if (_lastFetchAllSponsor.getTime() + 4000 > now.getTime()) return;
    _lastFetchAllSponsor = now;

    console.debug("fetching all sponsors...");
    const res = await getHttpClient().get("/get_all");
    sponsors.value = await res.data;
  }

  async function fetchSingleSponsor(uid: string) {
    const res = await getHttpClient(false).get(`/get/${uid}`);
    const data = (await res.data) as Sponsor;
    _replaceSponsor(data);
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

  async function createOrUpdateSponsor(
    sponsor: Sponsor,
    update: boolean,
    logo: File | undefined
  ): Promise<Sponsor> {
    const res = await getHttpClient().post(
      `/${update ? "update" : "create"}`,
      sponsor
    );
    let data = (await res.data) as Sponsor;

    if (logo) {
      const form = new FormData();
      form.set("sponsor_uid", data.uid!);
      form.set("data", logo);

      const res = await getHttpClient().post("/update_logo", form);
      data = (await res.data) as Sponsor;
      data.imageUrl = data.imageUrl + "?t=" + Math.random() * 100; // force refresh
    }

    _replaceSponsor(data);
    return data;
  }

  async function deleteSponsor(sponsor: Sponsor) {
    await getHttpClient().post("/delete", { uid: sponsor.uid });
    sponsors.value = sponsors.value.filter((s) => s.uid !== sponsor.uid);
  }

  function _replaceSponsor(sponsor: Sponsor) {
    sponsors.value = sponsors.value.filter((s) => s.uid !== sponsor.uid);
    sponsors.value.push(sponsor);
  }

  return {
    fetchAllSponsors,
    getAllFavours,
    fetchSingleSponsor,
    fetchSettings,
    saveSettings,
    createOrUpdateSponsor,
    deleteSponsor,
    settings,
    sponsors,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useMainStore, import.meta.hot));
}
