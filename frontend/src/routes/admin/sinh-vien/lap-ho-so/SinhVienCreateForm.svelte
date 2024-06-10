<script lang="ts">
  import { onMount } from "svelte";

  interface SinhVienCreatePayload {
    ten: string;
    can_cuoc: string;
    id_gioi_tinh: string;
    ngay_sinh: string;
    so_dien_thoai: string;
    email: string;
    id_doi_tuong: string;
    id_que_quan: string;
    id_chuong_trinh_hoc: string;
  }

  interface QueQuan {
    id: string;
    thanh_pho: string;
    tinh: string;
  }

  interface Nganh {
    id: string;
    ten: string;
    id_khoa: string;
  }

  let payload: SinhVienCreatePayload = {
    ten: "",
    id_gioi_tinh: "",
    ngay_sinh: "",
    id_doi_tuong: "",
    id_que_quan: "",
    id_chuong_trinh_hoc: "",
    can_cuoc: "",
    email: "",
    so_dien_thoai: "",
  };

  let id_nganh: string = "";
  let id_hoc_ky: string = "";
  let id_que_quan: string = "";

  $: payload.id_que_quan = id_que_quan;

  let que_quans: QueQuan[] = [];
  let nganhs: Nganh[] = [];

  onMount(async () => {
    fetch("http://localhost:8080/api/que-quan/get", {
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
      body: null,
    })
      .then((response) => response.json())
      .then((data) => (que_quans = data))
      .catch((error) => console.log(error));
  });

  onMount(async () => {
    fetch("http://localhost:8080/api/params/current-hoc-ky/get", {
      method: "GET",
      body: null,
    })
      .then((response) => response.json())
      .then((data) => (id_hoc_ky = data))
      .catch((error) => {
        throw new Error(error);
      });
  });

  onMount(async () => {
    fetch("http://localhost:8080/api/chuong-trinh-hoc/nganh/get", {
      method: "GET",
      body: null,
    })
      .then((response) => response.json())
      .then((data) => (nganhs = data))
      .catch((error) => {
        throw new Error(error);
      });
  });

  async function submitHandler() {
    const request = await fetch("http://localhost:8080/api/sinh-vien/post", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    if (!request.ok) {
      throw new Error(request.statusText);
    }
  }
</script>

<form on:submit|preventDefault={submitHandler}>
  <label class="lable mb-2">
    <span> Họ và tên sinh viên </span>
    <input class="input" type="text" bind:value={payload.ten} />
  </label>
  <label class="lable mb-2">
    <span> Giới tính </span>
    <select class="select" bind:value={payload.id_gioi_tinh}>
      <option value="Nam">Nam</option>
      <option value="Nữ">Nữ</option>
    </select>
  </label>
  <label class="lable mb-2">
    <span> Ngày sinh </span>
    <input class="input" type="date" bind:value={payload.ngay_sinh} />
  </label>
  <label class="lable mb-2">
    <span> Đối tượng ưu tiên </span>
    <input class="input" type="text" bind:value={payload.id_doi_tuong} />
  </label>
  <label class="lable mb-2">
    <span>Quê quán</span>
    <div class="input-group input-group-divider grid-cols-[1fr_auto]">
      <select class="select" bind:value={payload.id_que_quan}>
        {#each que_quans as iter}
          <option value={iter.id}>{iter.tinh}, {iter.thanh_pho}</option>
        {/each}
      </select>
      <div class="input-group-shim">{payload.id_que_quan}</div>
    </div>
  </label>
  <label class="lable mb-2">
    <span> Ngành học </span>
    <div class="input-group input-group-divider grid-cols-[1fr_auto]">
      <select class="select" bind:value={id_nganh}>
        {#each nganhs as iter}
          <option value={iter.id}>
            {iter.id_khoa}, {iter.ten}
          </option>
        {/each}
      </select>
      <div class="input-group-shim">{id_nganh}</div>
    </div>
  </label>

  <div class="flex flex-col">
    <button
      type="submit"
      class="mt-2 btn w-1/2 md:w-1/3 mx-auto variant-glass-primary">Tạo hồ sơ</button
    >
  </div>
</form>
