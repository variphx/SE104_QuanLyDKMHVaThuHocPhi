<script lang="ts">
	import { enhance } from '$app/forms';

	const ten_hoc_kys: string[] = ['Một', 'Hai', 'Hè'];
</script>

<form method="POST" action="?/tao" use:enhance>
	<label class="label">
		<span> Mã ngành </span>
		<input class="input" type="text" name="id_nganh" />
	</label>

	<label class="label">
		<span> Tên ngành </span>
		<input class="input" type="text" name="ten_nganh" />
	</label>

	<label class="label">
		<span> Năm nhập học </span>
		<input class="input" type="number" />
	</label>

	<label class="label">
		<span> Học kì nhập học </span>
		<select class="select">
			{#each ten_hoc_kys as ten_hoc_ky}
				<option value={ten_hoc_ky}>
					{ten_hoc_ky}
				</option>
			{/each}
		</select>
	</label>

	<div class="flex flex-col items-center mt-4">
		<button type="submit" class="btn variant-filled">Tạo chương trình học</button>
	</div>
</form>
