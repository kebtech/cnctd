import { v4 as uuidv4} from 'uuid';

const server = {
    clientId: uuidv4(),
    post: (channel: string, instruction?: string, data?: any): Promise<any> => {
        return new Promise((ok, err) => {
        const response_channel: string = uuidv4();


        const msg = JSON.stringify({ channel, instruction, data, response_channel, id: server.clientId, user_id: 0, jwt: ''});

        // const url = `${location.protocol}//${location.hostname + ':5050'}/rest`
        const url = `https://app.cnctd.world/rest`
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
    }
}

export { server }