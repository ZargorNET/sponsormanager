<template>
    <n-input v-if="edit" v-model:value="val" :type="multiLine ? 'textarea' : 'text'" clearable show-count/>
    <template v-else>
        <slot>{{ val }}</slot>
    </template>
</template>

<script setup lang="ts">

const props = defineProps<{
    edit: boolean,
    multiLine?: boolean
    modelValue: string
}>();
defineEmits(['update:modelValue']);

const val = useVModel(props, "modelValue");
</script>
