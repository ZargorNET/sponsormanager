// @ts-ignore
export default defineAppConfig({
  apiEndpoint: import.meta.env.PROD
    ? `${window.location.origin}/api/`
    : "http://localhost:8080/api/",
});
