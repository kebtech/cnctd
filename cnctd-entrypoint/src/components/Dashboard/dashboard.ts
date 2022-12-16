import { reactive } from "vue";

const descriptions = {
    signUp: `Thanks for your interest in cnctd. We are currently in the super early beta stage, which is invite-only. If you have an invite link, please enter it below to continue signing up.`
}


const dashboard = reactive({
    currentView: 'Home',
    descriptions
});



export {
    dashboard
}