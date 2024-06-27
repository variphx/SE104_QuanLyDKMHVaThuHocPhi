export type MonHoc = {
    id: string;
    ten: string;
    id_khoa: string;
    loai: string;
    so_tiet: string;
    so_tin_chi: string;
}

export const get_all_mon_hoc = async() =>  {
    const response = await fetch("http://localhost:8080/api/mon-hoc/all/get");

    if (!response.ok) {
        throw new  Error(await response.text());
    }

    
};