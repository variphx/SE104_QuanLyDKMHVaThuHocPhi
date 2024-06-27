<script lang="ts">
	import { page } from '$app/stores';
	import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton';

	type Path = {
		name: string;
		url: string;
	};

	const paths: Path[] = [
		{
			name: 'Tạo sinh viên',
			url: '/user/admin/sinh-vien/tao'
		},
		{
			name: 'Tra cứu sinh viên',
			url: '/user/admin/sinh-vien/tra-cuu'
		},
		{
			name: 'Tạo chương trình học',
			url: '/user/admin/chuong-trinh-hoc/tao'
		},
		{
			name: 'Tra cứu chương trình học',
			url: '/user/admin/chuong-trinh-hoc/tra-cuu'
		},
		{
			name: 'Tạo môn học',
			url: '/user/admin/mon-hoc/tao'
		},
		{
			name: 'Tra cứu môn học',
			url: '/user/admin/mon-hoc/tra-cuu'
		},
		{
			name: 'Mở môn học',
			url: '/user/admin/mon-hoc/mo'
		},
		{
			name: 'Tra cứu môn học mở',
			url: '/user/admin/mon-hoc/mo/tra-cuu'
		}
	];
</script>

<div class="grid grid-cols-[auto_1fr]">
	<AppRail class="h-screen">
		{#each paths as path}
			<AppRailAnchor href={path.url} selected={$page.url.pathname === path.url}>
				{path.name}
			</AppRailAnchor>
		{/each}
	</AppRail>
	<slot></slot>
</div>
