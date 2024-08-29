import { createApp } from "vue";
import registerPlugins from "./plugins";
import router from "./router";

import App from "./App.vue";

const app = createApp(App);

registerPlugins(app);

app.mount("#app");

app.config.globalProperties.$router = router;
app.config.globalProperties.$route = router.currentRoute.value;

declare module '@vue/runtime-core' {
    export interface ComponentCustomProperties {
        $router: import('vue-router').Router;
        $route: import('vue-router').RouteLocationNormalized;
    }
  }
