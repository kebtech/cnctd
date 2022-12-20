<template>
<div class="sign_up">
    <div class="not_signed_up" v-if="!submission.signedUp">
        <div class="top">
            <div class="description">{{dashboard.descriptions.signUp}}</div>
            <div :class="invitation.valid? 'dim': 'bright'">
                <input class="invitation_url"
                    ref="invitation_url" 
                    placeholder="paste invitation link here" 
                    @input="handlePaste" />
                <div class="status" v-if="invitation.url.length > 0">
                    <div v-if="invitation.valid" 
                        class="accepted">
                    </div>
                    <div v-if="!invitation.valid"
                        class="rejected">
                    </div>
                    <div class="status_message">{{invitation.valid? 'valid invite': invitation.error}}</div>
                </div>
            </div>
            
        </div>
        
        <form>
            <div :class="invitation.valid? 'valid_token': 'invalid_token'"
                class="fields">
                <div class="field">
                    <label for="username">Username</label>
                    <input class="username validate" 
                        ref="username"
                        type="text"
                        minlength="3"
                        maxlength="60"
                        required />
                    
                </div>  
                <div class="field">
                    <label for="display_name">Artist Name</label>
                    <input class="display_name validate" 
                        ref="display_name"
                        type="text"
                        minlength="3"
                        maxlength="60"
                        required />
                    
                </div>
                <div class="field">
                    <label for="email">Email</label>
                    <input class="email validate" 
                        ref="email"
                        type="email"
                        required
                        min="5"
                        maxlength="100"
                        :value="invitation.contact" />
                </div>
                <div class="field">
                    <label for="password">Password</label>
                    <input @keyup.enter="submit" class="password validate" 
                        ref="password"
                        type="password"
                        minlength="6"
                        maxlength="255"
                        required />
                </div>
                <div class="field">
                    <label for="verify_password">Verify Password</label>
                    <input @keyup.enter="submit" class="verify_password" 
                        ref="verify_password"
                        type="password"
                        @input="verifyPasswords"
                        :class="{ valid_input: submission.passwords_match }" />
                </div>
                <div class="field accept_check">
                    <input type="checkbox" name="accept" ref="accept" class="accept validate" required>
                    <label for="accept">I confirm that I am using cnctd as a beta tester and that nothing in life is guaranteed</label>
                </div>
                <button type="submit" @click.prevent="submit" class="submit">Sign Up</button>
                <div class="status" v-if="submission.error.length > 0">
                    <div class="rejected"></div>
                    <div class="submission_error">{{submission.error}}</div>
                </div>
                
            </div>
        </form>
    </div>
    <div class="post_sign_up" v-if="submission.signedUp">congrats dickweasel</div>
    
</div>
</template>

<script lang="ts">
import { defineComponent, inject, onMounted, reactive, Ref, ref } from 'vue';
import { dashboard } from '../dashboard';
import { server } from '@/server/server';

