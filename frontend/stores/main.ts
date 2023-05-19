export const useMainStore = defineStore('main', () => {
  //  const seasons: Ref<Array<string>> = ref(["2023", "2022"]);


    return {};
});

if (import.meta.hot) {
    import.meta.hot.accept(acceptHMRUpdate(useMainStore, import.meta.hot))
}
