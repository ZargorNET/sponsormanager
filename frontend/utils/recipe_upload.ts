/*
import {jsonRequestOptions} from "~/utils/http";
import {FormData} from "node-fetch-native";
import {Recipe} from "~/stores/recipes";

export interface UploadRecipe {
    name: string,
    tags: string[],
    time_required: string,
    summary: string,
    description: string,
    ingredients: { amount: string, typ: string }[]
}


export async function uploadRecipe(uploadRecipe: UploadRecipe, uploadFileKey: string): Promise<Recipe | null> {
    const {apiEndpoint} = useAppConfig();
    const authStore = useAuthStore();

    const sessionKey = authStore.user?.sessionToken ?? null;

    if (!sessionKey)
        return null;

    const {data, error} = await useFetch(`${apiEndpoint}/add_recipe`, {
        ...jsonRequestOptions,
        headers: {
            Authorization: `Bearer ${sessionKey}`
        },
        body: {...uploadRecipe, upload_file_key: uploadFileKey}
    });

    if (error.value) {
        console.error("error uploading file", error.value.data);
        return null;
    }

    return data.value.recipe;
}

/// return the upload_file_key
export async function uploadImage(file: File): Promise<string | undefined> {
    const {apiEndpoint} = useAppConfig();
    const authStore = useAuthStore();

    const sessionKey = authStore.user?.sessionToken ?? null;

    if (!sessionKey)
        return undefined;

    const formData = new FormData();
    formData.append("uploading", file);

    console.log("Uploading file", file);

    try {
        const res: any = await $fetch(`${apiEndpoint}/upload`, {
            method: "post",
            headers: {
                Authorization: `Bearer ${sessionKey}`
            },
            body: formData
        });

        return res.key;
    } catch (e) {
        console.error("error uploading file", e);
        return undefined;
    }
}
*/
