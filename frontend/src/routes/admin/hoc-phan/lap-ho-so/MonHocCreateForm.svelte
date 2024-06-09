<script lang="ts">
    interface MonHocCreatePayload {
        id: string;
        ten: string;
        id_khoa: string;
        id_giang_vien: string;
        so_tiet: number;
        id_loai_mon_hoc: string;
    }

    let payload: MonHocCreatePayload = {
        id: "",
        ten: "",
        id_khoa: "",
        id_giang_vien: "",
        so_tiet: 0,
        id_loai_mon_hoc: "",
    };

    async function submitHandler() {
        console.log(payload);
    }
</script>

<form on:submit|preventDefault={submitHandler}>
    <label class="lable">
        <span> Mã môn học </span>
        <input class="input" type="text" />
    </label>

    <lable class="lable">
        <span> Tên môn học </span>
        <input class="input" type="text" />
    </lable>

    <lable class="lable">
        <span> Khoa </span>
        <select class="select"></select>
    </lable>

    <lable class="lable">
        <span> Tên giảng viên </span>
        <select class="select" disabled={payload.id_khoa == ""}></select>
    </lable>

    <lable class="lable">
        <span> Số tiết </span>
        <input class="input" type="number" />
    </lable>

    <lable class="lable">
        <span> Hình thức thực hành </span>
        <select class="select"></select>
    </lable>

    <div>
        <button class="btn" type="submit"> Lập hồ sơ </button>
    </div>
</form>
