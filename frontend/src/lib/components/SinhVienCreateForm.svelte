<script lang="ts">
  import { get_nganh_options } from "$lib/api/get_nganh_options";
  import { get_thanh_pho_options } from "$lib/api/get_thanh_pho_options";

  export let tinh_options;
  export let doi_tuong_options;
  export let khoa_options;

  let tinh_selected_id: string;
  let khoa_selected_id: string;
  $: thanh_pho_options_json = get_thanh_pho_options(tinh_selected_id);
  $: nganh_options_json = get_nganh_options(khoa_selected_id);
</script>

<form method="POST" action="?/create_single_sinh_vien">
  <lable class="lable">
    <span> Họ và tên </span>
    <input type="text" name="ten" class="input" />
  </lable>

  <lable class="label">
    <span>Giới tính</span>
    <select name="id_gioi_tinh" class="select">
      <option value="Nam">Nam</option>
      <option value="Nữ">Nữ</option>
    </select>
  </lable>

  <lable class="label">
    <span>Ngày sinh</span>
    <input type="date" class="input" name="ngay_sinh" />
  </lable>

  <lable class="label">
    <span>Quê quán</span>
    <div class="grid grid-cols-2 gap-4">
      <label class="lable">
        <span>Tỉnh</span>
        <select class="select" bind:value={tinh_selected_id}>
          {#each tinh_options as tinh_option}
            <option value={tinh_option.id}>{tinh_option.ten}</option>
          {/each}
        </select>
      </label>

      <label class="label">
        <span>Thành phố</span>
        <select class="select" name="id_que_quan" disabled={!tinh_selected_id}>
          {#await thanh_pho_options_json then thanh_pho_options}
            {#each thanh_pho_options as thanh_pho_option}
              <option value={thanh_pho_option.id}>{thanh_pho_option.ten}</option
              >
            {/each}
          {/await}
        </select>
      </label>
    </div>
  </lable>

  <label class="label">
    <span>Đối tượng</span>
    <select class="select" name="id_doi_tuong">
      {#each doi_tuong_options as doi_tuong_option}
        <option value={doi_tuong_option.id}>{doi_tuong_option.ten}</option>
      {/each}
    </select>
  </label>

  <label class="label">
    <span>Khoa</span>
    <select class="select" bind:value={khoa_selected_id}>
      {#each khoa_options as khoa_option}
        <option value={khoa_option.id}>{khoa_option.ten}</option>
      {/each}
    </select>
  </label>

  <lable class="label">
    <span>Ngành</span>
    <select class="select" name="id_nganh" disabled={!khoa_selected_id}>
      {#await nganh_options_json then nganh_options}
        {#each nganh_options as nganh_option}
          <option value={nganh_option.id}>{nganh_option.ten}</option>
        {/each}
      {/await}
    </select>
  </lable>

  <div class="flex flex-col">
    <button type="submit" class="btn variant-filled mx-auto mt-4 px-8"
      >Tạo</button
    >
  </div>
</form>
