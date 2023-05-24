<template>
    <div>
        <div>
            <n-card>
                <div class="absolute top-8 right-8 z-10">
                    <n-tooltip>
                        <template #trigger>
                            <div class="text-3xl cursor-pointer rounded" @click="editBtn()">
                                <Icon :name="!edit ? 'material-symbols:edit' : 'material-symbols:save'"/>
                            </div>
                        </template>
                        {{ edit ? "Save" : "Edit" }}
                    </n-tooltip>
                </div>
                <div class="flex">
                    <div class="border-r-2 border-gray-600 p-2 mr-2">
                        <div class="w-72 h-72">
                            <SponsorImage :sponsor="sponsor"/>
                        </div>
                        <p class="text-center text-3xl text-bold mt-2">
                            {{ sponsor.name }}
                        </p>

                        <div class="flex mt-2">
                            <SponsorTags :tags="sponsor.tags" :edit="edit"/>
                        </div>

                        <div :class="sponsor.favoursCompleted ? 'bg-green-400' : 'bg-red-400'"
                             class="w-full py-2 my-2 text-lg text-center text-bold rounded shadow-innerRing">
                            <template v-if="sponsor.favoursCompleted">
                                All favours completed
                            </template>
                            <template v-else>
                                Some favours open!
                            </template>
                        </div>
                        <div>
                            Favours <span class="float-right">{{ sponsor.favours.filter(f => f.completed).length }}/{{
                                sponsor.favours.length
                            }}</span>
                        </div>
                    </div>
                    <div class="h-full w-full p-2 flex child:flex-1">
                        <div>
                            Fields:
                            <n-collapse>
                                <n-collapse-item title="Description">
                                    <EditableInputField :edit="edit" v-model="sponsor.shortDescription" multi-line/>
                                </n-collapse-item>
                                <n-collapse-item v-for="(field, index) in sponsor.fields" :title="field.name">
                                    <EditableInputField :edit="edit" v-model="sponsor.fields[index].value" multi-line/>

                                    <template #header-extra v-if="edit && !fieldIsMandatory(field.name)">
                                        <n-tooltip>
                                            <template #trigger>
                                                <div class="text-2xl" @click="sponsor.fields.splice(index, 1)">
                                                    <Icon name="material-symbols:delete"/>
                                                </div>
                                            </template>
                                            Delete
                                        </n-tooltip>
                                    </template>
                                </n-collapse-item>
                            </n-collapse>
                            <div class="flex justify-center mt-4">
                                <n-button type="success" v-if="edit" @click="addField()">Add</n-button>
                            </div>
                        </div>
                        <div>
                            Favours:
                            <n-collapse>
                                <n-collapse-item v-for="favour in favours" :title="favour.condition">
                                    {{ favour }}
                                </n-collapse-item>
                            </n-collapse>
                        </div>
                    </div>
                </div>
            </n-card>
        </div>
        {{ route.params.id }}
        <br/>
        {{ sponsor }}
    </div>
</template>

<script lang="ts" setup>
import {Sponsor, SponsorFavour} from "~/utils/sponsor";
import SponsorImage from "~/components/SponsorImage.vue";
import {ComputedRef, Ref} from "vue";
import {NButton, NInput} from "naive-ui";

const route = useRoute();
const dialog = useDialog();
const mainStore = useMainStore();
const favours: ComputedRef<SponsorFavour[]> = computed(() => {
    return [...sponsor.value.favours].sort((a, b) => (a.completed ? 1 : 0) - (b.completed ? 1 : 0));
});
const edit: Ref<boolean> = ref(false);

function editBtn() {
    if (!edit.value) {
        edit.value = true;

        window.addEventListener("beforeunload", areYouSureToExit);
        return;
    }

    window.removeEventListener("beforeunload", areYouSureToExit);
    edit.value = false;
}

onBeforeUnmount(() => {
    window.removeEventListener("beforeunload", areYouSureToExit);
});

const addFieldValue = ref();

function addField() {
    dialog.create({
        content: () => h(
            'div',
            [
                h("span", "Field Name"),
                h(NInput, {
                    "placeholder": "Field Name",
                    "v-model:value": addFieldValue.value,
                    "onUpdate:value": (val) => addFieldValue.value = val
                })
            ]
        ),
        title: "Add a Field",
        positiveText: "Ok",
        onPositiveClick: () => new Promise<void>((res) => {
            sponsor.value.fields.push({name: addFieldValue.value, value: ""})
            res();
        })
    })
}

function areYouSureToExit(e: BeforeUnloadEvent) {
    e.preventDefault();
    e.returnValue = "";
}

function fieldIsMandatory(field_name: string): boolean {
    return mainStore.settings.mandatoryFields.includes(field_name);
}

const sponsor: Ref<Sponsor> = ref({
    uid: "dw",
    name: "Sponsor1",
    shortDescription: "xD",
    fields: [
        {
            name: "wer machts?",
            value: "conner hat gesagt er macht es"
        },
        {
            name: "besonderheiten? anyy?",
            value: "abc hehehehehhwhwdu qd hqwjldjqw dj qw hqwd hqhd qwlid uqwhduqwiud3298r8wfdsv iu wqeuifadsf 3p892iuebjif jbh"
        }
    ],
    tags: ["junge", "xD"],
    favoursCompleted: true,
    favours: [{
        uid: "a",
        sponsorUid: "dw",
        condition: "Insta Posttt",
        completed: false,
        dueUntil: new Date(),
    }]
});

</script>
