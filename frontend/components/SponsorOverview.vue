<template>
    <div class="w-full h-full flex flex-col">
        <div class="flex ml-auto">
            <n-button :style="isGrid ? 'background-color: rgb(45 45 49)' : ''" secondary
                      style="margin-right: 4px !important"
                      @click="isGrid = true">
                <Icon name="material-symbols:grid-view" size="1.5rem"/>
            </n-button>
            <n-button :style="isGrid ? '' : 'background-color: rgb(45 45 49)'" secondary @click="isGrid = false">
                <Icon name="material-symbols:view-list" size="1.5rem"/>
            </n-button>
        </div>
        <SponsorOverviewGrid v-if="isGrid" :sponsors="sponsors"/>
        <SponsorOverviewList v-else :sponsors="sponsors"/>
    </div>
</template>

<script lang="ts" setup>
import SponsorOverviewList from "~/components/SponsorOverviewList.vue";
import SponsorOverviewGrid from "~/components/SponsorOverviewGrid.vue";
import {Ref} from "vue";
import {Sponsor} from "~/utils/sponsor";

const isGrid = ref(true);
const sponsors: Ref<Array<Sponsor>> = ref([{
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
    favours: []
},
    {
        uid: "dw",
        name: "Sponsor2",
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
        tags: ["Elektrik"],
        favoursCompleted: false,
        favours: []
    }])
</script>
