import {LoadingBarApi, NotificationApi} from "naive-ui";

export function getNotificationApi(): NotificationApi {
    //@ts-ignore
    return window["$notification"];
}

export function getLoadingBar(): LoadingBarApi {
    // @ts-ignore
    return window["$loadingBar"];
}
