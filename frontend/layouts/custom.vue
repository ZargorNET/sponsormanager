<template>
    <div class="bg-black w-screen h-screen text-white">
        <n-layout content-style="display: flex; flex-direction: column; width: 100vw; height: 100vh;">
            <n-layout-header>
                <Navbar/>
            </n-layout-header>
            <n-layout-content>
                <!--suppress JSUnusedGlobalSymbols -->
                <div class="w-full min-h-full flex flex-col py-2 px-8 child:flex-grow">
                    <NuxtPage
                        :key="key"
                        :transition="{
                        name: 'page',
                        mode: 'out-in',
                        onEnterCancelled: () => load.error(),
                        onLeaveCancelled: () => load.error(),
                        onBeforeEnter: () => load.finish(),
                        onBeforeLeave: () => load.start(),
                    }"
                    />
                    <div class="flex">
                        <Footer class="mt-auto"/>
                    </div>
                </div>

            </n-layout-content>
        </n-layout>
    </div>
</template>

<script setup>
import Navbar from "~/layouts/components/navbar/Navbar.vue";
import Footer from "~/layouts/components/Footer.vue";

const load = useLoadingBar()
const notification = useNotification();

window.$notification = notification;

// TODO: Remove when https://github.com/vuejs/core/issues/5513 fixed
const key = ref(0)
const messages = [
    "Uncaught NotFoundError: Failed to execute 'insertBefore' on 'Node': The node before which the new node is to be inserted is not a child of this node.", // chromium based
    "NotFoundError: The object can not be found here." // safari
]
if (typeof window !== "undefined") {
    // @ts-expect-error using arbitrary window key
    if (!window.__vue5513) {
        window.addEventListener("error", (event) => {
            if (messages.includes(event.message)) {
                event.preventDefault()
                notification.warning({
                    title: "Soft Reload",
                    description: "Ein Soft Reload wurde automatisch ausgeführt, um einen Bug zu beheben."
                        + " Verwende die Navigation langsamer um diesen Fehler zu vermeiden.",
                    keepAliveOnHover: false,
                    duration: 5000
                })
                console.warn(
                    "Rerendering layout because of https://github.com/vuejs/core/issues/5513"
                )
                key.value++
            }
        })
    }
    // @ts-expect-error using arbitrary window key
    window.__vue5513 = true
}
</script>


<style>
.page-enter-from {
    opacity: 0;
}

.page-enter-active {
    transition: all 0.3s ease-out;
}

.page-leave-to {
    opacity: 0;
}

.page-leave-active {
    transition: all 0.3s ease-in;
}
</style>
