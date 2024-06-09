<script lang="ts">
    interface LoginPayload {
        username: String;
        password: String;
    }

    let payload: LoginPayload = {
        username: "",
        password: "",
    };

    async function submitHandler() {
        console.log(payload);
    }
</script>

<form on:submit|preventDefault={submitHandler}>
    <lable class="label">
        <span> Mã công chức/sinh viên </span>
        <input type="text" class="input" bind:value={payload.username} />
    </lable>

    <lable class="label">
        <span> Mật khẩu </span>
        <input type="password" class="input" bind:value={payload.password} />
    </lable>

    <lable class="lable">
        <input class="checkbox" type="checkbox" />
        <span> Nhớ tài khoản </span>
    </lable>

    <div class="relative self-center">
        <button class="btn" type="submit"> Đăng nhập </button>
    </div>
</form>
