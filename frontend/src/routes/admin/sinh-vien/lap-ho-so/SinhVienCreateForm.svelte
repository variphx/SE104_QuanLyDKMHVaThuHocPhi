<script lang="ts">
    interface SinhVienCreatePayload {
        ten: string;
        gioi_tinh: string;
        ngay_sinh: string;
        id_doi_tuong: string;
        id_que_quan: string;
        id_nganh: string;
    }

    interface QueQuan {
        thanh_pho: string;
        tinh: string;
    }

    let payload: SinhVienCreatePayload = {
        ten: "",
        gioi_tinh: "",
        ngay_sinh: "",
        id_doi_tuong: "",
        id_que_quan: "",
        id_nganh: "",
    };

    let que_quan: QueQuan = {
        thanh_pho: "",
        tinh: "",
    };

    async function submitHandler() {
        console.log(payload);
    }

    let isInputTinh: boolean = false;

    $: isInputTinh = que_quan.tinh != "";
</script>

<form on:submit|preventDefault={submitHandler}>
    <label class="lable">
        <span> Họ và tên sinh viên </span>
        <input class="input" type="text" bind:value={payload.ten} />
    </label>
    <label class="lable">
        <span> Giới tính </span>
        <select class="select" bind:value={payload.gioi_tinh}>
            <option value="Nam">Nam</option>
            <option value="Nữ">Nữ</option>
        </select>
    </label>
    <label class="lable">
        <span> Ngày sinh </span>
        <input class="input" type="date" bind:value={payload.ngay_sinh} />
    </label>
    <label class="lable">
        <span> Đối tượng ưu tiên </span>
        <input class="input" type="text" bind:value={payload.id_doi_tuong} />
    </label>
    <label class="lable">
        <span> Quê quán </span>
        <div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
            <div class="input-group-shim">Tỉnh</div>
            <select></select>
        </div>
        <div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
            <div class="input-group-shim">Thành phố</div>
            <select disabled={!isInputTinh}></select>
        </div>
    </label>
    <label class="lable">
        <span> Ngành học </span>
        <input class="input" type="text" bind:value={payload.id_nganh} />
    </label>

    <div>
        <button type="submit" class="btn">Tạo hồ sơ</button>
    </div>
</form>
