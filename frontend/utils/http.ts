import axios, {AxiosInstance} from "axios";

export function getHttpClient(): AxiosInstance {
    // @ts-ignore
    const {apiEndpoint} = useAppConfig();

    return axios.create({
        baseURL: apiEndpoint,
        headers: {
            'Content-Type': 'application/json',
        },
    });
}
