import { ISession } from '../interface/session.interface';
export async function auth(session: ISession, base_url: string) {
    if (Object.keys(session).length === 0) return false
    try {
        const res = await fetch(`${base_url}/auth/me`, {
            headers:{
                "Authorization":`Bearer ${session.token}`
            }
        }).then(res => res.json())
        if(res.status_code !== 200) return false;
        return true;
    } catch (error) {
        console.error(error)
        return false;
    }
}