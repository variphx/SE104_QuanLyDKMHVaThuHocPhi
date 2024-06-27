<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<form method="POST" action="?/tao" use:enhance>
	<div class="grid grid-cols-1 gap-4">
		<label class="label">
			<span> Tên môn học </span>
			<input class="input" type="text" name="ten" />
		</label>

		<label class="label">
			<span> Mã môn học </span>
			<input class="input" type="text" name="id" />
		</label>

		<label class="label">
			<span> Thuộc khoa </span>
			<select class="select" name="id_khoa">
				{#each data.khoas as khoa (khoa.id)}
					<option value={khoa.id}>
						{khoa.ten}
					</option>
				{/each}
			</select>
		</label>

		<label class="lable">
			<span> Loại môn học </span>
			<select class="select" name="loai">
				<option value="LT"> Lý thuyết </option>
				<option value="TH"> Thực hành </option>
			</select>
		</label>

		<label class="label">
			<span> Số tiết </span>
			<input class="input" type="number" name="so_tiet" />
		</label>

		<div class="flex flex-col items-center">
			<button class="btn variant-filled" type="submit"> Tạo môn học </button>
		</div>
	</div>
</form>
