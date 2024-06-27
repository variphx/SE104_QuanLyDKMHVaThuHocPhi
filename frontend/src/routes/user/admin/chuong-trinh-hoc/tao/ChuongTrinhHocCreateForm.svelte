<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<form method="POST" action="?/tao" use:enhance>
	<div class="grid grid-cols-2 gap-4">
		<label class="label">
			<span> Học kỳ </span>
			<input
				class="input"
				disabled
				value={`Học kỳ ${data.current_hoc_ky.ten}, Năm học ${data.current_hoc_ky.nam_hoc}`}
			/>
		</label>

		<label class="label">
			<span> Mật khẩu </span>
			<input class="input" type="password" name="password" />
		</label>
	</div>

	<div class="flex flex-col items-center mt-4">
		<button class="btn variant-filled" type="submit"> Tạo chương trình học </button>
	</div>

	<div class="flex flex-col items-center mt-16 gap-4">
		{#each data.nganhs as nganh (nganh.id)}
			<div class="bg-surface-500 outline-surface-300 outline w-1/2 p-4 rounded">
				{nganh.ten}
			</div>
		{/each}
	</div>
</form>
