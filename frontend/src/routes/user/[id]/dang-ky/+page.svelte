<script lang="ts">
  import { onMount } from "svelte";

  interface DangKyHocPhanCreatePayload {}

  interface MonHoc {
    id: string;
  }

  let payloads: DangKyHocPhanCreatePayload[];

  let mon_hocs: MonHoc[];

  async function submitHandler() {
    for (let payload of payloads) {
      const request = await fetch("http://localhost:8080/api/hoc-phan/post", {
        headers: {
          "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(payload),
      });

      if (request.ok) {
        console.log("Đăng ký học phần thành công");
      } else {
        console.log("Đăng ký học phần thất bại");
      }
    }
  }

  onMount(async () => {
    await fetch("http://localhost:8080/api/mon-hoc/mo/get", {
      method: "GET",
    })
      .then((response) => response.json())
      .then((data) => (mon_hocs = data))
      .catch((error) => {
        throw new Error(error);
      });
  });
</script>

<div class="table-container">
  <div class="table">
    <th></th>
    <tbody>
      {#each mon_hocs as mon_hoc (mon_hoc.id)}{/each}
    </tbody>
  </div>
</div>
