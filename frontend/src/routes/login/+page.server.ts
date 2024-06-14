import { error, redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "../$types";
import { goto } from "$app/navigation";

export const load: PageServerLoad = async ({ cookies }) => {
    if (cookies.get("sessionid") && cookies.get("user")) {
        if (cookies.get("user") == "admin") {
            redirect(302, '/admin')
        } else {
            redirect(302, `/user/${cookies.get("user")}/dang-ky`);
        }
    }
}

export const actions = {
    login: async ({ cookies, request }) => {

        const data = await request.formData();
        const username = data.get('username')?.toString();
        const password = data.get('password')?.toString();

        if (!username || !password) {
            return { is_success: false };
        }


        const temp = await fetch('http://localhost:8080/api/user/get', {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                username: username,
                password: password
            })
        });

        const response = await temp.json();
        console.log(response);


        if (response.auth_token) {
            cookies.set('sessionid', response.auth_token, { path: '/', expires: new Date(Date.now() + 3600000), httpOnly: true, });
        } else {
            return { is_success: false };
        }

        if (username == "admin") {
            cookies.set('user', 'admin', { path: '/', expires: new Date(Date.now() + 3600000), httpOnly: true });
            redirect(302, '/admin');
        } else {
            cookies.set('user', username, { path: '/', expires: new Date(Date.now() + 3600000), httpOnly: true })
            redirect(302, `/user/${username}/dang-ky`);
        }
    }
}