<script lang="ts">
  import { get_thanh_pho_options } from "$lib/api/get_thanh_pho_options";

  export let tinh_options;
  export let doi_tuong_options;
  export let khoa_options;

  let tinh_selected_id: string;
  let khoa_selected_id: string;
  $: thanh_pho_options_json = get_thanh_pho_options(tinh_selected_id);
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
    <input type="date" class="input" />
  </lable>

  <lable class="label">
    <span>Quê quán</span>
    <div>
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

  <button type="submit" class="btn variant-filled">Tạo</button>
</form>
