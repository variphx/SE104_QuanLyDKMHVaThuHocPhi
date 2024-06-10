<script lang="ts">
  import { onMount } from "svelte";

  let nam_hoc_current: number = 2023;

  interface Nganh {
    id: string;
    id_khoa: string;
    ten: string;
  }

  let nganh_mos: Nganh[] = [];

  onMount(async () => {
    fetch("http://localhost:8080/api/chuong-trinh-hoc/nganh/get", {
      method: "GET",
      body: null,
    })
      .then((response) => response.json())
      .then((data) => (nganh_mos = data))
      .catch((error) => {
        throw new Error(error);
      });
  });

  async function removeNganh(id: string) {
    nganh_mos = nganh_mos.filter((nganh) => nganh.id != id);
  }

  async function submitHandler() {}
</script>

<form on:submit|preventDefault={submitHandler}>
  <div class="w-1/2">
  <lable class="lable">
    <span> Năm học tiếp theo </span>
    <input class="input" type="number" disabled value={nam_hoc_current + 1} />
  </lable>
  </div>

  <lable class="lable">
    <span> Ngành học sẽ mở </span>
    <ul>
      {#each nganh_mos as nganh (nganh.id)}
        <li>
          <span>
            {nganh.id_khoa}, {nganh.ten}
          </span>
          <button
            class="btn variant-glass-surface"
            type="button"
            on:click={async () => {
              await removeNganh(nganh.id);
            }}>(xóa)</button
          >
        </li>
      {/each}
    </ul>
  </lable>
</form>
