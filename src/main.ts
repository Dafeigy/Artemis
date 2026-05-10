import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createMemoryHistory } from "vue-router";
import UserSettings from "./pages/UserSettings.vue";
import Main from "./pages/Main.vue";

const routes = [
    {
        path: '/',
        component: Main,
        name: '/'
    },
    {
        path: '/Settings',
        component: UserSettings,
        name: "Settings",
    },
]

const router = createRouter(
    {
        history: createMemoryHistory(),
        routes,
    }
)

createApp(App).use(router).mount("#app");
