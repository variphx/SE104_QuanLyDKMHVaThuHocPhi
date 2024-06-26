<script lang="ts">
	import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton';

	type Path = {
		name: string;
		url: string;
	};

	const paths: Path[] = [
		{
			name: 'Sinh viên',
			url: './admin/sinh-vien'
		},
		{
			name: 'Chương trình học',
			url: './admin/chuong-trinh-hoc'
		},
		{
			name: 'Môn học',
			url: './admin/sinh-vien'
		}
	];
</script>

<div class="grid grid-cols-[auto_1fr]">
	<AppRail>
		{#each paths as path}
			<AppRailAnchor href={path.url}>
				{path.name}
			</AppRailAnchor>
		{/each}
	</AppRail>
	<slot></slot>
</div>
