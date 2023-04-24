import {UseFetchOptions} from "#app/composables/fetch";

export const jsonRequestOptions: UseFetchOptions<any> = {
    method: "post",
    headers: {
        "content-type": "application/json"
    }
};

export function get_recipe_image_url(key: String): String {
    const {apiEndpoint} = useAppConfig();
    return `${apiEndpoint}/image/${key}`;
}
