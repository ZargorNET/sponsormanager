<template>
    <div>
        <div class="mx-8 lg:mx-32 py-4 flex lg:items-center child:flex-1">
          <div class="flex">
            <NuxtLink to="/">

              <object :data="SVGLogo" class="w-24 pointer-events-none"/>

            </NuxtLink>
          </div>
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
              <NavbarSettings class="mr-4" v-if="authStore.isAdmin()"/>
              <NavbarHistory class="mr-4"/>
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

import SVGLogo from '@/assets/logo.svg?url';
import NavbarHistory from "~/layouts/components/navbar/NavbarHistory.vue";

const authStore = useAuthStore();
const mobileShowMenu = ref(false);
</script>
