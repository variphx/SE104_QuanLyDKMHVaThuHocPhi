<script lang="ts">
    import { onMount } from "svelte";

    interface SinhVienCreatePayload {
        ten: string;
        gioi_tinh: string;
        ngay_sinh: string;
        id_doi_tuong: string;
        id_que_quan: string;
        id_nganh: string;
    }

    interface QueQuan {
        id: string;
        thanh_pho: string;
        tinh: string;
    }

    interface Nganh {
        id: string;
        ten: string;
        khoa: string;
    }

    let payload: SinhVienCreatePayload = {
        ten: "",
        gioi_tinh: "",
        ngay_sinh: "",
        id_doi_tuong: "",
        id_que_quan: "",
        id_nganh: "",
    };

    let que_quans: QueQuan[] = [];
    let nganhs: Nganh[] = [];

    onMount(async () => {
        fetch("http://localhost:8080/api/que-quan/get", {
            headers: {
                "Content-Type": "application/json",
            },
            method: "POST",
            body: null,
        })
            .then((response) => response.json())
            .then((data) => (que_quans = data))
            .catch((error) => console.log(error));
    });

    async function submitHandler() {
        console.log(payload);
    }
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
        <span>Quê quán</span>
        <div class="input-group input-group-divider grid-cols-[1fr_auto]">
            <select class="select" bind:value={payload.id_que_quan}>
                {#each que_quans as iter}
                    <option value={iter.id}
                        >{iter.tinh}, {iter.thanh_pho}</option
                    >
                {/each}
            </select>
            <div class="input-group-shim">{payload.id_que_quan}</div>
        </div>
    </label>
    <label class="lable">
        <span> Ngành học </span>
        <div class="input-group input-group-divider grid-cols-[1fr_auto]">
            <select class="select" bind:value={payload.id_nganh}>
                {#each nganhs as iter}
                    <option value={iter.id}>
                        {iter.khoa}, {iter.ten}
                    </option>
                {/each}
            </select>
            <div class="input-group-shim">{payload.id_nganh}</div>
        </div>
    </label>

    <div>
        <button type="submit" class="btn">Tạo hồ sơ</button>
    </div>
</form>
