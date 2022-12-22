import { reactive } from "vue";

const descriptions = {
    signUp: `Thanks for your interest in cnctd. We are currently in the super early beta stage, which is invite-only. If you have an invite link, please enter it below to continue signing up.`,
    home: `Welcome to Connected Dot`
}


const dashboard = reactive({
    currentView: 'Home',
    descriptions
});

const path = location.pathname.split('/')[1];
if (path === 'download') {
    dashboard.currentView = 'Download';
}


export {
    dashboard
}