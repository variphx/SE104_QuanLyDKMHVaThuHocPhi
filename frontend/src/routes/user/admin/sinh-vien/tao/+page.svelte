<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<form method="POST" action="?/tao" use:enhance>
	<label class="label">
		<span> Họ và tên </span>
		<input class="input" type="text" name="ten" />
	</label>

	<label class="label">
		<span> Ngày sinh </span>
		<input class="input" type="date" name="ngay_sinh" />
	</label>

	<label class="label">
		<span> Giới tính </span>
		<select class="select" name="id_gioi_tinh">
			<option value="Nam">Nam</option>
			<option value="Nữ">Nữ</option>
		</select>
	</label>

	<label class="label">
		<span> Quê quán </span>
		<select class="select" name="id_que_quan">
			{#each data.que_quans as que_quan}
				<option value={que_quan.id}>
					{que_quan.thanh_pho}, {que_quan.tinh}
				</option>
			{/each}
		</select>
	</label>

	<label class="label">
		<span> Đối tượng chính sách </span>
		<select class="select" name="id_doi_tuong">
			{#each data.doi_tuongs as doi_tuong}
				<option value={doi_tuong.id}>
					{doi_tuong.ten}
				</option>
			{/each}
		</select>
	</label>

	<label class="label">
		<span> Chương trình học </span>
		<select class="select" name="id_chuong_trinh_hoc">
			{#each data.chuong_trinh_hocs as chuong_trinh_hoc}
				<option value={chuong_trinh_hoc.id}>
					Mã ngành: {chuong_trinh_hoc.id_nganh}, mã học kỳ: {chuong_trinh_hoc.id_hoc_ky}
				</option>
			{/each}
		</select>
	</label>

	<button type="submit" class="btn variant-filled"> Tạo hồ sơ </button>
</form>
