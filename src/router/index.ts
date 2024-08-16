import { createMemoryHistory, createRouter } from "vue-router";
import IndexPage from "../pages/IndexPage.vue";

const routes = [
    {
        path: "/",
        component: IndexPage
    }
];

export default createRouter({
    history: createMemoryHistory(),
    routes
});
