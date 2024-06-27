<script lang="ts">
	import type { PageServerData } from './$types';
	import MonHocCreateForm from './MonHocCreateForm.svelte';

	export let data: PageServerData;
</script>

<div class="flex flex-col items-center">
	<div class="w-2/3 mt-12">
		<MonHocCreateForm {data}></MonHocCreateForm>
	</div>
</div>
