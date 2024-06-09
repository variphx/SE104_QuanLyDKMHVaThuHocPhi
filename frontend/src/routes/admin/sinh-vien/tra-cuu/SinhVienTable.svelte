<script lang="ts">
    interface SinhVien {
        id: String;
        ho_ten: String;
        gioi_tinh: String;
        ngay_sinh: String;
        nganh: String;
    }
    let decoy: SinhVien[] = [
        {
            id: "22520278",
            ho_ten: "Huỳnh Anh Dũng",
            gioi_tinh: "Nam",
            ngay_sinh: "06-09-2004",
            nganh: "Khoa học Máy tính",
        },
    ];
</script>

<div class="table-container">
    <table class="table table-hover">
        <thead>
            <tr>
                <th> Mã số sinh viên </th>
                <th> Họ và tên sinh viên </th>
                <th> Giới tính </th>
                <th> Ngày sinh </th>
                <th> Ngành học </th>
                <th> Thao tác </th>
            </tr>
        </thead>

        <tbody>
            {#each decoy as sinh_vien}
            <tr>
                <td>{sinh_vien.id}</td>
                <td>{sinh_vien.ho_ten}</td>
                <td>{sinh_vien.gioi_tinh}</td>
                <td>{sinh_vien.ngay_sinh}</td>
                <td>{sinh_vien.nganh}</td>
                <td>
                    <span><a href="/temp">temp</a></span>
                    <span><a href="/temp">temp</a></span>
                </td>
            </tr>
            {/each}
        </tbody>
    </table>
</div>
