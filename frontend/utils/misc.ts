import {NotificationApi} from "naive-ui";

export function getNotificationApi(): NotificationApi {
    //@ts-ignore
    return window["$notification"];
}
