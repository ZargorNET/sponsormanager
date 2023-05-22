import axios, {AxiosError, AxiosInstance} from "axios";
import {getNotificationApi} from "~/utils/misc";

export function getHttpClient(): AxiosInstance {
    // @ts-ignore
    const {apiEndpoint} = useAppConfig();

    const instance = axios.create({
        baseURL: apiEndpoint,
        headers: {
            'Content-Type': 'application/json',
        },
    });

    instance.interceptors.response.use(null, (err: AxiosError) => {
        console.error(`Error while sending request`, err);

        getNotificationApi().error({title: "Error sending request. Please try again.", description: `${err.code} - ${err.message}`})
    });

    return instance;
}
