<script lang="ts">
	import type { PageServerData } from './$types';
	import SinhVienCreateForm from './SinhVienCreateForm.svelte';

	export let data: PageServerData;
</script>

<div class="flex flex-col items-center">
	<div class="mt-12 w-2/3">
		<SinhVienCreateForm {data}></SinhVienCreateForm>
	</div>
</div>
