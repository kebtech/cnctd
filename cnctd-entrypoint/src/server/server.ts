import { v4 as uuidv4} from 'uuid';

const server = {
    clientId: uuidv4(),
    post: (channel: string, instruction?: string, data?: any): Promise<any> => {
        return new Promise((ok, err) => {
        // const response_channel: string = uuidv4();


        const msg = JSON.stringify({ channel, instruction, data: data });

        // const url = `${location.protocol}//${location.hostname + ':5050'}/rest`
        const url = `https://cnctd.world/rest`
        console.log('post url', url);
        console.log('post msg', msg);
        fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            mode: 'cors',
            body: msg
        })
            .then(response => {
                console.log
                return Promise.all([response.json(),response])
            })
            .then(([responseData,response]) => {
                ok(responseData)
            })
            .catch(e => err(e))
        })
    },
    get: (channel: string, instruction?: string): Promise<any> => {
        return new Promise((ok, err) => {
        const response_channel: string = uuidv4();


        // const msg = JSON.stringify();
        const params: any = { channel, instruction };
        const query = Object.keys(params)
             .map(k => encodeURIComponent(k) + '=' + encodeURIComponent(params[k]))
             .join('&');
        // const url = `${location.protocol}//${location.hostname + ':5050'}/rest`
        const url = `https://cnctd.world/rest?${query}`
        console.log('post url', url);
        console.log('post msg', query);
        fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
            mode: 'cors',
        })
            .then(response => {
                console.log
                return Promise.all([response.json(),response])
            })
            .then(([responseData,response]) => {
                ok(responseData)
            })
            .catch(e => err(e))
        })
    }
}

export { server }