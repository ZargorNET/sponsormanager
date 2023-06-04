import axios, {AxiosError, AxiosInstance} from "axios";
import {getLoadingBar, getNotificationApi} from "~/utils/misc";

export function getHttpClient(addErrorInterceptor: boolean = true): AxiosInstance {
    // @ts-ignore
    const {apiEndpoint} = useAppConfig();

    const instance = axios.create({
        baseURL: apiEndpoint,
        headers: {
            'Content-Type': 'application/json',
        },
    });

    instance.interceptors.request.use((config) => {
        getLoadingBar().start();
        return config;
    });

    instance.interceptors.response.use((res) => {
        getLoadingBar().finish();
        return res;
    }, (err: AxiosError) => {
        if (!addErrorInterceptor)
            return;

        getLoadingBar().error();
        console.error(`Error while sending request`, err);

        getNotificationApi().error({
            title: "Error sending request. Please try again.",
            description: `${err.code} - ${err.message}`
        })
    });

    return instance;
}
