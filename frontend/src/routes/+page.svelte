<script lang="ts">
  </script>

<div class="flex flex-col min-h-screen">
  <div class="mx-auto my-auto w-4/5 md:w-1/3 xl:w-1/4">
    <form
      method="POST"
      action="?/login"
      class="px-8 py-4 variant-glass-surface rounded-lg"
    >
      <label class="lable my-2">
        <span> Mã công chức/sinh viên </span>
        <input class="input" type="text" name="username" />
      </label>

      <lable class="label my-2">
        <span> Mật khẩu </span>
        <input class="input" type="password" name="password" />
      </lable>

      <div class="flex flex-col mt-4">
        <div class="mx-auto my-auto">
          <button class="btn variant-filled mx-auto" type="submit"
            >Đăng nhập</button
          >
        </div>
      </div>
    </form>
  </div>
</div>
