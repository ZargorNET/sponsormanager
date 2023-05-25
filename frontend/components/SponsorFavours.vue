<template>
    <div>
        <div v-for="(favour, index) in favoursRef">
            <n-card>
                <div class="w-full">
                    <div class="flex w-full items-center">
                        <div class="ml-auto" :class="{'cursor-pointer': edit}" @click="changeCompleted(favour)">
                            <n-tooltip trigger="hover">
                                <template #trigger>
                                    <Icon v-if="favour.completed" class="text-green-500" name="ooui:success"
                                          size="1.8em"/>
                                    <Icon v-else class="text-red-500" name="ooui:clear" size="1.8em"/>
                                </template>
                                <template #default>
                                    <template v-if="favour.completed">
                                        Favour is completed!
                                    </template>
                                    <template v-else>
                                        Favour is not completed.
                                    </template>
                                </template>
                            </n-tooltip>
                        </div>
                        <n-tooltip v-if="edit">
                            <template #trigger>
                                <div class="cursor-pointer" @click="deleteFavour(index)">
                                    <Icon name="material-symbols:delete" size="2em"/>
                                </div>
                            </template>
                            Delete
                        </n-tooltip>
                    </div>

                    <div>
                        <EditableInputField :edit="edit as boolean" :multi-line="true"
                                            v-model:model-value="favour.condition"/>
                    </div>

                    <div class="child:rounded child:bg-[#101014] child:p-1 flex relative">
                        <NuxtLink v-if="fetchSponsor && getSponsor(favour.sponsorUid)"
                                  :to="`/sponsor/${favour.sponsorUid}`">
                            {{ getSponsor(favour.sponsorUid).name }}
                        </NuxtLink>
                        <div class="ml-auto text-md flex items-center" :class="{'cursor-pointer': edit}"
                             @click="edit && toggleDate(favour)">
                            <Icon name="material-symbols:calendar-today" class="text-xl mr-0.5"/>
                            {{ new Date(favour.dueUntil).toDateString() }}
                        </div>
                        <div class="absolute right-0 top-8 bg-none z-10" v-if="viewDatePicker.has(favour.uid)">
                            <n-date-picker panel type="date" v-model:value="favour.dueUntil as number"/>
                        </div>
                    </div>
                </div>
            </n-card>
        </div>
        <div v-if="favours.length === 0" class="flex justify-center w-full">
            <n-empty description="No favours found" size="huge"/>
        </div>
    </div>
</template>

<script setup lang="ts">
import {Sponsor, SponsorFavour} from "~/utils/sponsor";

const props = defineProps<{
    favours: SponsorFavour[],
    edit?: boolean,
    fetchSponsor?: boolean
}>();
defineEmits(['update:favours']);

const favoursRef = useVModel(props, "favours");
const mainStore = useMainStore();
const viewDatePicker = ref(new Set());

watch(() => props.edit, (value) => {
    if (!value)
        viewDatePicker.value.clear();
});

function getSponsor(uid: string): Sponsor | undefined {
    return mainStore.sponsors.find(s => s.uid === uid);
}

function changeCompleted(favour: SponsorFavour) {
    if (props.edit)
        favour.completed = !favour.completed;
}

function deleteFavour(index: number) {
    favoursRef.value.splice(index, 1);
}

function toggleDate(favour: SponsorFavour) {
    if (viewDatePicker.value.has(favour.uid))
        viewDatePicker.value.delete(favour.uid);
    else
        viewDatePicker.value.add(favour.uid);
}
</script>
