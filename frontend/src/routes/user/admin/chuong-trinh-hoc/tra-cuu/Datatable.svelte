<script lang="ts">
	import { DataHandler } from '@vincjo/datatables';
	import type { PageServerData } from './$types';
	import Search from '$lib/datatables/client/Search.svelte';
	import RowsPerPage from '$lib/datatables/client/RowsPerPage.svelte';
	import ThSort from '$lib/datatables/client/ThSort.svelte';
	import ThFilter from '$lib/datatables/client/ThFilter.svelte';
	import RowCount from '$lib/datatables/client/RowCount.svelte';
	import Pagination from '$lib/datatables/client/Pagination.svelte';

	export let data: PageServerData;

	const handler = new DataHandler(data.chuong_trinh_hocs, { rowsPerPage: 5 });
	const rows = handler.getRows();
</script>

<div class=" overflow-x-auto space-y-4">
	<!-- Header -->
	<header class="flex justify-between gap-4">
		<Search {handler} />
		<RowsPerPage {handler} />
	</header>
	<!-- Table -->
	<table class="table table-hover table-compact w-full table-auto">
		<thead>
			<tr>
				<ThSort {handler} orderBy="id">Mã chương trình học</ThSort>
				<ThSort {handler} orderBy="id_nganh">Mã ngành</ThSort>
				<ThSort {handler} orderBy="id_hoc_ky">Mã học kỳ</ThSort>
			</tr>
			<tr>
				<ThFilter {handler} filterBy="id" />
				<ThFilter {handler} filterBy="id_nganh" />
				<ThFilter {handler} filterBy="id_hoc_ky" />
			</tr>
		</thead>
		<tbody>
			{#each $rows as row}
				<tr>
					<td>{row.id}</td>
					<td>{row.id_nganh}</td>
					<td>{row.id_hoc_ky}</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<!-- Footer -->
	<footer class="flex justify-between">
		<RowCount {handler} />
		<Pagination {handler} />
	</footer>
</div>
