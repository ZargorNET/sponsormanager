<template>
    <div>
        <div class="mx-8 lg:mx-32 py-4 flex lg:items-center child:flex-1">
            <NuxtLink to="/">
                <div class="flex items-center">
                    <Icon name="fluent-emoji-flat:wrapped-gift" size="2.5rem"/>
                    <h1 class="font-extrabold text-2xl ml-1 text-white">Sponsored<span
                        class="text-[8pt] text-red-500">ALPHA</span></h1>
                </div>
            </NuxtLink>
            <div class="hidden lg:block">
                <NavbarSearch/>
            </div>
            <div class="flex flex-col lg:flex-row">
                <button class="lg:hidden ml-auto" @click="mobileShowMenu = !mobileShowMenu">
                    <Icon name="ion:menu" size="3em"/>
                </button>
                <div class="hidden lg:flex ml-auto items-center justify-center">
                    <NavbarAdd class="mr-4"/>
                    <NavbarFavours class="mr-4"/>
                    <NavbarSettings class="mr-4"/>
                    <!--                    <NavbarHistory class="mr-4"/>-->
                    <NavbarUser/>
                </div>
            </div>
        </div>
        <div class="lg:hidden flex flex-col items-center child:mb-2" :class="mobileShowMenu ? '' : 'hidden'">
            <NavbarAdd/>
            <NavbarFavours/>
            <NavbarSettings/>
            <!--            <NavbarHistory/>-->
            <NavbarUser/>
            <NavbarSearch/>
        </div>
    </div>
</template>

<script setup>

import NavbarAdd from "~/layouts/components/navbar/NavbarAdd.vue";
import NavbarSettings from "~/layouts/components/navbar/NavbarSettings.vue";
import NavbarUser from "~/layouts/components/navbar/NavbarUser.vue";
import NavbarFavours from "~/layouts/components/navbar/NavbarFavours.vue";
import NavbarSearch from "~/layouts/components/navbar/NavbarSearch.vue";

const mobileShowMenu = ref(false);
</script>
