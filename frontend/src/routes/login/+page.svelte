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

	<button class="btn variant-filled"> Đăng nhập </button>
</form>
