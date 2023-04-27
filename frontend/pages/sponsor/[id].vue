<template>
    <div>
        <div>
            <n-card>
                <div class="absolute top-8 right-8">
                    <n-tooltip>
                        <template #trigger>
                            <Icon :name="!edit ? 'material-symbols:edit' : 'material-symbols:save'" class="text-2xl cursor-pointer" @click="editBtn()"/>
                        </template>
                        Edit
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
                            <SponsorTags :tags="sponsor.tags" />
                        </div>

                        <div :class="sponsor.favoursCompleted ? 'bg-green-400' : 'bg-red-400'"
                             class="w-full py-2 my-2 text-lg text-center text-bold">
                            <template v-if="sponsor.favoursCompleted">
                                All favours completed
                            </template>
                            <template v-else>
                                {{ sponsor.favours.filter(f => !f.completed).length }} favours open!
                            </template>
                        </div>
                    </div>
                    <div class="h-full w-full p-2 flex child:flex-1">
                        <div>
                            Fields:
                            <n-collapse>
                                <n-collapse-item v-for="field in fields" :title="field.name">
                                    {{ field.value }}
                                </n-collapse-item>
                            </n-collapse>
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
        {{ sponsor }}
    </div>
</template>

<script lang="ts" setup>
import {Sponsor, SponsorFavour, SponsorField} from "~/utils/sponsor";
import SponsorImage from "~/components/SponsorImage.vue";
import {ComputedRef} from "vue";

const route = useRoute();
const fields: ComputedRef<SponsorField[]> = computed(() => [{
    name: "Description",
    value: sponsor.shortDescription
}].concat(sponsor.fields));
const favours: ComputedRef<SponsorFavour[]> = computed(() => {
   return [...sponsor.favours].sort((a,b) => (a.completed ? 1 : 0) - (b.completed ? 1 : 0));
});
const edit = ref(false);

function editBtn() {
    if(!edit.value) {
        edit.value = true;

        window.addEventListener("beforeunload", areYouSureToExit);
        return;
    }
}

onBeforeUnmount(() => {
    window.removeEventListener("beforeunload", areYouSureToExit);
});

function areYouSureToExit(e: BeforeUnloadEvent) {
    e.preventDefault();
    e.returnValue = "";
}

const sponsor: Sponsor = {
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
        comment: "Maaybe?"
    }]
};

</script>
