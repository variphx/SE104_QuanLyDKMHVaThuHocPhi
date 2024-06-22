export const get_nganh_options = async (id_khoa: string) =>  {
    const response = await fetch('http://localhost:8080/api/chuong-trinh-hoc/nganh/options/get', {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(id_khoa)
    });

    if (!response.ok) {
        throw new Error(await response.text());
    }

    return  response.json();
}