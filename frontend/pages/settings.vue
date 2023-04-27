<template>
    <div class="flex flex-grid justify-center">
        <div class="w-1/2">
            <n-card>
                <template #header>
                    <n-h1>Settings</n-h1>
                </template>

                <n-h2>
                    Mandatory Fields
                </n-h2>
                <n-p>Fields that are required on each new sponsor entry</n-p>
                <VariableInputs v-model:items="mandatoryFields" placeholder="Field Name"/>
                <div class="mt-2">
                    <n-button type="primary">Save</n-button>
                </div>

                <n-divider/>
                <n-h2>
                    Seasons
                </n-h2>

                <n-p>
                    <template v-for="s in mainStore.seasons">
                        {{ s }}<br/>
                    </template>
                </n-p>

                <div class="w-32">
                    Add a season:
                    <n-input maxlength="8" placeholder="Season"/>
                    <div class="mt-2">
                        <n-button type="primary">Add</n-button>
                    </div>
                </div>


                <n-divider/>
                <n-h2>
                    Statistics
                </n-h2>
                <n-p>Nothing here yet...</n-p>
            </n-card>
        </div>
    </div>
</template>

<script lang="ts" setup>

import VariableInputs from "~/components/VariableInputs.vue";
import {Ref} from "vue";

const mandatoryFields: Ref<Array<string>> = ref(["abc", "defg"]);
const mainStore = useMainStore();

</script>
