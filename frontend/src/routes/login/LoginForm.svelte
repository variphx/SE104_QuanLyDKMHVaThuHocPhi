<script lang="ts">
	import { enhance } from '$app/forms';
</script>

<form method="POST" action="?/dang_nhap" use:enhance>
	<label class="label">
		<span> Tên người dùng </span>
		<input class="input" type="text" name="username" />
	</label>

	<label class="label">
		<span> Mật khẩu </span>
		<input class="input" type="password" name="password" />
	</label>

	<div class="flex flex-col items-center">
		<button class="btn variant-filled mt-4"> Đăng nhập </button>
	</div>
</form>
