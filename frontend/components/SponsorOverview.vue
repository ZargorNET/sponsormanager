<template>
    <div class="w-full h-full flex flex-col">
        <SponsorOverviewGrid :sponsors="sponsors"/>
    </div>
</template>

<script lang="ts" setup>
import SponsorOverviewGrid from "~/components/SponsorOverviewGrid.vue";
import {Ref} from "vue";
import {Sponsor} from "~/utils/sponsor";

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
