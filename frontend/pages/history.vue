<template>
  <div>
    <n-data-table
      remote
      :columns="columns"
      :pagination="paginationReactive as object"
      :data="dataRef"
      :loading="loading"
      @update:page="handlePageChange"
    >
    </n-data-table>
  </div>
</template>

<script setup lang="ts">
import { NButton } from "naive-ui";

const columns = [
  {
    title: "Time",
    key: "time",
  },
  {
    title: "Who",
    key: "who",
  },
  {
    title: "Action",
    key: "type",
  },
  {
    title: "What",
    key: "what",
    render(row: any) {
      return h(
        NButton,
        {
          strong: true,
          tertiary: true,
          size: "small",
          onClick: () => showWhat(row.what),
        },
        { default: () => `Show` }
      );
    },
  },
];

const paginationReactive = reactive({
  page: 1,
  pageCount: 1,
  pageSize: 100,
  //@ts-ignore
  prefix({ itemCount }) {
    return `Total is ${itemCount}.`;
  },
});

const dataRef = ref([]);
const loading = ref(false);
const dialog = useDialog();

onMounted(async () => {
  await handlePageChange(1);
});

async function handlePageChange(page: number) {
  if (loading.value) return;

  loading.value = true;
  const res = await getHttpClient().get(`/changes/${100 * (page - 1)}`);
  let { changes, total } = await res.data;

  paginationReactive.pageCount = Math.ceil(total / 100);
  //@ts-ignore
  paginationReactive.itemCount = total;
  paginationReactive.page = page;

  const intl = Intl.DateTimeFormat("de", {
    day: "numeric",
    weekday: "short",
    month: "numeric",
    year: "numeric",
    hour: "numeric",
    minute: "numeric",
    second: "numeric",
  });
  changes = changes.map((c: any) => {
    return {
      time: intl.format(new Date(c.when)),
      who: c.who,
      what: JSON.stringify(c.what, null, 2),
      type: Object.keys(c.what)[0],
    };
  });
  dataRef.value = changes;

  loading.value = false;
}

function showWhat(s: string) {
  dialog.create({
    content: () =>
      h("div", { class: "bg-gray-900 p-2 rounded w-fit" }, [
        h("pre", { innerHTML: s }),
      ]),
    style: "width: fit-content",
  });
}
</script>
