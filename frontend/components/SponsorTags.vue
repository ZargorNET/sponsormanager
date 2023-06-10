<template>
  <div class="overflow-auto max-w-full">
    <n-space size="small">
      <n-tag :bordered="false" size="small" v-for="tag in tags" v-if="!edit">
        <NuxtLink
          :to="`/search/%23${encodeURI(`${tag}`)}`"
          class="overflow-hidden text-ellipsis"
        >
          {{ tag }}
        </NuxtLink>
      </n-tag>
      <template v-else>
        <n-button
          dashed
          v-for="(tag, index) in tags"
          size="tiny"
          @click="
            propIndexToEdit = index;
            showModal = true;
          "
        >
          {{ tag }}
        </n-button>
        <n-button type="primary" dashed size="tiny" @click="addTag()"
          >+
        </n-button>

        <n-modal
          v-model:show="showModal"
          style="
            width: 600px;
            position: fixed;
            top: 400px;
            right: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
          "
          preset="card"
          size="small"
          title="Edit Tag"
        >
          <template #default>
            <n-input v-model:value="tags[propIndexToEdit]" />

            <div class="flex justify-center child:mx-1 mt-4">
              <div>
                <n-button type="primary" @click="showModal = false"
                  >Ok
                </n-button>
              </div>
              <div>
                <n-button type="error" @click="deleteTag()">Delete</n-button>
              </div>
            </div>
          </template>
        </n-modal>
      </template>
    </n-space>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  tags: String[];
  edit?: boolean;
}>();

const showModal = ref(false);
const propIndexToEdit = ref(-1);
const tags = useVModel(props, "tags");

watch(showModal, (neww, _) => {
  if (!neww) cleanup();
});

function addTag() {
  if (tags.value[tags.value.length - 1] !== "") tags.value.push("");

  propIndexToEdit.value = tags.value.length - 1;
  showModal.value = true;
}

function cleanup() {
  for (let i = 0; i < tags.value.length; i++) {
    tags.value[i] = tags.value[i].trim();
  }

  if (tags.value[propIndexToEdit.value] === "")
    tags.value.splice(propIndexToEdit.value, 1);
}

function deleteTag() {
  tags.value.splice(propIndexToEdit.value, 1);

  showModal.value = false;
}
</script>