export default defineComponent({
    setup() {
        const store: any = inject('store');
        const username: Ref<HTMLInputElement | null> = ref(null);
        const display_name: Ref<HTMLInputElement | null> = ref(null);
        const email: Ref<HTMLInputElement | null> = ref(null);
        const password: Ref<HTMLInputElement | null> = ref(null);
        const verify_password: Ref<HTMLInputElement | null> = ref(null);
        const invitation_url: Ref<HTMLInputElement | null> = ref(null);
        const accept: Ref<HTMLInputElement | null> = ref(null); 

        const submission = reactive({
            failure: false,
            error: '',
            checked_terms: false,
            passwords_match: false,
            signedUp: false,
        });

        const invitation = reactive({
            contact: '',
            token: '',
            url: invitation_url.value? invitation_url.value.value: '',
            valid: false,
            error: 'invalid invite',
        });

        const verifyPasswords = () => {
            submission.passwords_match = password.value!.value === verify_password.value!.value;
        }


        const handlePaste = () => {
            invitation.url = invitation_url.value!.value;
            invitation.valid = false;
            invitation.error = 'invalid invite';
            // const el = e.target as HTMLInputElement;
            const url = invitation_url.value!.value;
            if (url.includes('?') && url.split('?').length > 0) {
                const inv = url.split('?')[1];
                const firstPart = inv.split('&')[0];
                const secondPart = inv.split('&')[1];
                const contact = firstPart.split('=')[1];
                const token = secondPart.split('=')[1];
                
                invitation.contact = contact;
                invitation.token = token;
                console.log(invitation);
                if (invitation.contact && invitation.token) {
                    server.post('user_account', 'verify_invite_token', { contact: invitation.contact, token: invitation.token })
                        .then(r => {
                            console.log(r)
                            if (r.success) invitation.valid = true
                            else invitation.error = r.msg === 'token already used'? 'invite already used': 'invalid invite'
                        })
                }
            }
            
        };
        
        const submit = () => {
            const inputElements = document.querySelectorAll('input');
            let count = 0;
            const requireValidation = [] as boolean[];
            inputElements.forEach(e => {
                if (e.required) {
                    console.log('required', e.name)
                    requireValidation.push(e.validity.valid)
                }
                count++;
                if (count === inputElements.length) { 
                    console.log('count it!')
                    console.log(requireValidation)
                    submission.checked_terms = accept.value!.checked;
                    submission.failure = (!submission.checked_terms || !submission.passwords_match) || requireValidation.includes(false);
                    if (!submission.failure) {
                        console.log('success!')
                        const newUser = {
                            username: username.value!.value,
                            display_name: display_name.value!.value,
                            email: email.value!.value,
                            password: password.value!.value,
                            contact: invitation.contact,
                            token: invitation.token
                        };
                        server.post('user_account', 'add', newUser).then(r => {
                            console.log('response', r);
                            if (!r.success) {
                                submission.failure = false;
                                submission.error = r.msg === 'token already used'? 'invite already used': r.msg;
                            } else {
                                submission.signedUp = true;
                            }
                        
                        })
                    } else {
                        submission.error = !submission.checked_terms? `you didn't accept terms`: '';
                        
                    }
                }
            })
            
            
        };

        onMounted(() => {
            const path = location.pathname.split('/')[1];
            console.log('path', path)
            if (path === 'invitation') {
                if (location.href.includes('?')) invitation_url.value!.value = location.href;
                dashboard.currentView = 'Sign Up';
                handlePaste()
            } else if (path === 'download') {
                dashboard.currentView = 'Download';
            }
        })

        return {
            dashboard,
            submit,
            invitation,
            handlePaste,
            invitation_url,
            username,
            display_name,
            email,
            password,
            verify_password,
            store,
            submission,
            accept,
            verifyPasswords
        }
    },

})
</script>

<style scoped>
.sign_up {
    color: white;
}
.description {
    margin-left: 10%;
    margin-right: 10%;
    text-align: left;
    margin-top: 40px;
}

.invitation_url {
    width: 75%;
    margin-top: 20px;
}
.fields {
    margin-top: 20px;
    display: flex;
    width: fit-content;
    flex-wrap: wrap;
    flex-direction: column;
    justify-content: space-evenly;
    margin-left: auto;
    margin-right: auto;
}
.field {
    width: 300px;
    display: flex;
    flex-direction: column;
}
label {
    text-align: center;
    margin-top: 10px;
}
input {
    margin-top: 5px;
}
.validate:valid {
    outline: 2px solid v-bind("store.colors.file + 99");
}
.valid_input {
    outline: 2px solid v-bind("store.colors.file + 99");
}
.invalid_input {
    outline: 2px solid #ff000099;
}
.accepted {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: v-bind("store.colors.file + 99");
    margin-right: 10px;
    margin-bottom: 5px;
    margin-top: auto;
}
.rejected {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: red;
    margin-right: 10px;
    margin-bottom: 5px;
    margin-top: auto;
}
.status {
    margin-right: 12.5%;
    text-align: right;
    display: flex;
    width: fit-content;
    margin-left: auto;
}
.valid_token {
    opacity: 1;
}
.invalid_token {
    opacity: .2;
    pointer-events: none;
}
.submit {
    background: v-bind('store.colors.collection');
    color: white;
    border-radius: 5px;
    width: 70%;
    padding: 5px;
    margin-top: 20px;
    margin-right: auto;
    margin-left: auto;
    border: none;
    font-size: 1.1rem;
    cursor: pointer;
}
.submission_error {
    /* width: 300px; */
    /* text-align: left; */
    margin-top: 5px;
}
.accept_check {
    display: flex;
    flex-direction: row;
    text-align: left;
}
.email {
    pointer-events: none;
    cursor: not-allowed;
}
.accept_check label {
    margin-left: 5px;
    text-align: left;
}
.accept:valid {
    outline: none
}
.bright {
    opacity: 1;
}
.dim {
    opacity: .2;
}
.accept_check input[type="checkbox"]:checked ~ .b-input {
  background: v-bind("store.colors.file + 99");;
  /* border-color: #1d4ed8; */
}

</style>