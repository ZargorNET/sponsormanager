import axios, { AxiosError, AxiosInstance } from "axios";
import { getLoadingBar, getNotificationApi } from "~/utils/misc";

export function getHttpClient(
  addErrorInterceptor: boolean = true,
  loadingBar: boolean = true
): AxiosInstance {
  // @ts-ignore
  const { apiEndpoint } = useAppConfig();
  const authStore = useAuthStore();

  const instance = axios.create({
    baseURL: apiEndpoint,
    headers: {
      Authorization: `Bearer ${authStore.sessionCookie}`,
    },
  });

  instance.interceptors.request.use((config) => {
    if (loadingBar) getLoadingBar().start();
    return config;
  });

  instance.interceptors.response.use(
    (res) => {
      if (loadingBar) getLoadingBar().finish();
      return res;
    },
    (err: AxiosError) => {
      if (loadingBar) getLoadingBar().error();
      console.error(`Error while sending request`, err);

      if (!addErrorInterceptor) return;

      getNotificationApi().error({
        title: "Error sending request. Please try again.",
        description: `${err.message} - ${JSON.stringify(err.response?.data)}`,
        duration: 6000,
      });
    }
  );

  return instance;
}
