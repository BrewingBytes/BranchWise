import registerPlugins from "@/plugins";
import router from "@/router";
import { createApp } from "vue";

import App from "@/App.vue";

const app = createApp(App);

registerPlugins(app);

app.mount("#app");

app.config.globalProperties.$router = router;

declare module "@vue/runtime-core" {
    export interface ComponentCustomProperties {
        $router: import("vue-router").Router;
    }
  }
